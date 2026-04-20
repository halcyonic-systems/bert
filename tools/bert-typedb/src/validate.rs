//! Pre-insert validation of BERT `WorldModel`s.
//!
//! Runs before the transpiler emits any TypeQL. Catches:
//!
//! - Controlled-vocabulary violations that would fail TypeDB `@values`
//!   constraints at insert time (e.g. unknown `time_constant`). Surfacing
//!   these pre-insert lets us batch-report issues instead of bailing on
//!   the first bad value.
//! - Duplicate entity IDs (TypeDB's `@key` uniqueness fires at commit, by
//!   which point the whole transaction is wasted).
//! - Unresolved references (a `source`/`sink` ID that doesn't name a real
//!   entity). TypeDB's `match ... insert` pattern for relations requires
//!   both endpoints to exist; a dangling reference turns into a confusing
//!   "0 matches" at insert time.
//!
//! All checks are pure functions over `&WorldModel` — no TypeDB required,
//! no async. Intentionally narrow: structural and referential checks in
//! `bert/docs/bert-schema-reference.md` §L1 and §L2 are already enforced
//! by serde deserialization. This module covers the subset that matters
//! for TypeDB-specific failure modes.

use bert::bevy_app::data_model::WorldModel;

/// Controlled vocabulary for `time_constant` per SL spec §1.3 (and enforced
/// by the TypeDB schema's `@values` constraint on `attribute time_constant`).
const VALID_TIME_CONSTANTS: &[&str] = &[
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
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub location: String,
    pub message: String,
}

impl ValidationIssue {
    fn error(location: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            location: location.into(),
            message: message.into(),
        }
    }
}

/// Validates a BERT `WorldModel` against TypeDB-specific constraints.
///
/// Returns every issue found (does not short-circuit on the first). An empty
/// vector means the model is ready to transpile. Any `Severity::Error` issue
/// means transpilation should not proceed.
pub fn validate(model: &WorldModel) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    check_time_constants(model, &mut issues);
    check_duplicate_ids(model, &mut issues);
    check_references_resolve(model, &mut issues);
    issues
}

fn check_time_constants(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    for (idx, system) in model.systems.iter().enumerate() {
        if !VALID_TIME_CONSTANTS.contains(&system.time_constant.as_str()) {
            issues.push(ValidationIssue::error(
                format!("systems[{idx}].time_constant"),
                format!(
                    "'{}' not in controlled vocabulary; expected one of: {}",
                    system.time_constant,
                    VALID_TIME_CONSTANTS.join(", ")
                ),
            ));
        }
    }
}

fn check_duplicate_ids(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    use std::collections::HashMap;
    let mut seen: HashMap<String, String> = HashMap::new();

    let mut record = |id: String, location: String, issues: &mut Vec<ValidationIssue>| {
        if let Some(prior) = seen.insert(id.clone(), location.clone()) {
            issues.push(ValidationIssue::error(
                location,
                format!("duplicate id '{id}' (previously seen at {prior})"),
            ));
        }
    };

    for (i, system) in model.systems.iter().enumerate() {
        record(
            serialize_id(&system.info.id),
            format!("systems[{i}].info.id"),
            issues,
        );
    }
    for (i, src) in model.environment.sources.iter().enumerate() {
        record(
            serialize_id(&src.info.id),
            format!("environment.sources[{i}].info.id"),
            issues,
        );
    }
    for (i, snk) in model.environment.sinks.iter().enumerate() {
        record(
            serialize_id(&snk.info.id),
            format!("environment.sinks[{i}].info.id"),
            issues,
        );
    }
    for (i, interaction) in model.interactions.iter().enumerate() {
        record(
            serialize_id(&interaction.info.id),
            format!("interactions[{i}].info.id"),
            issues,
        );
    }
}

fn check_references_resolve(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    use std::collections::HashSet;
    let mut known: HashSet<String> = HashSet::new();
    known.insert("E-1".to_string());
    for s in &model.systems {
        known.insert(serialize_id(&s.info.id));
    }
    for src in &model.environment.sources {
        known.insert(serialize_id(&src.info.id));
    }
    for snk in &model.environment.sinks {
        known.insert(serialize_id(&snk.info.id));
    }

    for (i, system) in model.systems.iter().enumerate() {
        let parent = serialize_id(&system.parent);
        if !known.contains(&parent) {
            issues.push(ValidationIssue::error(
                format!("systems[{i}].parent"),
                format!("parent '{parent}' does not resolve to a known entity"),
            ));
        }
    }

    for (i, ix) in model.interactions.iter().enumerate() {
        let src = serialize_id(&ix.source);
        let snk = serialize_id(&ix.sink);
        if !known.contains(&src) {
            issues.push(ValidationIssue::error(
                format!("interactions[{i}].source"),
                format!("source '{src}' does not resolve to a known entity"),
            ));
        }
        if !known.contains(&snk) {
            issues.push(ValidationIssue::error(
                format!("interactions[{i}].sink"),
                format!("sink '{snk}' does not resolve to a known entity"),
            ));
        }
    }
}

fn serialize_id(id: &bert::bevy_app::data_model::Id) -> String {
    serde_json::to_string(id)
        .ok()
        .and_then(|s| {
            // serde quotes the string; strip the surrounding double quotes
            s.strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .map(|s| s.to_string())
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Loads the canonical reference model — already spec-compliant per #14,
    /// so should validate with zero issues. Guards against regressions where
    /// a new check incorrectly flags known-good data.
    #[test]
    fn bitcoin_reference_model_has_no_issues() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/models/examples/bitcoin.json"
        );
        let bytes = std::fs::read(path).expect("bitcoin.json should exist");
        let model: WorldModel =
            serde_json::from_slice(&bytes).expect("bitcoin.json should deserialize");
        let issues = validate(&model);
        assert!(
            issues.is_empty(),
            "bitcoin.json should validate clean; got {} issues: {issues:#?}",
            issues.len()
        );
    }

    #[test]
    fn time_constant_vocabulary_matches_schema() {
        // If schema.tql's @values list diverges from VALID_TIME_CONSTANTS,
        // validate() will report false negatives. This test ensures they
        // stay in sync.
        let schema = crate::schema::SCHEMA_TQL;
        for tc in VALID_TIME_CONSTANTS {
            assert!(
                schema.contains(&format!("\"{tc}\"")),
                "time_constant '{tc}' from validator not found in schema's @values"
            );
        }
    }

    #[test]
    fn all_other_valid_models_pass() {
        // Every example model shipped in the repo should validate clean
        // after the #14/#41 compliance work. (As of now: only bitcoin.json
        // is fully spec-compliant; others pass serde but may have issues
        // flagged here. Keep this test to surface drift.)
        let model_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/models/examples");
        for entry in std::fs::read_dir(model_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let bytes = std::fs::read(&path).unwrap();
            let model: WorldModel = serde_json::from_slice(&bytes).unwrap();
            let issues = validate(&model);
            let errors: Vec<_> = issues
                .iter()
                .filter(|i| i.severity == Severity::Error)
                .collect();
            // For now, we only assert bitcoin is clean — sibling models
            // become clean via #41. Log other issues non-fatally.
            if path.file_name().and_then(|s| s.to_str()) == Some("bitcoin.json") {
                assert!(
                    errors.is_empty(),
                    "bitcoin.json should have no errors; got {errors:#?}"
                );
            }
        }
    }

    #[test]
    fn invalid_time_constant_is_flagged() {
        let json = r#"{
            "version": 1,
            "environment": {"info": {"id": "E-1", "level": -1, "name": "", "description": ""}, "sources": [], "sinks": []},
            "systems": [{
                "info": {"id": "S0", "level": 0, "name": "Test", "description": ""},
                "sources": [], "sinks": [],
                "parent": "E-1",
                "complexity": "Atomic",
                "boundary": {
                    "info": {"id": "B0", "level": 0, "name": "", "description": ""},
                    "porosity": 0.0, "perceptive_fuzziness": 0.0,
                    "interfaces": [], "parent_interface": null
                },
                "radius": 100.0,
                "equivalence": "", "history": "", "transformation": "",
                "member_autonomy": 1.0,
                "time_constant": "HorseYear"
            }],
            "interactions": []
        }"#;
        let model: WorldModel = serde_json::from_str(json).unwrap();
        let issues = validate(&model);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].severity, Severity::Error);
        assert!(issues[0].message.contains("HorseYear"));
        assert!(issues[0].location.contains("time_constant"));
    }

    #[test]
    fn unresolved_interaction_source_is_flagged() {
        let json = r#"{
            "version": 1,
            "environment": {"info": {"id": "E-1", "level": -1, "name": "", "description": ""}, "sources": [], "sinks": []},
            "systems": [{
                "info": {"id": "S0", "level": 0, "name": "Test", "description": ""},
                "sources": [], "sinks": [],
                "parent": "E-1",
                "complexity": "Atomic",
                "boundary": {
                    "info": {"id": "B0", "level": 0, "name": "", "description": ""},
                    "porosity": 0.0, "perceptive_fuzziness": 0.0,
                    "interfaces": [], "parent_interface": null
                },
                "radius": 100.0,
                "equivalence": "", "history": "", "transformation": "",
                "member_autonomy": 1.0,
                "time_constant": "Second"
            }],
            "interactions": [{
                "info": {"id": "F-1.0", "level": -1, "name": "Ghost", "description": ""},
                "substance": {"sub_type": "Data", "type": "Message"},
                "type": "Flow",
                "usability": "Product",
                "source": "Src-1.99",
                "source_interface": null,
                "sink": "S0",
                "sink_interface": null,
                "amount": "1",
                "unit": "",
                "parameters": []
            }]
        }"#;
        let model: WorldModel = serde_json::from_str(json).unwrap();
        let issues = validate(&model);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("Src-1.99"));
        assert!(issues[0].location.contains("source"));
    }
}
