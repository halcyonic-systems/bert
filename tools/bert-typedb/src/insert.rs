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
    AgentKind, AgentModel, Boundary, Complexity, ExternalEntity, Id, Interaction, Interface,
    InterfaceType, ProcessPrimitive, System, WorldModel,
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
    out.push(emit_bert_model(
        model_name,
        &model.environment.info.description,
    ));
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

    // Phase 2: relations (match-then-insert)
    for system in &model.systems {
        out.push(emit_has_boundary(system, model_name));
        for iface in &system.boundary.interfaces {
            out.push(emit_has_interface(&system.boundary, iface, model_name));
        }
        // Parent relationship: root system plays in_environment with the bert_model;
        // subsystems play composition with their parent system.
        if is_environment_id(&system.parent) {
            out.push(emit_in_environment_for_system(system, model_name));
        } else {
            out.push(emit_composition(system, model_name));
        }
        // Agent bundle: only when archetype == Agent AND agent field populated.
        if let Some(agent) = &system.agent {
            out.push(emit_agent_bundle(agent, system, model_name));
        }
    }
    for ee in model
        .environment
        .sources
        .iter()
        .chain(model.environment.sinks.iter())
    {
        out.push(emit_in_environment_for_external(ee, model_name));
    }
    for ix in &model.interactions {
        out.push(emit_participates_in(ix, model_name, "source", &ix.source));
        out.push(emit_participates_in(ix, model_name, "sink", &ix.sink));
        if let Some(iface_id) = &ix.source_interface {
            out.push(emit_routes_through(ix, iface_id, model_name, "start"));
        }
        if let Some(iface_id) = &ix.sink_interface {
            out.push(emit_routes_through(ix, iface_id, model_name, "end"));
        }
    }
    out.extend(emit_equivalence_pairs(&model.environment, model_name));

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
        format!(
            r#"has bert_id "{}""#,
            namespaced_id(model_name, &system.info.id)
        ),
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
        format!(r#"has interface_type "{}""#, interface_type_str(iface.ty)),
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
// Relation emitters
// ---------------------------------------------------------------------------

fn emit_has_boundary(system: &System, model_name: &str) -> String {
    format!(
        r#"match $s isa system, has bert_id "{sys}"; $b isa boundary, has bert_id "{bnd}"; insert (enclosed: $s, enclosure: $b) isa has_boundary;"#,
        sys = namespaced_id(model_name, &system.info.id),
        bnd = namespaced_id(model_name, &system.boundary.info.id),
    )
}

fn emit_has_interface(boundary: &Boundary, iface: &Interface, model_name: &str) -> String {
    format!(
        r#"match $b isa boundary, has bert_id "{bnd}"; $i isa interface, has bert_id "{ifc}"; insert (boundary: $b, interface: $i) isa has_interface;"#,
        bnd = namespaced_id(model_name, &boundary.info.id),
        ifc = namespaced_id(model_name, &iface.info.id),
    )
}

fn emit_composition(child_system: &System, model_name: &str) -> String {
    format!(
        r#"match $parent isa system, has bert_id "{parent}"; $child isa system, has bert_id "{child}"; insert (whole: $parent, part: $child) isa composition;"#,
        parent = namespaced_id(model_name, &child_system.parent),
        child = namespaced_id(model_name, &child_system.info.id),
    )
}

fn emit_in_environment_for_system(system: &System, model_name: &str) -> String {
    format!(
        r#"match $m isa bert_model, has model_name "{mname}"; $s isa system, has bert_id "{sys}"; insert (environment: $m, contained_system: $s) isa in_environment;"#,
        mname = escape_typeql_string(model_name),
        sys = namespaced_id(model_name, &system.info.id),
    )
}

fn emit_in_environment_for_external(ee: &ExternalEntity, model_name: &str) -> String {
    format!(
        r#"match $m isa bert_model, has model_name "{mname}"; $e isa external_entity, has bert_id "{ee}"; insert (environment: $m, contained_system: $e) isa in_environment;"#,
        mname = escape_typeql_string(model_name),
        ee = namespaced_id(model_name, &ee.info.id),
    )
}

/// Emit one `participates_in(entity, interaction, role)` relation.
///
/// The `entity` endpoint can be either a `system` or an `external_entity` —
/// TypeDB's match can resolve across both because `bert_id` is a @key
/// shared by any entity that plays `participates_in:entity`. We use
/// `isa $_` style by declaring the concrete type inferred from the Id's
/// type prefix rather than relying on meta-type `entity` matching.
fn emit_participates_in(
    ix: &Interaction,
    model_name: &str,
    role: &str,
    endpoint_id: &Id,
) -> String {
    let entity_type = concrete_type_for_id(endpoint_id);
    format!(
        // NOTE: reserved-word collisions in TypeQL 3.x surfaced when loading
        // the schema against a live server:
        //   - attribute `role` → `participation_role` (`role` is reserved)
        //   - role `entity` → `participant` (`entity` is the built-in
        //     meta-type for all entity types)
        r#"match $e isa {etype}, has bert_id "{eid}"; $f isa interaction, has bert_id "{fid}"; insert (participant: $e, interaction: $f) isa participates_in, has participation_role "{role}";"#,
        etype = entity_type,
        eid = namespaced_id(model_name, endpoint_id),
        fid = namespaced_id(model_name, &ix.info.id),
        role = role,
    )
}

fn emit_routes_through(
    ix: &Interaction,
    iface_id: &Id,
    model_name: &str,
    endpoint: &str,
) -> String {
    format!(
        r#"match $f isa interaction, has bert_id "{fid}"; $i isa interface, has bert_id "{iid}"; insert (interaction: $f, interface: $i) isa routes_through, has endpoint "{ep}";"#,
        fid = namespaced_id(model_name, &ix.info.id),
        iid = namespaced_id(model_name, iface_id),
        ep = endpoint,
    )
}

/// Emit one combined match+insert statement per agent-archetype system that
/// materializes the `agent_model` entity AND all its primitive + cognitive
/// param entities AND all the linking relations in a single query.
///
/// Why bundled: `agent_model`, `primitive_assignment`, and `cognitive_parameter`
/// are schema entities without a `@key` — they can't be matched later by a
/// stable ID. Creating them in the same statement keeps variable bindings
/// (`$a`, `$p1`, etc.) alive so the relations wire up correctly.
fn emit_agent_bundle(agent: &AgentModel, system: &System, model_name: &str) -> String {
    let sys_id = namespaced_id(model_name, &system.info.id);
    let mut inserts: Vec<String> = Vec::new();

    inserts.push(format!(
        r#"$a isa agent_model, has agent_kind "{kind}", has agency_capacity {cap}"#,
        kind = agent_kind_str(agent.kind),
        cap = agent.agency_capacity,
    ));
    inserts.push("(system: $s, config: $a) isa has_agent_config".to_string());

    for (i, prim) in agent.primitives.iter().enumerate() {
        inserts.push(format!(
            r#"$p{idx} isa primitive_assignment, has process_primitive "{val}""#,
            idx = i,
            val = process_primitive_str(*prim),
        ));
        inserts.push(format!(
            "(agent: $a, primitive: $p{idx}) isa has_primitive",
            idx = i
        ));
    }

    for (i, (k, v)) in agent.cognitive_params.iter().enumerate() {
        inserts.push(format!(
            r#"$cp{idx} isa cognitive_parameter, has cognitive_param_name "{key}", has cognitive_param_value {val}"#,
            idx = i,
            key = escape_typeql_string(k),
            val = v,
        ));
        inserts.push(format!(
            "(agent: $a, param: $cp{idx}) isa has_cognitive_param",
            idx = i
        ));
    }

    format!(
        r#"match $s isa system, has bert_id "{sys}"; insert {body};"#,
        sys = sys_id,
        body = inserts.join("; "),
    )
}

/// Produce one `is_equivalent_to` per pair of external entities that share
/// an `is_same_as_id` value. Design decision: pairwise (complete graph).
/// Rationale: within a single BERT model, groups are almost always pairs
/// (one source-role + one sink-role representing the same real-world
/// entity, linked by is_same_as_id). For the common N=2 case, pairwise =
/// chain in output. If cross-model linking scales N, revisit with a
/// TypeDB inference rule for transitive closure.
///
/// Canonical ordering: within a group, relations are emitted for all
/// index pairs (i, j) where i < j (so deterministic across runs). The
/// earlier-listed entity plays the `primary` role, the later plays
/// `equivalent`. This assignment is arbitrary for symmetric equivalence
/// — downstream queries should match both `(primary, equivalent)` and
/// `(equivalent, primary)` orderings, or define a symmetric-closure
/// inference rule when needed.
fn emit_equivalence_pairs(
    env: &bert::bevy_app::data_model::Environment,
    model_name: &str,
) -> Vec<String> {
    use std::collections::HashMap;
    let mut groups: HashMap<usize, Vec<&ExternalEntity>> = HashMap::new();
    for ee in env.sources.iter().chain(env.sinks.iter()) {
        if let Some(key) = ee.is_same_as_id {
            groups.entry(key).or_default().push(ee);
        }
    }

    let mut out = Vec::new();
    for members in groups.values() {
        for i in 0..members.len() {
            for j in (i + 1)..members.len() {
                out.push(format!(
                    r#"match $a isa external_entity, has bert_id "{a}"; $b isa external_entity, has bert_id "{b}"; insert (primary: $a, equivalent: $b) isa is_equivalent_to;"#,
                    a = namespaced_id(model_name, &members[i].info.id),
                    b = namespaced_id(model_name, &members[j].info.id),
                ));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The environment sentinel ID `E-1`. Used as the `parent` of the root
/// system — distinguishing it from subsystems whose parent is another system.
fn is_environment_id(id: &Id) -> bool {
    serialize_id(id) == "E-1"
}

/// Determine the TypeDB entity type name from an Id's type prefix.
/// Needed for `participates_in` match queries where the entity endpoint
/// may be either a `system` or an `external_entity`.
fn concrete_type_for_id(id: &Id) -> &'static str {
    let s = serialize_id(id);
    if s.starts_with("Src") || s.starts_with("Snk") {
        "external_entity"
    } else {
        "system"
    }
}

fn agent_kind_str(k: AgentKind) -> &'static str {
    match k {
        AgentKind::Reactive => "Reactive",
        AgentKind::Anticipatory => "Anticipatory",
        AgentKind::Intentional => "Intentional",
    }
}

fn process_primitive_str(p: ProcessPrimitive) -> &'static str {
    match p {
        ProcessPrimitive::Combining => "Combining",
        ProcessPrimitive::Splitting => "Splitting",
        ProcessPrimitive::Buffering => "Buffering",
        ProcessPrimitive::Impeding => "Impeding",
        ProcessPrimitive::Propelling => "Propelling",
        ProcessPrimitive::Copying => "Copying",
        ProcessPrimitive::Sensing => "Sensing",
        ProcessPrimitive::Modulating => "Modulating",
        ProcessPrimitive::Inverting => "Inverting",
    }
}

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

        // Entity inserts start with `insert `; relation statements start
        // with `match ` and also contain `isa X` substrings in their match
        // clauses. Filter to insert-only to count entity creations.
        let is_entity_insert = |s: &&String, type_name: &str| -> bool {
            s.starts_with("insert ") && s.contains(&format!("isa {type_name},"))
        };

        assert_eq!(
            stmts
                .iter()
                .filter(|s| is_entity_insert(s, "bert_model"))
                .count(),
            1,
            "expected one bert_model insert"
        );

        let ee_count = stmts
            .iter()
            .filter(|s| is_entity_insert(s, "external_entity"))
            .count();
        assert_eq!(
            ee_count,
            model.environment.sources.len() + model.environment.sinks.len()
        );

        let sys_count = stmts
            .iter()
            .filter(|s| is_entity_insert(s, "system"))
            .count();
        assert_eq!(sys_count, model.systems.len());

        let b_count = stmts
            .iter()
            .filter(|s| is_entity_insert(s, "boundary"))
            .count();
        assert_eq!(b_count, model.systems.len());

        let i_count = stmts
            .iter()
            .filter(|s| is_entity_insert(s, "interface"))
            .count();
        let expected_i: usize = model
            .systems
            .iter()
            .map(|s| s.boundary.interfaces.len())
            .sum();
        assert_eq!(i_count, expected_i);

        let f_count = stmts
            .iter()
            .filter(|s| is_entity_insert(s, "interaction"))
            .count();
        assert_eq!(f_count, model.interactions.len());
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
                    ["Resource", "Disruption", "Product", "Waste"].contains(&v.as_str()),
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
            .find(|s| s.contains(r#"has bert_id "bitcoin:F0.3""#) && s.starts_with("insert "))
            .expect("F0.3 interaction should be emitted");
        assert!(
            f03_insert.contains(r#"has interaction_type "Force""#),
            "F0.3 lost its Force type during transpilation: {f03_insert}"
        );
    }

    // -------------------------------------------------------------------
    // Phase 2 relation tests
    // -------------------------------------------------------------------

    #[test]
    fn bitcoin_produces_expected_relation_counts() {
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();

        // has_boundary: one per system
        let has_boundary = stmts
            .iter()
            .filter(|s| s.contains("isa has_boundary"))
            .count();
        assert_eq!(has_boundary, model.systems.len());

        // has_interface: one per interface
        let has_interface = stmts
            .iter()
            .filter(|s| s.contains("isa has_interface"))
            .count();
        let expected_interfaces: usize = model
            .systems
            .iter()
            .map(|s| s.boundary.interfaces.len())
            .sum();
        assert_eq!(has_interface, expected_interfaces);

        // composition: one per system whose parent isn't E-1
        let composition = stmts
            .iter()
            .filter(|s| s.contains("isa composition"))
            .count();
        let expected_comp = model
            .systems
            .iter()
            .filter(|s| !is_environment_id(&s.parent))
            .count();
        assert_eq!(composition, expected_comp);

        // in_environment: one per root system + one per external entity
        let in_env = stmts
            .iter()
            .filter(|s| s.contains("isa in_environment"))
            .count();
        let expected_in_env = model
            .systems
            .iter()
            .filter(|s| is_environment_id(&s.parent))
            .count()
            + model.environment.sources.len()
            + model.environment.sinks.len();
        assert_eq!(in_env, expected_in_env);

        // participates_in: two per interaction (source + sink)
        let participates = stmts
            .iter()
            .filter(|s| s.contains("isa participates_in"))
            .count();
        assert_eq!(participates, model.interactions.len() * 2);
    }

    #[test]
    fn participates_in_roles_are_source_or_sink_only() {
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();

        for stmt in stmts.iter().filter(|s| s.contains("isa participates_in")) {
            let role_marker = r#"has participation_role ""#;
            let pos = stmt
                .find(role_marker)
                .expect("participates_in missing participation_role");
            let rest = &stmt[pos + role_marker.len()..];
            let end = rest.find('"').unwrap_or(rest.len());
            let role = &rest[..end];
            assert!(
                role == "source" || role == "sink",
                "participates_in role out of vocab: '{role}' in {stmt}"
            );
        }
    }

    #[test]
    fn routes_through_endpoints_are_start_or_end_only() {
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();

        for stmt in stmts.iter().filter(|s| s.contains("isa routes_through")) {
            let marker = r#"has endpoint ""#;
            let pos = stmt.find(marker).expect("routes_through missing endpoint");
            let rest = &stmt[pos + marker.len()..];
            let end = rest.find('"').unwrap_or(rest.len());
            let val = &rest[..end];
            assert!(
                val == "start" || val == "end",
                "routes_through endpoint out of vocab: '{val}' in {stmt}"
            );
        }
    }

    #[test]
    fn bitcoin_users_pair_produces_one_is_equivalent_to() {
        // Per #14: Src-1.0 and Snk-1.1 ("Users") share is_same_as_id=0.
        // Pairwise emission for N=2 should yield exactly 1 relation.
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();
        let eq_count = stmts
            .iter()
            .filter(|s| s.contains("isa is_equivalent_to"))
            .count();
        assert_eq!(
            eq_count, 1,
            "expected 1 is_equivalent_to for the Users pair, got {eq_count}"
        );

        let eq_stmt = stmts
            .iter()
            .find(|s| s.contains("isa is_equivalent_to"))
            .unwrap();
        assert!(
            eq_stmt.contains(r#"bitcoin:Src-1.0"#) && eq_stmt.contains(r#"bitcoin:Snk-1.1"#),
            "is_equivalent_to does not link Src-1.0 ↔ Snk-1.1: {eq_stmt}"
        );
    }

    #[test]
    fn routes_through_emitted_only_when_interface_set() {
        // F0.3 Protocol → Mining is an internal flow — its source_interface
        // and sink_interface are both null, so it should produce zero
        // routes_through statements. F-1.2 Users → S0 (via I0.52) should
        // produce exactly one routes_through (endpoint=end).
        let model = load_bitcoin();
        let stmts = model_to_typeql(&model, "bitcoin").unwrap();

        let f03_routes = stmts
            .iter()
            .filter(|s| s.contains("isa routes_through") && s.contains("bitcoin:F0.3"))
            .count();
        assert_eq!(
            f03_routes, 0,
            "F0.3 is internal flow — no interface routing"
        );

        // Count routes_through in total — should equal the sum of non-null
        // source_interface and sink_interface across all interactions.
        let expected_routes: usize = model
            .interactions
            .iter()
            .map(|ix| {
                usize::from(ix.source_interface.is_some())
                    + usize::from(ix.sink_interface.is_some())
            })
            .sum();
        let actual_routes = stmts
            .iter()
            .filter(|s| s.contains("isa routes_through"))
            .count();
        assert_eq!(
            actual_routes, expected_routes,
            "routes_through count diverges from source/sink_interface presence"
        );
    }
}
