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

    ValidationResult { issues }
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
    fn all_example_models_validate_clean() {
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
                result.is_clean(),
                "{name} should validate clean; got {} issues: {:#?}",
                result.issues.len(),
                result.issues
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
        assert!(result
            .issues
            .iter()
            .any(|i| i.message.contains("no flow routing and no attached processor")));
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
}
