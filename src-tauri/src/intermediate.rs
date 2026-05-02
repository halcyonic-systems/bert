/// Intermediate format types for the deterministic BERT model generator.
///
/// These structs represent the validated intermediate spec that the LLM produces
/// and the generator compiles into a complete BERT JSON WorldModel.
use serde::{Deserialize, Serialize};

/// Top-level intermediate spec — the input to `BertModelGenerator`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntermediateSpec {
    // May be nested under "system" key or flat at top level.
    #[serde(default)]
    pub system: Option<SystemSpec>,

    // Flat top-level fields (after normalization)
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub environment_name: Option<String>,
    #[serde(default)]
    pub environment_description: Option<String>,

    #[serde(default)]
    pub sources: Vec<Source>,
    #[serde(default)]
    pub sinks: Vec<Sink>,

    // routing_table is the user-facing format; normalized to interfaces
    #[serde(default)]
    pub routing_table: Vec<RoutingEntry>,

    // After normalization, interfaces replaces routing_table
    #[serde(default)]
    pub interfaces: Vec<InterfaceSpec>,

    #[serde(default)]
    pub subsystems: Vec<Subsystem>,

    #[serde(default)]
    pub external_flows: Vec<ExternalFlow>,
    #[serde(default)]
    pub internal_flows: Vec<InternalFlow>,
    #[serde(default)]
    pub processor_flows: Vec<ProcessorFlow>,
}

/// Nested system metadata (name, description, complexity).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemSpec {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub complexity: Option<String>,
}

/// External source of flows into the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// External sink for flows out of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sink {
    pub name: String,
    #[serde(default)]
    pub description: String,
}

/// One row in the routing_table (user-facing input format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingEntry {
    pub interface: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub connected_to: String,
    #[serde(default)]
    pub has_processor: bool,
    #[serde(default)]
    pub target_subsystem: String,
    #[serde(default)]
    pub processor_name: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub protocol: String,
}

/// Interface spec produced by normalization (replaces routing_table).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceSpec {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub receives_from: Vec<String>,
    #[serde(default)]
    pub exports_to: Vec<String>,
    #[serde(default)]
    pub processor: Option<ProcessorSpec>,
}

/// Processor attached to an interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorSpec {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub target: String,
}

/// Level-1 (and parent of level-2) subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subsystem {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub complexity: Option<String>,
    #[serde(default)]
    pub adaptable: Option<bool>,
    #[serde(default)]
    pub evolveable: Option<bool>,
    #[serde(default)]
    pub archetype: Option<String>,
    /// For agent archetype.
    #[serde(default)]
    pub agent: Option<AgentSpec>,
    #[serde(default)]
    pub children: Vec<Subsystem>,
}

/// Agent model (only used when archetype == "Agent").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub agency_capacity: Option<f64>,
}

/// Flow that crosses the system boundary (source↔S0 or S0↔sink).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalFlow {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub interface: String,
    #[serde(default)]
    pub substance: Option<Substance>,
    #[serde(default)]
    pub usability: Option<String>,
    #[serde(default)]
    pub interaction_type: Option<String>,
    #[serde(default)]
    pub amount: Option<serde_json::Value>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub parameters: Vec<FlowParameter>,

    // Populated during normalization
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub sink: Option<String>,

    // Flattened substance fields (populated during normalization)
    #[serde(default)]
    pub substance_type: Option<String>,
    #[serde(default)]
    pub substance_subtype: Option<String>,
}

/// Flow between a processor and an internal subsystem.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessorFlow {
    #[serde(default)]
    pub processor_interface: String,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub substance: Option<Substance>,
    #[serde(default)]
    pub usability: Option<String>,
    #[serde(default)]
    pub interaction_type: Option<String>,
    #[serde(default)]
    pub amount: Option<serde_json::Value>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub parameters: Vec<FlowParameter>,

    // Flattened
    #[serde(default)]
    pub substance_type: Option<String>,
    #[serde(default)]
    pub substance_subtype: Option<String>,
}

/// Flow between two subsystems (internal to the system).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InternalFlow {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub sink: String,
    #[serde(default)]
    pub substance: Option<Substance>,
    #[serde(default)]
    pub usability: Option<String>,
    #[serde(default)]
    pub interaction_type: Option<String>,
    #[serde(default)]
    pub amount: Option<serde_json::Value>,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub parameters: Vec<FlowParameter>,

    // Flattened
    #[serde(default)]
    pub substance_type: Option<String>,
    #[serde(default)]
    pub substance_subtype: Option<String>,
}

/// Substance carried by a flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Substance {
    #[serde(rename = "type", default)]
    pub type_name: String,
    #[serde(default)]
    pub sub_type: String,
}

/// Named parameter attached to a flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowParameter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: serde_json::Value,
    #[serde(default)]
    pub unit: String,
}

// -- Valid enum values -------------------------------------------------------

const VALID_ARCHETYPES: &[&str] = &["Agent", "Economy", "Governance"];
const VALID_AGENT_KINDS: &[&str] = &["Reactive", "Anticipatory", "Intentional"];
const VALID_COMPLEXITIES: &[&str] = &["Complex", "Atomic"];
const VALID_SUBSTANCE_TYPES: &[&str] = &["Energy", "Material", "Message"];
const VALID_USABILITIES: &[&str] = &["Resource", "Product", "Waste", "Disruption"];
const VALID_INTERFACE_TYPES: &[&str] = &["Import", "Export"];
const VALID_INTERACTION_TYPES: &[&str] = &["Flow", "Force"];

// -- Validator ---------------------------------------------------------------

/// Validate an intermediate format specification.
///
/// Returns a list of error messages. Empty list means valid.
pub fn validate_intermediate(spec: &IntermediateSpec) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();

    // -- system ---------------------------------------------------------------
    // Accept name from either spec.system.name or spec.name (flat form).
    let system_name: &str = spec
        .system
        .as_ref()
        .and_then(|s| s.name.as_deref())
        .or(spec.name.as_deref())
        .unwrap_or("");

    if system_name.trim().is_empty() {
        errors.push("system.name is required and must be a non-empty string".to_string());
    }

    if let Some(sys) = &spec.system {
        if let Some(complexity) = &sys.complexity {
            if !VALID_COMPLEXITIES.contains(&complexity.as_str()) {
                errors.push(format!(
                    "system.complexity must be one of {:?}, got '{complexity}'",
                    sorted(VALID_COMPLEXITIES)
                ));
            }
        }
    }

    // -- sources --------------------------------------------------------------
    if spec.sources.is_empty() {
        errors.push("'sources' must have at least 1 entry".to_string());
    } else {
        let mut source_names: Vec<&str> = Vec::new();
        for (i, src) in spec.sources.iter().enumerate() {
            if src.name.trim().is_empty() {
                errors.push(format!(
                    "sources[{i}].name is required and must be non-empty"
                ));
            } else {
                source_names.push(&src.name);
            }
        }
        for dupe in find_duplicates(&source_names) {
            errors.push(format!("Duplicate source names: {dupe:?}"));
        }
    }

    // -- sinks ----------------------------------------------------------------
    if spec.sinks.is_empty() {
        errors.push("'sinks' must have at least 1 entry".to_string());
    } else {
        let mut sink_names: Vec<&str> = Vec::new();
        for (i, snk) in spec.sinks.iter().enumerate() {
            if snk.name.trim().is_empty() {
                errors.push(format!(
                    "sinks[{i}].name is required and must be non-empty"
                ));
            } else {
                sink_names.push(&snk.name);
            }
        }
        for dupe in find_duplicates(&sink_names) {
            errors.push(format!("Duplicate sink names: {dupe:?}"));
        }
    }

    // -- subsystems -----------------------------------------------------------
    if spec.subsystems.is_empty() {
        errors.push("'subsystems' must have at least 1 entry".to_string());
    } else {
        let mut subsystem_names: Vec<&str> = Vec::new();

        for (i, sub) in spec.subsystems.iter().enumerate() {
            validate_subsystem(sub, i, None, &mut subsystem_names, &mut errors);
        }

        for dupe in find_duplicates(&subsystem_names) {
            errors.push(format!("Duplicate subsystem names: {dupe:?}"));
        }
    }

    // Build name sets for reference validation.
    let source_name_set: std::collections::HashSet<&str> =
        spec.sources.iter().map(|s| s.name.as_str()).collect();
    let sink_name_set: std::collections::HashSet<&str> =
        spec.sinks.iter().map(|s| s.name.as_str()).collect();

    // -- routing_table --------------------------------------------------------
    // Also accept the normalized `interfaces` list; validate whichever is present.
    let mut interface_names: std::collections::HashSet<String> =
        std::collections::HashSet::new();
    let mut processor_names: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for (i, rt) in spec.routing_table.iter().enumerate() {
        if rt.interface.trim().is_empty() {
            errors.push(format!(
                "routing_table[{i}].interface is required and must be a non-empty string"
            ));
        } else {
            interface_names.insert(rt.interface.clone());
        }

        if !VALID_INTERFACE_TYPES.contains(&rt.type_name.as_str()) {
            errors.push(format!(
                "routing_table[{i}].type must be one of {:?}, got '{}'",
                sorted(VALID_INTERFACE_TYPES),
                rt.type_name
            ));
        }

        if rt.connected_to.trim().is_empty() {
            errors.push(format!("routing_table[{i}].connected_to is required"));
        } else {
            if rt.type_name == "Import" && !source_name_set.contains(rt.connected_to.as_str()) {
                errors.push(format!(
                    "routing_table[{i}].connected_to '{}' must reference an existing source name for Import interfaces",
                    rt.connected_to
                ));
            } else if rt.type_name == "Export"
                && !sink_name_set.contains(rt.connected_to.as_str())
            {
                errors.push(format!(
                    "routing_table[{i}].connected_to '{}' must reference an existing sink name for Export interfaces",
                    rt.connected_to
                ));
            }
        }

        if rt.has_processor {
            if rt.target_subsystem.trim().is_empty() {
                errors.push(format!(
                    "routing_table[{i}] has has_processor=true but no target_subsystem"
                ));
            } else {
                processor_names.insert(rt.target_subsystem.clone());
                if !rt.interface.trim().is_empty() {
                    processor_names.insert(rt.interface.clone());
                }
                if let Some(proc_name) = &rt.processor_name {
                    processor_names.insert(proc_name.clone());
                }
            }
        }
    }

    // Also register interface names from the normalized `interfaces` list.
    for iface in &spec.interfaces {
        if !iface.name.trim().is_empty() {
            interface_names.insert(iface.name.clone());
        }
        if let Some(proc) = &iface.processor {
            if !proc.name.trim().is_empty() {
                processor_names.insert(proc.name.clone());
            }
            if !proc.target.trim().is_empty() {
                processor_names.insert(proc.target.clone());
            }
            processor_names.insert(iface.name.clone());
        }
    }

    // -- external_flows -------------------------------------------------------
    for (i, ef) in spec.external_flows.iter().enumerate() {
        if ef.name.trim().is_empty() {
            errors.push(format!(
                "external_flows[{i}].name is required and must be non-empty"
            ));
        }

        if !ef.interface.trim().is_empty() && !interface_names.contains(&ef.interface) {
            errors.push(format!(
                "external_flows[{i}].interface '{}' must reference an interface defined in routing_table",
                ef.interface
            ));
        }

        if let Some(substance) = &ef.substance {
            if !substance.type_name.trim().is_empty()
                && !VALID_SUBSTANCE_TYPES.contains(&substance.type_name.as_str())
            {
                errors.push(format!(
                    "external_flows[{i}].substance.type must be one of {:?}, got '{}'",
                    sorted(VALID_SUBSTANCE_TYPES),
                    substance.type_name
                ));
            }
        }

        if let Some(usability) = &ef.usability {
            if !VALID_USABILITIES.contains(&usability.as_str()) {
                errors.push(format!(
                    "external_flows[{i}].usability must be one of {:?}, got '{usability}'",
                    sorted(VALID_USABILITIES)
                ));
            }
        }
    }

    // -- internal_flows -------------------------------------------------------
    // Build full ref set: subsystem names + "Parent/Child" refs + processor names.
    let mut all_refs: std::collections::HashSet<String> = std::collections::HashSet::new();
    collect_subsystem_refs(&spec.subsystems, None, &mut all_refs);
    all_refs.extend(processor_names);

    for (i, ifl) in spec.internal_flows.iter().enumerate() {
        if ifl.name.trim().is_empty() {
            errors.push(format!(
                "internal_flows[{i}].name is required and must be non-empty"
            ));
        }

        if ifl.source.trim().is_empty() {
            errors.push(format!("internal_flows[{i}].source is required"));
        } else if !all_refs.contains(&ifl.source) {
            errors.push(format!(
                "internal_flows[{i}].source '{}' must reference an existing subsystem name",
                ifl.source
            ));
        }

        if ifl.sink.trim().is_empty() {
            errors.push(format!("internal_flows[{i}].sink is required"));
        } else if !all_refs.contains(&ifl.sink) {
            errors.push(format!(
                "internal_flows[{i}].sink '{}' must reference an existing subsystem name",
                ifl.sink
            ));
        }

        if let Some(substance) = &ifl.substance {
            if !substance.type_name.trim().is_empty()
                && !VALID_SUBSTANCE_TYPES.contains(&substance.type_name.as_str())
            {
                errors.push(format!(
                    "internal_flows[{i}].substance.type must be one of {:?}, got '{}'",
                    sorted(VALID_SUBSTANCE_TYPES),
                    substance.type_name
                ));
            }
        }

        if let Some(usability) = &ifl.usability {
            if !VALID_USABILITIES.contains(&usability.as_str()) {
                errors.push(format!(
                    "internal_flows[{i}].usability must be one of {:?}, got '{usability}'",
                    sorted(VALID_USABILITIES)
                ));
            }
        }

        if let Some(interaction_type) = &ifl.interaction_type {
            if !VALID_INTERACTION_TYPES.contains(&interaction_type.as_str()) {
                errors.push(format!(
                    "internal_flows[{i}].interaction_type must be one of {:?}, got '{interaction_type}'",
                    sorted(VALID_INTERACTION_TYPES)
                ));
            }
        }
    }

    errors
}

// -- Internal helpers --------------------------------------------------------

/// Validate a single subsystem (and its children recursively).
/// `parent_name` is `Some(name)` when validating a child subsystem.
/// `level1_names` accumulates top-level subsystem names for duplicate checking.
fn validate_subsystem<'a>(
    sub: &'a Subsystem,
    idx: usize,
    parent_name: Option<&str>,
    level1_names: &mut Vec<&'a str>,
    errors: &mut Vec<String>,
) {
    let prefix = match parent_name {
        Some(p) => format!("subsystems (child of '{p}')[{idx}]"),
        None => format!("subsystems[{idx}]"),
    };

    if sub.name.trim().is_empty() {
        errors.push(format!("{prefix}.name is required and must be non-empty"));
    } else if parent_name.is_none() {
        // Only track top-level names for duplicate detection here;
        // child duplicates are checked per-parent below.
        level1_names.push(&sub.name);
    }

    if let Some(arch) = &sub.archetype {
        if !VALID_ARCHETYPES.contains(&arch.as_str()) {
            errors.push(format!(
                "{prefix}.archetype must be one of {:?}, got '{arch}'",
                sorted(VALID_ARCHETYPES)
            ));
        }
    }

    // agent_kind lives under sub.agent.kind in the existing struct.
    if let Some(agent) = &sub.agent {
        if let Some(kind) = &agent.kind {
            if !VALID_AGENT_KINDS.contains(&kind.as_str()) {
                errors.push(format!(
                    "{prefix}.agent.kind must be one of {:?}, got '{kind}'",
                    sorted(VALID_AGENT_KINDS)
                ));
            }
        }
    }

    if let Some(complexity) = &sub.complexity {
        if !VALID_COMPLEXITIES.contains(&complexity.as_str()) {
            errors.push(format!(
                "{prefix}.complexity must be one of {:?}, got '{complexity}'",
                sorted(VALID_COMPLEXITIES)
            ));
        }
    }

    // Children
    if !sub.children.is_empty() {
        let mut child_names: Vec<&str> = Vec::new();
        for (j, child) in sub.children.iter().enumerate() {
            let child_prefix = format!("subsystems[{idx}].children[{j}]");

            if child.name.trim().is_empty() {
                errors.push(format!(
                    "{child_prefix}.name is required and must be non-empty"
                ));
            } else {
                child_names.push(&child.name);
            }

            if let Some(arch) = &child.archetype {
                if !VALID_ARCHETYPES.contains(&arch.as_str()) {
                    errors.push(format!(
                        "{child_prefix}.archetype must be one of {:?}, got '{arch}'",
                        sorted(VALID_ARCHETYPES)
                    ));
                }
            }

            if let Some(agent) = &child.agent {
                if let Some(kind) = &agent.kind {
                    if !VALID_AGENT_KINDS.contains(&kind.as_str()) {
                        errors.push(format!(
                            "{child_prefix}.agent.kind must be one of {:?}, got '{kind}'",
                            sorted(VALID_AGENT_KINDS)
                        ));
                    }
                }
            }

            if let Some(complexity) = &child.complexity {
                if !VALID_COMPLEXITIES.contains(&complexity.as_str()) {
                    errors.push(format!(
                        "{child_prefix}.complexity must be one of {:?}, got '{complexity}'",
                        sorted(VALID_COMPLEXITIES)
                    ));
                }
            }
        }

        for dupe in find_duplicates(&child_names) {
            errors.push(format!(
                "Duplicate child names in subsystems[{idx}] ('{}'): {dupe:?}",
                sub.name
            ));
        }
    }
}

/// Recursively populate `refs` with subsystem names and "Parent/Child" paths.
fn collect_subsystem_refs(
    subsystems: &[Subsystem],
    parent_name: Option<&str>,
    refs: &mut std::collections::HashSet<String>,
) {
    for sub in subsystems {
        if sub.name.trim().is_empty() {
            continue;
        }
        let full_name = match parent_name {
            Some(p) => format!("{p}/{}", sub.name),
            None => sub.name.clone(),
        };
        refs.insert(full_name.clone());
        if !sub.children.is_empty() {
            collect_subsystem_refs(&sub.children, Some(&full_name), refs);
        }
    }
}

/// Return names that appear more than once. Preserves first-seen order of dupes.
fn find_duplicates<'a>(names: &[&'a str]) -> Vec<&'a str> {
    let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
    let mut dupes: Vec<&str> = Vec::new();
    for &n in names {
        if !seen.insert(n) && !dupes.contains(&n) {
            dupes.push(n);
        }
    }
    dupes
}

/// Return a sorted copy of a `&[&str]` for use in error messages.
fn sorted<'a>(values: &'a [&'a str]) -> Vec<&'a str> {
    let mut v = values.to_vec();
    v.sort_unstable();
    v
}

// -- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn parse(v: serde_json::Value) -> IntermediateSpec {
        serde_json::from_value(v).expect("failed to parse IntermediateSpec")
    }

    fn minimal() -> serde_json::Value {
        json!({
            "system": { "name": "Test System" },
            "sources": [{ "name": "Environment" }],
            "sinks": [{ "name": "Waste Sink" }],
            "subsystems": [{ "name": "Core Processor" }],
            "routing_table": [
                {
                    "interface": "Input Port",
                    "type": "Import",
                    "connected_to": "Environment"
                }
            ],
            "external_flows": [
                {
                    "name": "Energy In",
                    "interface": "Input Port"
                }
            ],
            "internal_flows": []
        })
    }

    #[test]
    fn valid_minimal_spec_passes() {
        let spec = parse(minimal());
        let errors = validate_intermediate(&spec);
        assert!(errors.is_empty(), "unexpected errors: {errors:?}");
    }

    #[test]
    fn empty_system_name_is_error() {
        let mut v = minimal();
        v["system"]["name"] = json!("");
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("system.name")),
            "expected system.name error, got: {errors:?}"
        );
    }

    #[test]
    fn missing_sources_is_error() {
        let mut v = minimal();
        v["sources"] = json!([]);
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("sources")),
            "expected sources error, got: {errors:?}"
        );
    }

    #[test]
    fn duplicate_source_names_detected() {
        let mut v = minimal();
        v["sources"] = json!([{ "name": "Env" }, { "name": "Env" }]);
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("Duplicate source")),
            "expected duplicate source error, got: {errors:?}"
        );
    }

    #[test]
    fn invalid_archetype_is_error() {
        let mut v = minimal();
        v["subsystems"][0]["archetype"] = json!("Robot");
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("archetype")),
            "expected archetype error, got: {errors:?}"
        );
    }

    #[test]
    fn invalid_substance_type_is_error() {
        let mut v = minimal();
        v["external_flows"][0]["substance"] = json!({ "type": "Magic" });
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("substance.type")),
            "expected substance.type error, got: {errors:?}"
        );
    }

    #[test]
    fn internal_flow_bad_source_reference_is_error() {
        let mut v = minimal();
        v["internal_flows"] = json!([{
            "name": "Signal",
            "source": "NonExistent",
            "sink": "Core Processor"
        }]);
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors
                .iter()
                .any(|e| e.contains("source") && e.contains("NonExistent")),
            "expected bad source ref error, got: {errors:?}"
        );
    }

    #[test]
    fn routing_connected_to_wrong_direction_is_error() {
        let mut v = minimal();
        // Export route but points to a source name, not a sink name.
        v["routing_table"] = json!([{
            "interface": "Out Port",
            "type": "Export",
            "connected_to": "Environment"
        }]);
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors
                .iter()
                .any(|e| e.contains("connected_to") && e.contains("Export")),
            "expected Export/sink mismatch error, got: {errors:?}"
        );
    }

    #[test]
    fn has_processor_without_target_is_error() {
        let mut v = minimal();
        v["routing_table"][0]["has_processor"] = json!(true);
        // no target_subsystem field — serde default is empty string
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("has_processor")),
            "expected has_processor error, got: {errors:?}"
        );
    }

    #[test]
    fn child_parent_slash_ref_valid_in_internal_flow() {
        let mut v = minimal();
        v["subsystems"][0]["children"] = json!([{ "name": "Sub-Unit" }]);
        v["internal_flows"] = json!([{
            "name": "Signal",
            "source": "Core Processor/Sub-Unit",
            "sink": "Core Processor"
        }]);
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(errors.is_empty(), "unexpected errors: {errors:?}");
    }

    #[test]
    fn invalid_agent_kind_is_error() {
        let mut v = minimal();
        v["subsystems"][0]["agent"] = json!({ "kind": "Omniscient" });
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("agent.kind")),
            "expected agent.kind error, got: {errors:?}"
        );
    }

    #[test]
    fn invalid_interaction_type_is_error() {
        let mut v = minimal();
        v["internal_flows"] = json!([{
            "name": "Signal",
            "source": "Core Processor",
            "sink": "Core Processor",
            "interaction_type": "Telekinesis"
        }]);
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("interaction_type")),
            "expected interaction_type error, got: {errors:?}"
        );
    }

    #[test]
    fn duplicate_subsystem_names_detected() {
        let mut v = minimal();
        v["subsystems"] = json!([
            { "name": "Alpha" },
            { "name": "Alpha" }
        ]);
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(
            errors.iter().any(|e| e.contains("Duplicate subsystem")),
            "expected duplicate subsystem error, got: {errors:?}"
        );
    }

    #[test]
    fn flat_name_accepted_when_no_system_block() {
        // Some LLM outputs omit the nested "system" key and put name at top level.
        let v = json!({
            "name": "Flat System",
            "sources": [{ "name": "Src" }],
            "sinks": [{ "name": "Snk" }],
            "subsystems": [{ "name": "Sub" }]
        });
        let spec = parse(v);
        let errors = validate_intermediate(&spec);
        assert!(errors.is_empty(), "unexpected errors: {errors:?}");
    }
}
