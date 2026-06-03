//! BERT validator 2 of 3 — pre-render, structural (operates on a loaded `WorldModel`; errors block load).
//! Siblings: `general-systems-reasoner/core/src/constraints.rs` (generation-time, spec `Value`),
//! `bert/tools/bert-typedb/src/validate.rs` (pre-transpile). See the bert-dev skill "Validators".

use super::*;
use std::collections::{HashMap, HashSet};

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
    pub suggestion: Option<String>,
}

impl ValidationIssue {
    fn error(
        location: impl Into<String>,
        message: impl Into<String>,
        suggestion: Option<&str>,
    ) -> Self {
        Self {
            severity: Severity::Error,
            location: location.into(),
            message: message.into(),
            suggestion: suggestion.map(|s| s.to_string()),
        }
    }

    fn warning(
        location: impl Into<String>,
        message: impl Into<String>,
        suggestion: Option<&str>,
    ) -> Self {
        Self {
            severity: Severity::Warning,
            location: location.into(),
            message: message.into(),
            suggestion: suggestion.map(|s| s.to_string()),
        }
    }
}

pub struct ValidationResult {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn has_errors(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.issues.iter().any(|i| i.severity == Severity::Warning)
    }

    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }
}

pub fn validate(model: &WorldModel) -> ValidationResult {
    let mut issues = Vec::new();

    let known_ids = collect_known_ids(model);
    let interface_ids = collect_interface_ids(model);

    check_orphan_sources(model, &mut issues);
    check_orphan_sinks(model, &mut issues);
    check_interaction_references(model, &known_ids, &mut issues);
    check_interface_references(model, &interface_ids, &mut issues);
    check_orphan_interfaces(model, &mut issues);
    check_parent_references(model, &known_ids, &mut issues);
    check_duplicate_ids(model, &mut issues);

    check_environment_id(model, &mut issues);
    check_source_sink_type_consistency(model, &mut issues);
    check_version(model, &mut issues);
    check_level_consistency(model, &mut issues);
    check_processor_flows(model, &mut issues);
    check_s0_interface_processors(model, &mut issues);

    ValidationResult { issues }
}

/// Classify a model as open or closed *with respect to mass*, returning a short
/// human-readable note for the UI (shown as a non-blocking toast on load).
///
/// Closed = no Energy/Material flow crosses the boundary (every mass flow runs
/// system↔system); open = some mass flow has an external `Source` or `Sink` endpoint.
/// Message flows are ignored (information is not conserved). An empty environment is
/// the *signature* of a closed system, not a defect — so instead of staying silent on
/// such a model, the loader names the regime and its conservation invariant.
pub fn classify_openness(model: &WorldModel) -> String {
    use crate::{InteractionType, SubstanceType};

    let mut inflows: Vec<String> = Vec::new();
    let mut outflows: Vec<String> = Vec::new();
    for ix in &model.interactions {
        if matches!(ix.ty, InteractionType::Force) {
            continue;
        }
        if !matches!(
            ix.substance.ty,
            SubstanceType::Energy | SubstanceType::Material
        ) {
            continue;
        }
        if ix.source.ty == IdType::Source {
            inflows.push(ix.info.name.clone());
        }
        if ix.sink.ty == IdType::Sink {
            outflows.push(ix.info.name.clone());
        }
    }

    if inflows.is_empty() && outflows.is_empty() {
        "Closed system (mass): no Energy/Material flow crosses the boundary — total mass is conserved.".to_string()
    } else {
        format!(
            "Open system (mass): mass crosses the boundary (in: [{}], out: [{}]) — internal mass changes by net flux.",
            inflows.join(", "),
            outflows.join(", "),
        )
    }
}

fn serialize_id(id: &Id) -> String {
    serde_json::to_string(id)
        .ok()
        .and_then(|s| {
            s.strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .map(|s| s.to_string())
        })
        .unwrap_or_default()
}

fn collect_known_ids(model: &WorldModel) -> HashSet<String> {
    let mut known = HashSet::new();
    known.insert(serialize_id(&model.environment.info.id));
    for system in &model.systems {
        known.insert(serialize_id(&system.info.id));
        for src in &system.sources {
            known.insert(serialize_id(&src.info.id));
        }
        for snk in &system.sinks {
            known.insert(serialize_id(&snk.info.id));
        }
        for iface in &system.boundary.interfaces {
            known.insert(serialize_id(&iface.info.id));
        }
    }
    for src in &model.environment.sources {
        known.insert(serialize_id(&src.info.id));
    }
    for snk in &model.environment.sinks {
        known.insert(serialize_id(&snk.info.id));
    }
    known
}

fn collect_interface_ids(model: &WorldModel) -> HashSet<String> {
    let mut ids = HashSet::new();
    for system in &model.systems {
        for iface in &system.boundary.interfaces {
            ids.insert(serialize_id(&iface.info.id));
        }
    }
    ids
}

fn check_orphan_sources(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    let referenced_sources: HashSet<String> = model
        .interactions
        .iter()
        .filter(|ix| ix.source.ty == IdType::Source)
        .map(|ix| serialize_id(&ix.source))
        .collect();

    let mut check_sources = |sources: &[ExternalEntity], loc_prefix: &str| {
        for (i, src) in sources.iter().enumerate() {
            let id_str = serialize_id(&src.info.id);
            if !referenced_sources.contains(&id_str) {
                issues.push(ValidationIssue::error(
                    format!("{loc_prefix}.sources[{i}]"),
                    format!("orphan source '{id_str}' is not referenced by any interaction"),
                    Some("Add an interaction with this source, or remove it"),
                ));
            }
        }
    };

    check_sources(&model.environment.sources, "environment");
    for (i, system) in model.systems.iter().enumerate() {
        check_sources(&system.sources, &format!("systems[{i}]"));
    }
}

fn check_orphan_sinks(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    let referenced_sinks: HashSet<String> = model
        .interactions
        .iter()
        .filter(|ix| ix.sink.ty == IdType::Sink)
        .map(|ix| serialize_id(&ix.sink))
        .collect();

    let mut check_sinks = |sinks: &[ExternalEntity], loc_prefix: &str| {
        for (i, snk) in sinks.iter().enumerate() {
            let id_str = serialize_id(&snk.info.id);
            if !referenced_sinks.contains(&id_str) {
                issues.push(ValidationIssue::error(
                    format!("{loc_prefix}.sinks[{i}]"),
                    format!("orphan sink '{id_str}' is not referenced by any interaction"),
                    Some("Add an interaction with this sink, or remove it"),
                ));
            }
        }
    };

    check_sinks(&model.environment.sinks, "environment");
    for (i, system) in model.systems.iter().enumerate() {
        check_sinks(&system.sinks, &format!("systems[{i}]"));
    }
}

fn check_interaction_references(
    model: &WorldModel,
    known: &HashSet<String>,
    issues: &mut Vec<ValidationIssue>,
) {
    for (i, ix) in model.interactions.iter().enumerate() {
        let src = serialize_id(&ix.source);
        if !known.contains(&src) {
            issues.push(ValidationIssue::error(
                format!("interactions[{i}].source"),
                format!("source '{src}' does not resolve to any known entity"),
                Some("Check the source ID matches an existing system, source, or sink"),
            ));
        }
        let snk = serialize_id(&ix.sink);
        if !known.contains(&snk) {
            issues.push(ValidationIssue::error(
                format!("interactions[{i}].sink"),
                format!("sink '{snk}' does not resolve to any known entity"),
                Some("Check the sink ID matches an existing system, source, or sink"),
            ));
        }
    }
}

fn check_interface_references(
    model: &WorldModel,
    interfaces: &HashSet<String>,
    issues: &mut Vec<ValidationIssue>,
) {
    for (i, ix) in model.interactions.iter().enumerate() {
        if let Some(ref src_iface) = ix.source_interface {
            let id_str = serialize_id(src_iface);
            if !interfaces.contains(&id_str) {
                issues.push(ValidationIssue::error(
                    format!("interactions[{i}].source_interface"),
                    format!("source_interface '{id_str}' does not resolve to any known interface"),
                    Some("Check the interface ID exists on the source system's boundary"),
                ));
            }
        }
        if let Some(ref snk_iface) = ix.sink_interface {
            let id_str = serialize_id(snk_iface);
            if !interfaces.contains(&id_str) {
                issues.push(ValidationIssue::error(
                    format!("interactions[{i}].sink_interface"),
                    format!("sink_interface '{id_str}' does not resolve to any known interface"),
                    Some("Check the interface ID exists on the sink system's boundary"),
                ));
            }
        }
    }
}

fn check_orphan_interfaces(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    let mut referenced: HashSet<String> = HashSet::new();

    for ix in &model.interactions {
        if let Some(ref id) = ix.source_interface {
            referenced.insert(serialize_id(id));
        }
        if let Some(ref id) = ix.sink_interface {
            referenced.insert(serialize_id(id));
        }
    }

    for system in &model.systems {
        if let Some(ref id) = system.boundary.parent_interface {
            referenced.insert(serialize_id(id));
        }
    }

    for (i, system) in model.systems.iter().enumerate() {
        for (j, iface) in system.boundary.interfaces.iter().enumerate() {
            let id_str = serialize_id(&iface.info.id);
            if !referenced.contains(&id_str) {
                issues.push(ValidationIssue::warning(
                    format!("systems[{i}].boundary.interfaces[{j}]"),
                    format!("interface '{id_str}' has no flow routing and no attached processor"),
                    Some("Add a flow using this interface, attach an interface processor, or remove it if unused"),
                ));
            }
        }
    }
}

fn check_parent_references(
    model: &WorldModel,
    known: &HashSet<String>,
    issues: &mut Vec<ValidationIssue>,
) {
    for (i, system) in model.systems.iter().enumerate() {
        let parent = serialize_id(&system.parent);
        if !known.contains(&parent) {
            issues.push(ValidationIssue::error(
                format!("systems[{i}].parent"),
                format!("parent '{parent}' does not resolve to any known entity"),
                Some("Parent must be 'E-1' (environment) or an existing system ID"),
            ));
        }
    }
}

fn check_duplicate_ids(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    let mut seen: HashMap<String, String> = HashMap::new();

    let mut record = |id_str: String, location: String, issues: &mut Vec<ValidationIssue>| {
        if let Some(prior) = seen.insert(id_str.clone(), location.clone()) {
            issues.push(ValidationIssue::error(
                &location,
                format!("duplicate ID '{id_str}' (first seen at {prior})"),
                Some("Each entity must have a unique ID"),
            ));
        }
    };

    for (i, system) in model.systems.iter().enumerate() {
        record(
            serialize_id(&system.info.id),
            format!("systems[{i}].info.id"),
            issues,
        );
        for (j, src) in system.sources.iter().enumerate() {
            record(
                serialize_id(&src.info.id),
                format!("systems[{i}].sources[{j}].info.id"),
                issues,
            );
        }
        for (j, snk) in system.sinks.iter().enumerate() {
            record(
                serialize_id(&snk.info.id),
                format!("systems[{i}].sinks[{j}].info.id"),
                issues,
            );
        }
        for (j, iface) in system.boundary.interfaces.iter().enumerate() {
            record(
                serialize_id(&iface.info.id),
                format!("systems[{i}].boundary.interfaces[{j}].info.id"),
                issues,
            );
        }
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
    for (i, ix) in model.interactions.iter().enumerate() {
        record(
            serialize_id(&ix.info.id),
            format!("interactions[{i}].info.id"),
            issues,
        );
    }
}

fn check_environment_id(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    let env_id = serialize_id(&model.environment.info.id);
    if env_id != "E-1" {
        issues.push(ValidationIssue::warning(
            "environment.info.id",
            format!("environment ID is '{env_id}', expected 'E-1'"),
            Some("The environment entity should always have ID 'E-1'"),
        ));
    }
}

fn check_source_sink_type_consistency(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    let check = |sources: &[ExternalEntity],
                 sinks: &[ExternalEntity],
                 loc_prefix: &str,
                 issues: &mut Vec<ValidationIssue>| {
        for (i, src) in sources.iter().enumerate() {
            if !matches!(src.ty, ExternalEntityType::Source) {
                issues.push(ValidationIssue::warning(
                    format!("{loc_prefix}.sources[{i}].type"),
                    "entity in sources array has type 'Sink'".to_string(),
                    Some("Entities in the sources array should have type 'Source'"),
                ));
            }
        }
        for (i, snk) in sinks.iter().enumerate() {
            if !matches!(snk.ty, ExternalEntityType::Sink) {
                issues.push(ValidationIssue::warning(
                    format!("{loc_prefix}.sinks[{i}].type"),
                    "entity in sinks array has type 'Source'".to_string(),
                    Some("Entities in the sinks array should have type 'Sink'"),
                ));
            }
        }
    };

    check(
        &model.environment.sources,
        &model.environment.sinks,
        "environment",
        issues,
    );
    for (i, system) in model.systems.iter().enumerate() {
        check(
            &system.sources,
            &system.sinks,
            &format!("systems[{i}]"),
            issues,
        );
    }
}

fn check_version(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    if model.version != CURRENT_FILE_VERSION {
        issues.push(ValidationIssue::warning(
            "version",
            format!(
                "model version is {}, current is {CURRENT_FILE_VERSION}",
                model.version
            ),
            Some("This model may have been created with a different version of BERT"),
        ));
    }
}

fn check_level_consistency(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    for (i, system) in model.systems.iter().enumerate() {
        let expected = (system.info.id.indices.len() as i32) - 1;
        if system.info.level != expected {
            issues.push(ValidationIssue::warning(
                format!("systems[{i}].info.level"),
                format!(
                    "level is {} but ID '{}' implies level {}",
                    system.info.level,
                    serialize_id(&system.info.id),
                    expected
                ),
                Some("Level should equal the number of ID indices minus one"),
            ));
        }
    }
}

// ── L3: Pre-parse structural validation ──────────────────────

const WORLD_MODEL_FIELDS: &[&str] = &["version", "environment", "systems", "interactions"];
const SYSTEM_FIELDS: &[&str] = &[
    "info",
    "sources",
    "sinks",
    "parent",
    "complexity",
    "boundary",
    "radius",
    "equivalence",
    "history",
    "transformation",
    "member_autonomy",
    "time_constant",
];
const INFO_FIELDS: &[&str] = &["id", "level", "name", "description"];
const BOUNDARY_FIELDS: &[&str] = &["info", "porosity", "perceptive_fuzziness", "interfaces"];
const ENVIRONMENT_FIELDS: &[&str] = &["info", "sources", "sinks"];
const INTERACTION_FIELDS: &[&str] = &[
    "info",
    "substance",
    "type",
    "usability",
    "source",
    "sink",
    "amount",
    "unit",
    "parameters",
];
const EXTERNAL_ENTITY_FIELDS: &[&str] = &["info", "type", "equivalence", "model"];
const INTERFACE_FIELDS: &[&str] = &["info", "protocol", "type", "exports_to", "receives_from"];

fn check_required_fields(
    obj: &serde_json::Value,
    required: &[&str],
    location: &str,
    issues: &mut Vec<ValidationIssue>,
) {
    if let Some(map) = obj.as_object() {
        for &field in required {
            if !map.contains_key(field) {
                issues.push(ValidationIssue::error(
                    location,
                    format!("Missing required field '{field}'"),
                    Some(&format!("Add the '{field}' field to {location}")),
                ));
            }
        }
    } else {
        issues.push(ValidationIssue::error(
            location,
            format!("Expected an object, found {}", json_type_name(obj)),
            None,
        ));
    }
}

fn json_type_name(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

fn name_from_info(obj: &serde_json::Value) -> String {
    obj.get("info")
        .and_then(|i| i.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("(unnamed)")
        .to_string()
}

pub fn validate_json_structure(json: &serde_json::Value) -> ValidationResult {
    let mut issues = Vec::new();

    check_required_fields(json, WORLD_MODEL_FIELDS, "root", &mut issues);

    if let Some(env) = json.get("environment") {
        check_required_fields(env, ENVIRONMENT_FIELDS, "environment", &mut issues);
        if let Some(info) = env.get("info") {
            check_required_fields(info, INFO_FIELDS, "environment.info", &mut issues);
        }
        for (label, key) in [("sources", "sources"), ("sinks", "sinks")] {
            if let Some(arr) = env.get(key).and_then(|v| v.as_array()) {
                for (i, ent) in arr.iter().enumerate() {
                    let loc = format!("environment.{label}[{i}] '{}'", name_from_info(ent));
                    check_required_fields(ent, EXTERNAL_ENTITY_FIELDS, &loc, &mut issues);
                }
            }
        }
    }

    if let Some(systems) = json.get("systems").and_then(|v| v.as_array()) {
        for (i, sys) in systems.iter().enumerate() {
            let name = name_from_info(sys);
            let loc = format!("systems[{i}] '{name}'");
            check_required_fields(sys, SYSTEM_FIELDS, &loc, &mut issues);

            if let Some(info) = sys.get("info") {
                check_required_fields(info, INFO_FIELDS, &format!("{loc}.info"), &mut issues);
            }
            if let Some(boundary) = sys.get("boundary") {
                check_required_fields(
                    boundary,
                    BOUNDARY_FIELDS,
                    &format!("{loc}.boundary"),
                    &mut issues,
                );
                if let Some(info) = boundary.get("info") {
                    check_required_fields(
                        info,
                        INFO_FIELDS,
                        &format!("{loc}.boundary.info"),
                        &mut issues,
                    );
                }
                if let Some(ifaces) = boundary.get("interfaces").and_then(|v| v.as_array()) {
                    for (j, iface) in ifaces.iter().enumerate() {
                        let iname = name_from_info(iface);
                        let iloc = format!("{loc}.boundary.interfaces[{j}] '{iname}'");
                        check_required_fields(iface, INTERFACE_FIELDS, &iloc, &mut issues);
                    }
                }
            }
            for (label, key) in [("sources", "sources"), ("sinks", "sinks")] {
                if let Some(arr) = sys.get(key).and_then(|v| v.as_array()) {
                    for (j, ent) in arr.iter().enumerate() {
                        let eloc = format!("{loc}.{label}[{j}] '{}'", name_from_info(ent));
                        check_required_fields(ent, EXTERNAL_ENTITY_FIELDS, &eloc, &mut issues);
                    }
                }
            }
        }
    }

    if let Some(interactions) = json.get("interactions").and_then(|v| v.as_array()) {
        for (i, flow) in interactions.iter().enumerate() {
            let name = name_from_info(flow);
            let loc = format!("interactions[{i}] '{name}'");
            check_required_fields(flow, INTERACTION_FIELDS, &loc, &mut issues);
        }
    }

    ValidationResult { issues }
}

// ── L4: Processor boundary-tracing ───────────────────────────

fn check_processor_flows(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    for system in &model.systems {
        if system.boundary.parent_interface.is_none() {
            continue;
        }
        let sys_id = serialize_id(&system.info.id);
        let is_source = model
            .interactions
            .iter()
            .any(|f| serialize_id(&f.source) == sys_id);
        let is_sink = model
            .interactions
            .iter()
            .any(|f| serialize_id(&f.sink) == sys_id);
        if !is_source && !is_sink {
            issues.push(ValidationIssue::warning(
                format!("systems.{name}", name = system.info.name),
                format!(
                    "Processor '{}' has parent_interface but no connecting flows",
                    system.info.name
                ),
                Some("Import processors should be a source in at least one flow; export processors should be a sink"),
            ));
        }
    }
}

fn check_s0_interface_processors(model: &WorldModel, issues: &mut Vec<ValidationIssue>) {
    let s0_entry = model
        .systems
        .iter()
        .enumerate()
        .find(|(_, s)| s.info.level == 0);
    let (s0_idx, s0) = match s0_entry {
        Some(entry) => entry,
        None => return,
    };

    let claimed: HashSet<String> = model
        .systems
        .iter()
        .filter_map(|s| s.boundary.parent_interface.as_ref())
        .map(serialize_id)
        .collect();

    for (j, iface) in s0.boundary.interfaces.iter().enumerate() {
        let id_str = serialize_id(&iface.info.id);
        if !claimed.contains(&id_str) {
            issues.push(ValidationIssue::warning(
                format!("systems[{s0_idx}].boundary.interfaces[{j}]"),
                format!(
                    "Interface '{}' has no processor — external flows won't trace to internal subsystems",
                    iface.info.name
                ),
                Some("Add a level-1 subsystem with boundary.parent_interface pointing to this interface"),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_example_model(name: &str) -> WorldModel {
        let path = format!(
            "{}/assets/models/examples/{name}",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = std::fs::read(&path).unwrap_or_else(|_| panic!("should read {path}"));
        serde_json::from_slice(&bytes).unwrap_or_else(|_| panic!("should parse {name}"))
    }

    #[test]
    fn all_example_models_validate_without_errors() {
        let dir = format!("{}/assets/models/examples", env!("CARGO_MANIFEST_DIR"));
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let name = path.file_name().unwrap().to_str().unwrap();
            let model = load_example_model(name);
            let result = validate(&model);
            assert!(
                !result.has_errors(),
                "{name} should have no errors; got: {:#?}",
                result
                    .issues
                    .iter()
                    .filter(|i| i.severity == Severity::Error)
                    .collect::<Vec<_>>()
            );
        }
    }

    fn minimal_model() -> WorldModel {
        let env_id = Id {
            ty: IdType::Environment,
            indices: vec![-1],
        };
        WorldModel {
            version: CURRENT_FILE_VERSION,
            environment: Environment {
                info: Info {
                    id: env_id.clone(),
                    level: -1,
                    name: String::new(),
                    description: String::new(),
                },
                sources: vec![],
                sinks: vec![],
            },
            systems: vec![System {
                info: Info {
                    id: Id {
                        ty: IdType::System,
                        indices: vec![0],
                    },
                    level: 0,
                    name: "Test".to_string(),
                    description: String::new(),
                },
                sources: vec![],
                sinks: vec![],
                parent: env_id.clone(),
                complexity: Complexity::Atomic,
                boundary: Boundary {
                    info: Info {
                        id: Id {
                            ty: IdType::Boundary,
                            indices: vec![0],
                        },
                        level: 0,
                        name: String::new(),
                        description: String::new(),
                    },
                    porosity: 0.0,
                    perceptive_fuzziness: 0.0,
                    interfaces: vec![],
                    parent_interface: None,
                },
                radius: 100.0,
                transform: None,
                equivalence: String::new(),
                history: String::new(),
                transformation: String::new(),
                member_autonomy: 1.0,
                time_constant: "Second".to_string(),
                archetype: None,
                agent: None,
            }],
            interactions: vec![],
            hidden_entities: vec![],
        }
    }

    #[test]
    fn minimal_model_validates_clean() {
        let model = minimal_model();
        let result = validate(&model);
        assert!(result.is_clean(), "got: {:#?}", result.issues);
    }

    #[test]
    fn orphan_source_is_error() {
        let mut model = minimal_model();
        model.environment.sources.push(ExternalEntity {
            info: Info {
                id: Id {
                    ty: IdType::Source,
                    indices: vec![-1, 0],
                },
                level: -1,
                name: "Orphan".to_string(),
                description: String::new(),
            },
            ty: ExternalEntityType::Source,
            transform: None,
            equivalence: String::new(),
            model: String::new(),
            is_same_as_id: None,
        });
        let result = validate(&model);
        assert!(result.has_errors());
        assert!(result
            .issues
            .iter()
            .any(|i| i.message.contains("orphan source")));
    }

    #[test]
    fn orphan_sink_is_error() {
        let mut model = minimal_model();
        model.environment.sinks.push(ExternalEntity {
            info: Info {
                id: Id {
                    ty: IdType::Sink,
                    indices: vec![-1, 0],
                },
                level: -1,
                name: "Orphan".to_string(),
                description: String::new(),
            },
            ty: ExternalEntityType::Sink,
            transform: None,
            equivalence: String::new(),
            model: String::new(),
            is_same_as_id: None,
        });
        let result = validate(&model);
        assert!(result.has_errors());
        assert!(result
            .issues
            .iter()
            .any(|i| i.message.contains("orphan sink")));
    }

    #[test]
    fn dangling_interaction_source_is_error() {
        let mut model = minimal_model();
        model.interactions.push(Interaction {
            info: Info {
                id: Id {
                    ty: IdType::Flow,
                    indices: vec![-1, 0],
                },
                level: -1,
                name: "Ghost".to_string(),
                description: String::new(),
            },
            substance: Substance {
                sub_type: String::new(),
                ty: SubstanceType::Message,
            },
            ty: InteractionType::Flow,
            usability: InteractionUsability::Product,
            source: Id {
                ty: IdType::Source,
                indices: vec![-1, 99],
            },
            source_interface: None,
            sink: Id {
                ty: IdType::System,
                indices: vec![0],
            },
            sink_interface: None,
            amount: rust_decimal::Decimal::ZERO,
            unit: String::new(),
            parameters: vec![],
            smart_parameters: vec![],
            endpoint_offset: None,
        });
        let result = validate(&model);
        assert!(result.has_errors());
        assert!(result
            .issues
            .iter()
            .any(|i| i.location.contains("source") && i.message.contains("does not resolve")));
    }

    #[test]
    fn wrong_version_is_warning() {
        let mut model = minimal_model();
        model.version = 999;
        let result = validate(&model);
        assert!(!result.has_errors());
        assert!(result.has_warnings());
        assert!(result.issues.iter().any(|i| i.location == "version"));
    }

    #[test]
    fn level_mismatch_is_warning() {
        let mut model = minimal_model();
        model.systems[0].info.level = 5;
        let result = validate(&model);
        assert!(!result.has_errors());
        assert!(result.has_warnings());
        assert!(result.issues.iter().any(|i| i.location.contains("level")));
    }

    #[test]
    fn preparse_missing_radius_is_error() {
        let json: serde_json::Value = serde_json::json!({
            "version": 1,
            "environment": {
                "info": { "id": {"ty": "Environment", "indices": [-1]}, "level": -1, "name": "", "description": "" },
                "sources": [],
                "sinks": []
            },
            "systems": [{
                "info": { "id": {"ty": "System", "indices": [0]}, "level": 0, "name": "Test", "description": "" },
                "sources": [], "sinks": [],
                "parent": {"ty": "Environment", "indices": [-1]},
                "complexity": "Atomic",
                "boundary": {
                    "info": { "id": {"ty": "Boundary", "indices": [0]}, "level": 0, "name": "", "description": "" },
                    "porosity": 0.0, "perceptive_fuzziness": 0.0, "interfaces": []
                },
                "equivalence": "", "history": "", "transformation": "",
                "member_autonomy": 1.0, "time_constant": "Second"
            }],
            "interactions": []
        });
        let result = validate_json_structure(&json);
        assert!(result.has_errors(), "should catch missing radius");
        assert!(result.issues.iter().any(|i| i.message.contains("radius")));
    }

    #[test]
    fn preparse_complete_model_is_clean() {
        let path = format!(
            "{}/assets/models/examples/bitcoin.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = std::fs::read(&path).expect("should read bitcoin.json");
        let json: serde_json::Value = serde_json::from_slice(&bytes).expect("should parse JSON");
        let result = validate_json_structure(&json);
        assert!(
            result.is_clean(),
            "coffee_shop should pre-parse clean; got: {:#?}",
            result.issues
        );
    }

    #[test]
    fn preparse_all_examples_clean() {
        let dir = format!("{}/assets/models/examples", env!("CARGO_MANIFEST_DIR"));
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let name = path.file_name().unwrap().to_str().unwrap();
            let bytes = std::fs::read(&path).unwrap_or_else(|_| panic!("should read {name}"));
            let json: serde_json::Value =
                serde_json::from_slice(&bytes).unwrap_or_else(|_| panic!("should parse {name}"));
            let result = validate_json_structure(&json);
            assert!(
                result.is_clean(),
                "{name} should pre-parse clean; got {} issues: {:#?}",
                result.issues.len(),
                result.issues
            );
        }
    }

    #[test]
    fn processor_without_flows_is_warning() {
        let mut model = minimal_model();
        let proc_id = Id {
            ty: IdType::System,
            indices: vec![0, 0],
        };
        let parent_iface_id = Id {
            ty: IdType::Interface,
            indices: vec![0, 0],
        };
        model.systems[0].boundary.interfaces.push(Interface {
            info: Info {
                id: parent_iface_id.clone(),
                level: 1,
                name: "I0".to_string(),
                description: String::new(),
            },
            protocol: String::new(),
            ty: InterfaceType::Import,
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(0.0),
        });
        model.systems.push(System {
            info: Info {
                id: proc_id,
                level: 1,
                name: "Orphan Processor".to_string(),
                description: String::new(),
            },
            sources: vec![],
            sinks: vec![],
            parent: model.systems[0].info.id.clone(),
            complexity: Complexity::Atomic,
            boundary: Boundary {
                info: Info {
                    id: Id {
                        ty: IdType::Boundary,
                        indices: vec![0, 0],
                    },
                    level: 1,
                    name: String::new(),
                    description: String::new(),
                },
                porosity: 0.0,
                perceptive_fuzziness: 0.0,
                interfaces: vec![],
                parent_interface: Some(parent_iface_id),
            },
            radius: 50.0,
            transform: None,
            equivalence: String::new(),
            history: String::new(),
            transformation: String::new(),
            member_autonomy: 1.0,
            time_constant: "Second".to_string(),
            archetype: None,
            agent: None,
        });
        let result = validate(&model);
        assert!(result.has_warnings());
        assert!(result
            .issues
            .iter()
            .any(|i| i.message.contains("Processor") && i.message.contains("no connecting flows")));
    }

    #[test]
    fn orphan_interface_is_warning() {
        let mut model = minimal_model();
        model.systems[0].boundary.interfaces.push(Interface {
            info: Info {
                id: Id {
                    ty: IdType::Interface,
                    indices: vec![0, 0],
                },
                level: 1,
                name: "Orphan".to_string(),
                description: String::new(),
            },
            protocol: String::new(),
            ty: InterfaceType::Import,
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(0.0),
        });
        let result = validate(&model);
        assert!(!result.has_errors());
        assert!(result.has_warnings());
        assert!(result.issues.iter().any(|i| i
            .message
            .contains("no flow routing and no attached processor")));
    }

    #[test]
    fn env_id_wrong_is_warning() {
        let mut model = minimal_model();
        model.environment.info.id = Id {
            ty: IdType::Environment,
            indices: vec![0],
        };
        let result = validate(&model);
        assert!(result.has_warnings());
        assert!(result
            .issues
            .iter()
            .any(|i| i.location == "environment.info.id"));
    }

    #[test]
    fn s0_interface_without_processor_is_warning() {
        let mut model = minimal_model();
        let iface_id = Id {
            ty: IdType::Interface,
            indices: vec![0, 0],
        };
        model.systems[0].boundary.interfaces.push(Interface {
            info: Info {
                id: iface_id.clone(),
                level: 1,
                name: "Uncovered".to_string(),
                description: String::new(),
            },
            protocol: String::new(),
            ty: InterfaceType::Import,
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(0.0),
        });
        // Add a flow referencing the interface so check_orphan_interfaces doesn't fire
        model.environment.sources.push(ExternalEntity {
            info: Info {
                id: Id {
                    ty: IdType::Source,
                    indices: vec![-1, 0],
                },
                level: -1,
                name: "Src".to_string(),
                description: String::new(),
            },
            ty: ExternalEntityType::Source,
            transform: None,
            equivalence: String::new(),
            model: String::new(),
            is_same_as_id: None,
        });
        model.interactions.push(Interaction {
            info: Info {
                id: Id {
                    ty: IdType::Flow,
                    indices: vec![-1, 0],
                },
                level: -1,
                name: "Inflow".to_string(),
                description: String::new(),
            },
            substance: Substance {
                sub_type: String::new(),
                ty: SubstanceType::Message,
            },
            ty: InteractionType::Flow,
            usability: InteractionUsability::Product,
            source: Id {
                ty: IdType::Source,
                indices: vec![-1, 0],
            },
            source_interface: None,
            sink: Id {
                ty: IdType::System,
                indices: vec![0],
            },
            sink_interface: Some(iface_id),
            amount: rust_decimal::Decimal::ZERO,
            unit: String::new(),
            parameters: vec![],
            smart_parameters: vec![],
            endpoint_offset: None,
        });
        let result = validate(&model);
        assert!(!result.has_errors());
        assert!(result
            .issues
            .iter()
            .any(|i| i.message.contains("has no processor")));
    }

    #[test]
    fn s0_interface_with_processor_no_warning() {
        let mut model = minimal_model();
        let iface_id = Id {
            ty: IdType::Interface,
            indices: vec![0, 0],
        };
        model.systems[0].boundary.interfaces.push(Interface {
            info: Info {
                id: iface_id.clone(),
                level: 1,
                name: "Covered".to_string(),
                description: String::new(),
            },
            protocol: String::new(),
            ty: InterfaceType::Import,
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(0.0),
        });
        // Add a processor subsystem claiming this interface
        model.systems.push(System {
            info: Info {
                id: Id {
                    ty: IdType::System,
                    indices: vec![0, 0],
                },
                level: 1,
                name: "Processor".to_string(),
                description: String::new(),
            },
            sources: vec![],
            sinks: vec![],
            parent: model.systems[0].info.id.clone(),
            complexity: Complexity::Atomic,
            boundary: Boundary {
                info: Info {
                    id: Id {
                        ty: IdType::Boundary,
                        indices: vec![0, 0],
                    },
                    level: 1,
                    name: String::new(),
                    description: String::new(),
                },
                porosity: 0.0,
                perceptive_fuzziness: 0.0,
                interfaces: vec![],
                parent_interface: Some(iface_id.clone()),
            },
            radius: 12.0,
            transform: None,
            equivalence: String::new(),
            history: String::new(),
            transformation: String::new(),
            member_autonomy: 1.0,
            time_constant: "Second".to_string(),
            archetype: None,
            agent: None,
        });
        // Add a flow so the processor doesn't trigger check_processor_flows warning
        model.environment.sources.push(ExternalEntity {
            info: Info {
                id: Id {
                    ty: IdType::Source,
                    indices: vec![-1, 0],
                },
                level: -1,
                name: "Src".to_string(),
                description: String::new(),
            },
            ty: ExternalEntityType::Source,
            transform: None,
            equivalence: String::new(),
            model: String::new(),
            is_same_as_id: None,
        });
        model.interactions.push(Interaction {
            info: Info {
                id: Id {
                    ty: IdType::Flow,
                    indices: vec![-1, 0],
                },
                level: -1,
                name: "Inflow".to_string(),
                description: String::new(),
            },
            substance: Substance {
                sub_type: String::new(),
                ty: SubstanceType::Message,
            },
            ty: InteractionType::Flow,
            usability: InteractionUsability::Product,
            source: Id {
                ty: IdType::Source,
                indices: vec![-1, 0],
            },
            source_interface: None,
            sink: Id {
                ty: IdType::System,
                indices: vec![0, 0],
            },
            sink_interface: Some(iface_id),
            amount: rust_decimal::Decimal::ZERO,
            unit: String::new(),
            parameters: vec![],
            smart_parameters: vec![],
            endpoint_offset: None,
        });
        let result = validate(&model);
        assert!(
            !result
                .issues
                .iter()
                .any(|i| i.message.contains("has no processor")),
            "should not warn when processor exists; got: {:#?}",
            result.issues
        );
    }
}
