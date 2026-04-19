//! WorldModel → TypeQL insert statement generation.
//!
//! Pure-function translation from BERT's data model to the TypeDB schema
//! designed in `docs/bert-typedb-schema.md`. No network, no async.
//!
//! # Two-phase emission
//!
//! 1. **Entities** (this module, so far): every atomic thing that exists —
//!    `bert_model`, `system`, `external_entity`, `boundary`, `interface`,
//!    `interaction`. One `insert` per instance.
//!
//! 2. **Relations** (upcoming commit): connections between entities —
//!    `composition`, `has_boundary`, `has_interface`, `participates_in`, etc.
//!    Uses TypeDB's `match ... insert` pattern so both endpoints must already
//!    exist. That's why entities are phase 1 and relations are phase 2.
//!
//! # Namespacing
//!
//! Every `bert_id` is prefixed with `{model_name}:{local_id}` per the
//! schema design's multi-model story. `model_name` comes from the caller
//! (typically the JSON file stem like `"bitcoin"`).
//!
//! # Known deferrals (not in Phase 1)
//!
//! - `parameters: Vec<Parameter>` on interactions — no schema entity yet
//! - `smart_parameters: Vec<SmartParameter>` — no schema entity yet
//! - `hidden_entities` — UI state, not meaningful in a typed graph
//! - `transform: Option<Transform2d>` — visual position, not in schema
//! - `parent_interface: Option<Id>` on boundaries — interface processor
//!   linkage not represented in the current schema
//! - `exports_to` / `receives_from` on interfaces — implicit in the
//!   `routes_through` relations emitted in phase 2
//! - `ExternalEntityType` (Source/Sink) — role derived from flow direction
//!   per SL spec §1.2; not an attribute on `external_entity`

use crate::error::TranspilerResult;
use crate::escape::escape_typeql_string;
// The serializable data-model types live under `data_model`. The enum
// definitions they reference (SubstanceType, InteractionUsability, HcgsArchetype)
// live under `components::system_elements` — `data_model/mod.rs` does
// `use crate::bevy_app::components::*;` but doesn't re-export, so we import
// them from their real home.
use bert::bevy_app::components::{
    HcgsArchetype, InteractionType, InteractionUsability, SubstanceType,
};
use bert::bevy_app::data_model::{
    Boundary, Complexity, ExternalEntity, Id, Interaction, Interface, InterfaceType, System,
    WorldModel,
};

/// Produce every TypeQL `insert` statement needed to materialize `model` in
/// TypeDB. Statements are ordered: all entities first, then (in later
/// commits) relations. A caller can issue them in one Write transaction or
/// split across transactions — the output is a flat list either way.
///
/// `model_name` becomes the prefix on every `bert_id` value to enable
/// multi-model coexistence in one database.
pub fn model_to_typeql(model: &WorldModel, model_name: &str) -> TranspilerResult<Vec<String>> {
    let mut out = Vec::new();

    // Phase 1: entities
    out.push(emit_bert_model(model_name, &model.environment.info.description));
    for ee in model
        .environment
        .sources
        .iter()
        .chain(model.environment.sinks.iter())
    {
        out.push(emit_external_entity(ee, model_name));
    }
    for system in &model.systems {
        out.push(emit_system(system, model_name));
        out.push(emit_boundary(&system.boundary, model_name));
        for iface in &system.boundary.interfaces {
            out.push(emit_interface(iface, model_name));
        }
    }
    for ix in &model.interactions {
        out.push(emit_interaction(ix, model_name));
    }

    // Phase 2: relations — emitted in the next commit.

    Ok(out)
}

// ---------------------------------------------------------------------------
// Entity emitters
// ---------------------------------------------------------------------------

fn emit_bert_model(model_name: &str, description: &str) -> String {
    format!(
        r#"insert $m isa bert_model, has model_name "{name}", has description "{desc}";"#,
        name = escape_typeql_string(model_name),
        desc = escape_typeql_string(description),
    )
}

fn emit_external_entity(ee: &ExternalEntity, model_name: &str) -> String {
    format!(
        r#"insert $ee isa external_entity, has bert_id "{id}", has display_name "{name}", has description "{desc}", has equivalence_class "{equiv}";"#,
        id = namespaced_id(model_name, &ee.info.id),
        name = escape_typeql_string(&ee.info.name),
        desc = escape_typeql_string(&ee.info.description),
        equiv = escape_typeql_string(&ee.equivalence),
    )
}

fn emit_system(system: &System, model_name: &str) -> String {
    let mut attrs = vec![
        format!(r#"has bert_id "{}""#, namespaced_id(model_name, &system.info.id)),
        format!(
            r#"has display_name "{}""#,
            escape_typeql_string(&system.info.name)
        ),
        format!(
            r#"has description "{}""#,
            escape_typeql_string(&system.info.description)
        ),
        format!("has system_level {}", system.info.level),
        format!("has radius {}", system.radius),
        format!(
            r#"has equivalence_class "{}""#,
            escape_typeql_string(&system.equivalence)
        ),
        format!(
            r#"has history_note "{}""#,
            escape_typeql_string(&system.history)
        ),
        format!(
            r#"has transformation_note "{}""#,
            escape_typeql_string(&system.transformation)
        ),
        format!("has member_autonomy {}", system.member_autonomy),
        format!(
            r#"has time_constant "{}""#,
            escape_typeql_string(&system.time_constant)
        ),
    ];

    // Archetype: emit only when Some and not Unspecified. The @values list
    // in the schema includes "Unspecified" for forward-compat, but emitting
    // it in practice would be noise.
    if let Some(arch) = system.archetype {
        if arch != HcgsArchetype::Unspecified {
            attrs.push(format!(r#"has archetype "{}""#, archetype_str(arch)));
        }
    }

    // Complexity: emit conditionally based on variant.
    match system.complexity {
        Complexity::Atomic => {
            attrs.push(r#"has complexity_kind "Atomic""#.to_string());
        }
        Complexity::Complex {
            adaptable,
            evolveable,
        } => {
            attrs.push(r#"has complexity_kind "Complex""#.to_string());
            attrs.push(format!("has complex_adaptable {}", adaptable));
            attrs.push(format!("has complex_evolveable {}", evolveable));
        }
        Complexity::Multiset(n) => {
            attrs.push(r#"has complexity_kind "Multiset""#.to_string());
            attrs.push(format!("has multiset_count {}", n));
        }
    }

    format!("insert $s isa system, {};", attrs.join(", "))
}

fn emit_boundary(boundary: &Boundary, model_name: &str) -> String {
    format!(
        r#"insert $b isa boundary, has bert_id "{id}", has display_name "{name}", has description "{desc}", has porosity {porosity}, has perceptive_fuzziness {fuzz};"#,
        id = namespaced_id(model_name, &boundary.info.id),
        name = escape_typeql_string(&boundary.info.name),
        desc = escape_typeql_string(&boundary.info.description),
        porosity = boundary.porosity,
        fuzz = boundary.perceptive_fuzziness,
    )
}

fn emit_interface(iface: &Interface, model_name: &str) -> String {
    let mut attrs = vec![
        format!(
            r#"has bert_id "{}""#,
            namespaced_id(model_name, &iface.info.id)
        ),
        format!(
            r#"has display_name "{}""#,
            escape_typeql_string(&iface.info.name)
        ),
        format!(
            r#"has description "{}""#,
            escape_typeql_string(&iface.info.description)
        ),
        format!(
            r#"has protocol "{}""#,
            escape_typeql_string(&iface.protocol)
        ),
        format!(
            r#"has interface_type "{}""#,
            interface_type_str(iface.ty)
        ),
    ];
    if let Some(angle) = iface.angle {
        attrs.push(format!("has interface_angle {}", angle));
    }
    format!("insert $i isa interface, {};", attrs.join(", "))
}

fn emit_interaction(ix: &Interaction, model_name: &str) -> String {
    format!(
        r#"insert $f isa interaction, has bert_id "{id}", has display_name "{name}", has description "{desc}", has substance_type "{stype}", has substance_sub_type "{ssub}", has interaction_type "{ity}", has usability "{use_}", has amount "{amt}", has unit "{unit}";"#,
        id = namespaced_id(model_name, &ix.info.id),
        name = escape_typeql_string(&ix.info.name),
        desc = escape_typeql_string(&ix.info.description),
        stype = substance_type_str(ix.substance.ty),
        ssub = escape_typeql_string(&ix.substance.sub_type),
        ity = interaction_type_str(ix.ty),
        use_ = usability_str(ix.usability),
        // Decimal → string. Matches the serde representation.
        amt = escape_typeql_string(&ix.amount.to_string()),
        unit = escape_typeql_string(&ix.unit),
    )
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build the namespaced `bert_id` attribute value: `{model_name}:{local_id}`.
pub(crate) fn namespaced_id(model_name: &str, id: &Id) -> String {
    let escaped_prefix = escape_typeql_string(model_name);
    let local = serialize_id(id);
    format!("{}:{}", escaped_prefix, escape_typeql_string(&local))
}

/// Serialize an Id to its wire representation (e.g. `"S0"`, `"C0.1.2"`)
/// by round-tripping through serde_json and stripping the surrounding quotes.
/// Small surface — Id's Serialize impl is the authoritative source of the
/// string format, so this avoids duplicating the format logic here.
pub(crate) fn serialize_id(id: &Id) -> String {
    serde_json::to_string(id)
        .ok()
        .and_then(|s| {
            s.strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .map(String::from)
        })
        .unwrap_or_default()
}

fn archetype_str(a: HcgsArchetype) -> &'static str {
    match a {
        HcgsArchetype::Unspecified => "Unspecified",
        HcgsArchetype::Governance => "Governance",
        HcgsArchetype::Economy => "Economy",
        HcgsArchetype::Agent => "Agent",
    }
}

fn substance_type_str(s: SubstanceType) -> &'static str {
    match s {
        SubstanceType::Energy => "Energy",
        SubstanceType::Material => "Material",
        SubstanceType::Message => "Message",
    }
}

fn interaction_type_str(t: InteractionType) -> &'static str {
    match t {
        InteractionType::Flow => "Flow",
        InteractionType::Force => "Force",
    }
}

fn usability_str(u: InteractionUsability) -> &'static str {
    match u {
        InteractionUsability::Resource => "Resource",
        InteractionUsability::Disruption => "Disruption",
        InteractionUsability::Product => "Product",
        InteractionUsability::Waste => "Waste",
    }
}

fn interface_type_str(t: InterfaceType) -> &'static str {
    match t {
        InterfaceType::Import => "Import",
        InterfaceType::Export => "Export",
        InterfaceType::Hybrid => "Hybrid",
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn load_bitcoin() -> WorldModel {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/models/examples/bitcoin.json"
        );
        let bytes = std::fs::read(path).expect("bitcoin.json should exist");
        serde_json::from_slice(&bytes).expect("bitcoin.json should deserialize")
    }

    #[test]
    fn bitcoin_produces_expected_entity_counts() {
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();

        // Exactly one `isa bert_model` line for this model.
        let bert_models = stmts.iter().filter(|s| s.contains("isa bert_model")).count();
        assert_eq!(bert_models, 1, "expected one bert_model insert");

        // One external_entity per source + sink in the environment.
        let ee_count = stmts
            .iter()
            .filter(|s| s.contains("isa external_entity"))
            .count();
        let expected_ee =
            model.environment.sources.len() + model.environment.sinks.len();
        assert_eq!(ee_count, expected_ee, "external_entity count mismatch");

        // One system per system in the model.
        let sys_count = stmts.iter().filter(|s| s.contains("isa system")).count();
        assert_eq!(sys_count, model.systems.len(), "system count mismatch");

        // One boundary per system (every system has a boundary).
        let b_count = stmts.iter().filter(|s| s.contains("isa boundary")).count();
        assert_eq!(b_count, model.systems.len(), "boundary count mismatch");

        // Interfaces: sum across all system boundaries.
        let i_count = stmts.iter().filter(|s| s.contains("isa interface")).count();
        let expected_i: usize = model
            .systems
            .iter()
            .map(|s| s.boundary.interfaces.len())
            .sum();
        assert_eq!(i_count, expected_i, "interface count mismatch");

        // One interaction per interaction.
        let f_count = stmts
            .iter()
            .filter(|s| s.contains("isa interaction"))
            .count();
        assert_eq!(
            f_count,
            model.interactions.len(),
            "interaction count mismatch"
        );
    }

    #[test]
    fn every_statement_is_well_formed() {
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();
        for stmt in &stmts {
            assert!(
                stmt.starts_with("insert ") || stmt.starts_with("match "),
                "statement does not start with insert/match: {stmt}"
            );
            assert!(
                stmt.ends_with(';'),
                "statement does not end with semicolon: {stmt}"
            );
            // Count unescaped quotes only. An escaped quote is any `"` where
            // the preceding character is a backslash AND that backslash is
            // not itself escaped (preceded by another backslash). Each string
            // literal contributes exactly 2 unescaped quotes — so the count
            // across a well-formed statement must be even.
            let unescaped_quotes = count_unescaped_quotes(stmt);
            assert!(
                unescaped_quotes % 2 == 0,
                "unbalanced unescaped quotes ({unescaped_quotes}) in: {stmt}"
            );
        }
    }

    /// Counts `"` characters that are NOT escaped by a preceding backslash.
    /// Walks the backslash run preceding each quote — an odd-length run means
    /// the quote is escaped, even-length means the backslashes are themselves
    /// escaped pairs and the quote stands alone.
    fn count_unescaped_quotes(s: &str) -> usize {
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        for (i, &c) in chars.iter().enumerate() {
            if c != '"' {
                continue;
            }
            let mut backslashes = 0;
            let mut j = i;
            while j > 0 && chars[j - 1] == '\\' {
                backslashes += 1;
                j -= 1;
            }
            if backslashes % 2 == 0 {
                count += 1;
            }
        }
        count
    }

    #[test]
    fn namespace_prefix_applied_to_every_bert_id() {
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();
        for stmt in &stmts {
            if let Some(pos) = stmt.find(r#"has bert_id ""#) {
                let rest = &stmt[pos + r#"has bert_id ""#.len()..];
                let end = rest.find('"').unwrap_or(rest.len());
                let id_value = &rest[..end];
                assert!(
                    id_value.starts_with("bitcoin:"),
                    "bert_id lacks namespace prefix: '{id_value}' in {stmt}"
                );
            }
        }
    }

    #[test]
    fn enum_attributes_emit_only_controlled_vocabulary() {
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();

        let extract = |s: &str, key: &str| -> Option<String> {
            let needle = format!(r#"has {key} ""#);
            s.find(&needle).map(|i| {
                let rest = &s[i + needle.len()..];
                let end = rest.find('"').unwrap_or(0);
                rest[..end].to_string()
            })
        };

        for stmt in &stmts {
            if let Some(v) = extract(stmt, "archetype") {
                assert!(
                    ["Governance", "Economy", "Agent"].contains(&v.as_str()),
                    "archetype out of vocab: {v}"
                );
            }
            if let Some(v) = extract(stmt, "substance_type") {
                assert!(
                    ["Energy", "Material", "Message"].contains(&v.as_str()),
                    "substance_type out of vocab: {v}"
                );
            }
            if let Some(v) = extract(stmt, "interaction_type") {
                assert!(
                    ["Flow", "Force"].contains(&v.as_str()),
                    "interaction_type out of vocab: {v}"
                );
            }
            if let Some(v) = extract(stmt, "usability") {
                assert!(
                    ["Resource", "Disruption", "Product", "Waste"]
                        .contains(&v.as_str()),
                    "usability out of vocab: {v}"
                );
            }
            if let Some(v) = extract(stmt, "interface_type") {
                assert!(
                    ["Import", "Export", "Hybrid"].contains(&v.as_str()),
                    "interface_type out of vocab: {v}"
                );
            }
            if let Some(v) = extract(stmt, "time_constant") {
                assert!(
                    [
                        "Millisecond",
                        "Second",
                        "Minute",
                        "Hour",
                        "Day",
                        "Week",
                        "Month",
                        "Year",
                        "Decade",
                        "Century",
                        "Epoch",
                    ]
                    .contains(&v.as_str()),
                    "time_constant out of vocab: {v}"
                );
            }
        }
    }

    #[test]
    fn f0_3_carries_force_interaction_type_through() {
        // Guards the spec-compliance work from #14: the F0.3 reclassification
        // as Force must survive transpilation.
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();

        let f03_insert = stmts
            .iter()
            .find(|s| s.contains(r#"has bert_id "bitcoin:F0.3""#))
            .expect("F0.3 interaction should be emitted");
        assert!(
            f03_insert.contains(r#"has interaction_type "Force""#),
            "F0.3 lost its Force type during transpilation: {f03_insert}"
        );
    }
}
