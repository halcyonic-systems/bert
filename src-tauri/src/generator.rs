/// Deterministic BERT JSON Model Generator.
///
/// Takes a validated intermediate format (as `serde_json::Value`) and compiles
/// it into a complete, valid BERT JSON WorldModel. No LLM involved — pure
/// deterministic computation.
///
/// Part of the bert-rag deterministic pipeline:
///   intermediate spec (LLM) -> generator.rs (this) -> BERT JSON WorldModel
use std::collections::HashMap;
use std::f64::consts::PI;

use serde_json::{json, Value};

// The intermediate types are defined in the sibling module. The generator
// receives the spec as a raw serde_json::Value (already deserialized by the
// caller) so it can interoperate with both the Tauri command layer and the
// Python-compatible JSON format directly.
#[allow(unused_imports)]
use super::intermediate::{
    ExternalFlow, InterfaceSpec, InternalFlow, ProcessorFlow, ProcessorSpec, Subsystem,
};

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Compiles a validated intermediate spec (as a raw `serde_json::Value`) into
/// a complete BERT JSON WorldModel.
///
/// The input Value is expected to match the intermediate format documented in
/// `intermediate.rs`. After calling this function the returned Value is ready
/// to be serialized with `serde_json::to_string`.
pub struct BertModelGenerator {
    /// Working copy of the spec — mutated during normalization.
    spec: Value,

    systems: Vec<Value>,
    interactions: Vec<Value>,
    environment: Option<Value>,

    // Lookup tables populated during assign_ids
    subsystem_ids: HashMap<String, String>,
    subsystem_positions: HashMap<String, (f64, f64)>,
    source_ids: HashMap<String, String>,
    sink_ids: HashMap<String, String>,
    interface_ids: HashMap<String, String>,
    processor_ids: HashMap<String, String>,
    interface_by_name: HashMap<String, Value>,

    // processor_name → interface name  (e.g. "Entry Point" → "User Entry")
    processor_name_to_iface: HashMap<String, String>,

    // Sequential counters
    bare_interface_counter: u32,
    processor_interface_counter: u32,
    external_flow_counter: u32,
    internal_flow_counter: u32,
}

impl BertModelGenerator {
    /// Create a generator from a raw intermediate spec Value.
    pub fn new(spec: Value) -> Self {
        Self {
            spec,
            systems: Vec::new(),
            interactions: Vec::new(),
            environment: None,
            subsystem_ids: HashMap::new(),
            subsystem_positions: HashMap::new(),
            source_ids: HashMap::new(),
            sink_ids: HashMap::new(),
            interface_ids: HashMap::new(),
            processor_ids: HashMap::new(),
            interface_by_name: HashMap::new(),
            processor_name_to_iface: HashMap::new(),
            bare_interface_counter: 0,
            processor_interface_counter: 50,
            external_flow_counter: 0,
            internal_flow_counter: 0,
        }
    }

    /// Generate a complete BERT WorldModel from the intermediate spec.
    pub fn generate(&mut self) -> Value {
        self.normalize_spec();
        self.assign_ids();
        self.build_environment();
        self.build_root_system();
        self.build_subsystems();
        self.build_interface_processors();
        self.build_external_flows();
        self.build_processor_flows();
        self.build_internal_flows();
        self.assemble()
    }

    // -----------------------------------------------------------------------
    // Phase 0a: Normalization
    // -----------------------------------------------------------------------

    /// Normalize the intermediate format into the internal format the generator
    /// expects.
    ///
    /// Bridges the user-facing intermediate format (routing_table, system.name,
    /// etc.) to the internal format (interfaces, top-level name, processor
    /// dicts, etc.).
    fn normalize_spec(&mut self) {
        // 1. Flatten system.name/description to top level
        if let Some(system_obj) = self.spec.get("system").cloned() {
            if system_obj.is_object() {
                if self.spec.get("name").is_none() {
                    let name = system_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("System")
                        .to_string();
                    self.spec["name"] = json!(name);
                }
                if self.spec.get("description").is_none() {
                    let desc = system_obj
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    self.spec["description"] = json!(desc);
                }
            }
        }

        // 2. Convert routing_table → interfaces + external_flows + processor_flows
        let has_routing = self.spec.get("routing_table").is_some();
        let has_interfaces = self.spec.get("interfaces").is_some();

        if has_routing && !has_interfaces {
            let routing_table: Vec<Value> = self
                .spec
                .get("routing_table")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            let mut ext_flows: Vec<Value> = self
                .spec
                .get("external_flows")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            let mut proc_flows: Vec<Value> = self
                .spec
                .get("processor_flows")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            // Build lookup: interface name → external flows for that interface
            let mut ext_flow_by_iface: HashMap<String, Vec<usize>> = HashMap::new();
            for (idx, ef) in ext_flows.iter().enumerate() {
                let iface_name = ef.get("interface").and_then(|v| v.as_str()).unwrap_or("");
                ext_flow_by_iface
                    .entry(iface_name.to_string())
                    .or_default()
                    .push(idx);
            }

            let int_flows: Vec<Value> = self
                .spec
                .get("internal_flows")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            let mut interfaces: Vec<Value> = Vec::new();

            for rt in &routing_table {
                let iface_name = rt
                    .get("interface")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let iface_type = rt
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Import")
                    .to_string();
                let connected_to = rt
                    .get("connected_to")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let has_processor = rt
                    .get("has_processor")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let target_sub = rt
                    .get("target_subsystem")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let processor_name = rt
                    .get("processor_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&iface_name)
                    .to_string();
                let description = rt
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let protocol = rt
                    .get("protocol")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                // Build receives_from / exports_to from connected_to
                let (receives_from, exports_to) = if iface_type == "Import" {
                    (
                        if connected_to.is_empty() {
                            vec![]
                        } else {
                            vec![connected_to.clone()]
                        },
                        vec![],
                    )
                } else {
                    (
                        vec![],
                        if connected_to.is_empty() {
                            vec![]
                        } else {
                            vec![connected_to.clone()]
                        },
                    )
                };

                let mut iface_dict = json!({
                    "name": iface_name,
                    "description": description,
                    "protocol": protocol,
                    "type": iface_type,
                    "receives_from": receives_from,
                    "exports_to": exports_to,
                });

                if has_processor {
                    iface_dict["processor"] = json!({
                        "name": processor_name,
                        "description": "",
                        "target": target_sub,
                    });

                    // Auto-generate processor flow if not already covered by
                    // processor_flows OR internal_flows.
                    let existing_pf = proc_flows.iter().any(|pf| {
                        pf.get("processor_interface")
                            .and_then(|v| v.as_str())
                            == Some(&iface_name)
                    });

                    let existing_if = int_flows.iter().any(|ifl| {
                        let src = ifl.get("source").and_then(|v| v.as_str()).unwrap_or("");
                        let snk = ifl.get("sink").and_then(|v| v.as_str()).unwrap_or("");
                        src == iface_name
                            || src == processor_name
                            || snk == iface_name
                            || snk == processor_name
                    });

                    if !existing_pf && !existing_if {
                        // Find the external flow for this interface to get substance info
                        let substance = ext_flow_by_iface
                            .get(&iface_name)
                            .and_then(|idxs| idxs.first())
                            .and_then(|&i| ext_flows.get(i))
                            .and_then(|ef| ef.get("substance"))
                            .cloned()
                            .unwrap_or_else(|| json!({"type": "Message", "sub_type": ""}));

                        proc_flows.push(json!({
                            "processor_interface": iface_name,
                            "target": target_sub,
                            "name": format!("{iface_name} routing"),
                            "description": "",
                            "substance": substance,
                            "usability": "Resource",
                            "interaction_type": "Flow",
                        }));
                    }
                }

                interfaces.push(iface_dict);
            }

            // Annotate external flows with direction based on routing_table type
            let rt_types: HashMap<String, String> = routing_table
                .iter()
                .filter_map(|rt| {
                    let iface = rt.get("interface")?.as_str()?.to_string();
                    let ty = rt.get("type")?.as_str()?.to_string();
                    Some((iface, ty))
                })
                .collect();

            let rt_connected: HashMap<String, String> = routing_table
                .iter()
                .filter_map(|rt| {
                    let iface = rt.get("interface")?.as_str()?.to_string();
                    let conn = rt
                        .get("connected_to")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    Some((iface, conn))
                })
                .collect();

            for ef in ext_flows.iter_mut() {
                let iface_name = ef
                    .get("interface")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                if let Some(ty) = rt_types.get(&iface_name) {
                    if ty == "Import" {
                        ef["direction"] = json!("in");
                        if ef.get("source").is_none() || ef["source"].is_null() {
                            if let Some(conn) = rt_connected.get(&iface_name) {
                                ef["source"] = json!(conn);
                            }
                        }
                    } else {
                        ef["direction"] = json!("out");
                        if ef.get("sink").is_none() || ef["sink"].is_null() {
                            if let Some(conn) = rt_connected.get(&iface_name) {
                                ef["sink"] = json!(conn);
                            }
                        }
                    }
                }
            }

            self.spec["interfaces"] = json!(interfaces);
            self.spec["processor_flows"] = json!(proc_flows);
            self.spec["external_flows"] = json!(ext_flows);
        }

        // 3. Build processor_name → interface name mapping
        if let Some(rt_arr) = self.spec.get("routing_table").and_then(|v| v.as_array()) {
            for rt in rt_arr {
                let has_proc = rt
                    .get("has_processor")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                if has_proc {
                    let iface_name = rt
                        .get("interface")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let proc_name = rt
                        .get("processor_name")
                        .and_then(|v| v.as_str())
                        .unwrap_or(&iface_name)
                        .to_string();
                    self.processor_name_to_iface
                        .insert(proc_name, iface_name);
                }
            }
        }

        // 4. Normalize substance format in all flow lists
        for flow_list_key in &["external_flows", "internal_flows", "processor_flows"] {
            if let Some(flows) = self.spec.get_mut(flow_list_key).and_then(|v| v.as_array_mut()) {
                for flow in flows.iter_mut() {
                    // Flatten substance dict → substance_type / substance_subtype
                    if let Some(subst) = flow.get("substance").cloned() {
                        if subst.is_object() {
                            let stype = subst
                                .get("type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Message")
                                .to_string();
                            let ssubtype = subst
                                .get("sub_type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            flow["substance_type"] = json!(stype);
                            flow["substance_subtype"] = json!(ssubtype);
                        }
                    }
                    // Default interaction_type
                    if flow.get("interaction_type").is_none() {
                        flow["interaction_type"] = json!("Flow");
                    }
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Phase 0b: ID and position assignment
    // -----------------------------------------------------------------------

    fn assign_ids(&mut self) {
        // Sources
        let sources: Vec<Value> = self
            .spec
            .get("sources")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for (i, src) in sources.iter().enumerate() {
            if let Some(name) = src.get("name").and_then(|v| v.as_str()) {
                self.source_ids
                    .insert(name.to_string(), format!("Src-1.{i}"));
            }
        }

        // Sinks
        let sinks: Vec<Value> = self
            .spec
            .get("sinks")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for (i, snk) in sinks.iter().enumerate() {
            if let Some(name) = snk.get("name").and_then(|v| v.as_str()) {
                self.sink_ids
                    .insert(name.to_string(), format!("Snk-1.{i}"));
            }
        }

        // Interfaces — partition into bare vs processor-equipped
        let interfaces: Vec<Value> = self
            .spec
            .get("interfaces")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for iface in &interfaces {
            let has_processor = iface
                .get("processor")
                .map(|v| !v.is_null())
                .unwrap_or(false);
            let name = iface
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            if has_processor {
                let idx = self.processor_interface_counter;
                self.processor_interface_counter += 1;
                let iface_id = format!("I0.{idx}");
                self.interface_ids.insert(name.clone(), iface_id);
                self.processor_ids
                    .insert(name.clone(), format!("C0.{idx}"));
            } else {
                let idx = self.bare_interface_counter;
                self.bare_interface_counter += 1;
                let iface_id = format!("I0.{idx}");
                self.interface_ids.insert(name.clone(), iface_id);
            }

            self.interface_by_name.insert(name, iface.clone());
        }

        // Level-1 subsystems (non-processor)
        let subsystems: Vec<Value> = self
            .spec
            .get("subsystems")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let level1_positions = compute_level1_positions(subsystems.len());

        for (i, sub) in subsystems.iter().enumerate() {
            let name = sub
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let sub_id = format!("C0.{i}");
            self.subsystem_ids.insert(name.clone(), sub_id);
            self.subsystem_positions.insert(name.clone(), level1_positions[i]);

            // Level-2 children
            let children: Vec<Value> = sub
                .get("children")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            if !children.is_empty() {
                let child_positions =
                    compute_level2_positions(children.len(), level1_positions[i]);
                for (j, child) in children.iter().enumerate() {
                    let child_name = child
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let child_id = format!("C0.{i}.{j}");
                    self.subsystem_ids
                        .insert(child_name.clone(), child_id);
                    self.subsystem_positions
                        .insert(child_name, child_positions[j]);
                }
            }
        }

        // Processor positions: approximate boundary position from interface angle
        let ifaces_for_processors: Vec<Value> = self
            .spec
            .get("interfaces")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        for iface in &ifaces_for_processors {
            if iface.get("processor").map(|v| !v.is_null()).unwrap_or(false) {
                let iface_name = iface
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let angle = self.get_interface_angle(&iface_name);
                let (bx, by) = match angle {
                    Some(a) => (300.0 * a.cos(), 300.0 * a.sin()),
                    None => (-300.0, 0.0),
                };
                self.subsystem_positions.insert(iface_name, (bx, by));
            }
        }
    }

    // -----------------------------------------------------------------------
    // Phase 1: Build environment
    // -----------------------------------------------------------------------

    fn build_environment(&mut self) {
        let sources: Vec<Value> = self
            .spec
            .get("sources")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let sinks: Vec<Value> = self
            .spec
            .get("sinks")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let source_positions = compute_source_positions(sources.len());
        let sink_positions = compute_sink_positions(sinks.len());

        let env_sources: Vec<Value> = sources
            .iter()
            .enumerate()
            .map(|(i, src)| {
                let name = src
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let desc = src
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let pos = source_positions[i];
                json!({
                    "info": {
                        "id": format!("Src-1.{i}"),
                        "level": -1,
                        "name": name,
                        "description": desc,
                    },
                    "type": "Source",
                    "transform": {
                        "translation": [pos.0, pos.1],
                        "rotation": -PI,
                    },
                    "equivalence": "",
                    "model": "",
                    "is_same_as_id": null,
                })
            })
            .collect();

        let env_sinks: Vec<Value> = sinks
            .iter()
            .enumerate()
            .map(|(i, snk)| {
                let name = snk
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let desc = snk
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let pos = sink_positions[i];
                json!({
                    "info": {
                        "id": format!("Snk-1.{i}"),
                        "level": -1,
                        "name": name,
                        "description": desc,
                    },
                    "type": "Sink",
                    "transform": {
                        "translation": [pos.0, pos.1],
                        "rotation": 0.0_f64,
                    },
                    "equivalence": "",
                    "model": "",
                    "is_same_as_id": null,
                })
            })
            .collect();

        let env_name = self
            .spec
            .get("environment_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let env_desc = self
            .spec
            .get("environment_description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        self.environment = Some(json!({
            "info": {
                "id": "E-1",
                "level": -1,
                "name": env_name,
                "description": env_desc,
            },
            "sources": env_sources,
            "sinks": env_sinks,
        }));
    }

    // -----------------------------------------------------------------------
    // Phase 2: Build root system S0
    // -----------------------------------------------------------------------

    fn build_root_system(&mut self) {
        let interfaces = self.build_interfaces();

        let sys_name = self
            .spec
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("System")
            .to_string();
        let sys_desc = self
            .spec
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let s0 = json!({
            "info": {
                "id": "S0",
                "level": 0,
                "name": sys_name,
                "description": sys_desc,
            },
            "sources": [],
            "sinks": [],
            "parent": "E-1",
            "complexity": { "Complex": { "adaptable": true, "evolveable": true } },
            "boundary": {
                "info": {
                    "id": "B0",
                    "level": 0,
                    "name": "",
                    "description": "",
                },
                "porosity": 0.0_f64,
                "perceptive_fuzziness": 0.0_f64,
                "interfaces": interfaces,
                "parent_interface": null,
            },
            "radius": 300.0_f64,
            "transform": {
                "translation": [0.0_f64, 0.0_f64],
                "rotation": 0.0_f64,
            },
            "equivalence": "",
            "history": "",
            "transformation": "",
            "member_autonomy": 1.0_f64,
            "time_constant": "Second",
        });

        self.systems.push(s0);
    }

    /// Build the interface list for S0's boundary.
    fn build_interfaces(&self) -> Vec<Value> {
        let iface_specs: Vec<Value> = self
            .spec
            .get("interfaces")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let imports: Vec<&Value> = iface_specs
            .iter()
            .filter(|i| i.get("type").and_then(|v| v.as_str()) == Some("Import"))
            .collect();
        let exports: Vec<&Value> = iface_specs
            .iter()
            .filter(|i| i.get("type").and_then(|v| v.as_str()) == Some("Export"))
            .collect();

        let import_angles = compute_interface_angles(imports.len(), "Import");
        let export_angles = compute_interface_angles(exports.len(), "Export");

        let mut import_idx = 0usize;
        let mut export_idx = 0usize;

        let mut interfaces = Vec::new();

        for iface in &iface_specs {
            let name = iface
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let iface_id = self
                .interface_ids
                .get(&name)
                .cloned()
                .unwrap_or_else(|| name.clone());
            let iface_type = iface
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("Import");
            let description = iface
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let protocol = iface
                .get("protocol")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let (angle, receives_from, exports_to) = if iface_type == "Import" {
                let a = import_angles
                    .get(import_idx)
                    .copied()
                    .unwrap_or(PI);
                import_idx += 1;

                let rf: Vec<Value> = iface
                    .get("receives_from")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .filter_map(|src_name| {
                                self.source_ids.get(src_name).map(|id| json!(id))
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                (a, rf, vec![])
            } else {
                let a = export_angles
                    .get(export_idx)
                    .copied()
                    .unwrap_or(0.0);
                export_idx += 1;

                let et: Vec<Value> = iface
                    .get("exports_to")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .filter_map(|snk_name| {
                                self.sink_ids.get(snk_name).map(|id| json!(id))
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                (a, vec![], et)
            };

            interfaces.push(json!({
                "info": {
                    "id": iface_id,
                    "level": 1,
                    "name": name,
                    "description": description,
                },
                "protocol": protocol,
                "type": iface_type,
                "exports_to": exports_to,
                "receives_from": receives_from,
                "angle": angle,
            }));
        }

        interfaces
    }

    // -----------------------------------------------------------------------
    // Phase 3: Build subsystems (level-1 and level-2)
    // -----------------------------------------------------------------------

    fn build_subsystems(&mut self) {
        let subsystems: Vec<Value> = self
            .spec
            .get("subsystems")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        for (i, sub) in subsystems.iter().enumerate() {
            let sub_id = format!("C0.{i}");
            let name = sub
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let desc = sub
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let pos = self
                .subsystem_positions
                .get(&name)
                .copied()
                .unwrap_or((0.0, 0.0));

            let complexity = sub
                .get("complexity")
                .and_then(|v| v.as_str())
                .unwrap_or("Complex")
                .to_string();
            let adaptable = sub
                .get("adaptable")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let evolveable = sub
                .get("evolveable")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let archetype = sub.get("archetype").and_then(|v| v.as_str()).map(String::from);
            let agent = sub.get("agent").cloned();

            let system = make_system(
                &sub_id,
                1,
                &name,
                &desc,
                "S0",
                42.0,
                pos,
                &complexity,
                adaptable,
                evolveable,
                archetype.as_deref(),
                agent.as_ref(),
                None,
            );
            self.systems.push(system);

            // Level-2 children
            let children: Vec<Value> = sub
                .get("children")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            for (j, child) in children.iter().enumerate() {
                let child_id = format!("C0.{i}.{j}");
                let child_name = child
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let child_desc = child
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let child_pos = self
                    .subsystem_positions
                    .get(&child_name)
                    .copied()
                    .unwrap_or((0.0, 0.0));

                // Level-2 transforms are relative to parent
                let relative_pos = (child_pos.0 - pos.0, child_pos.1 - pos.1);

                let child_complexity = child
                    .get("complexity")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Complex")
                    .to_string();
                let child_adaptable = child
                    .get("adaptable")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let child_evolveable = child
                    .get("evolveable")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                let child_archetype = child.get("archetype").and_then(|v| v.as_str()).map(String::from);
                let child_agent = child.get("agent").cloned();

                let child_system = make_system(
                    &child_id,
                    2,
                    &child_name,
                    &child_desc,
                    &sub_id,
                    11.2,
                    relative_pos,
                    &child_complexity,
                    child_adaptable,
                    child_evolveable,
                    child_archetype.as_deref(),
                    child_agent.as_ref(),
                    None,
                );
                self.systems.push(child_system);
            }
        }
    }

    // -----------------------------------------------------------------------
    // Phase 4: Build interface processors
    // -----------------------------------------------------------------------

    fn build_interface_processors(&mut self) {
        let interfaces: Vec<Value> = self
            .spec
            .get("interfaces")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        for iface in &interfaces {
            let proc_spec = match iface.get("processor") {
                Some(p) if !p.is_null() => p.clone(),
                _ => continue,
            };

            let iface_name = iface
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let iface_id = self
                .interface_ids
                .get(&iface_name)
                .cloned()
                .unwrap_or_default();
            let proc_id = self
                .processor_ids
                .get(&iface_name)
                .cloned()
                .unwrap_or_default();

            // Boundary ID: replace leading 'C' with 'B'
            let boundary_id = if proc_id.starts_with('C') {
                format!("B{}", &proc_id[1..])
            } else {
                proc_id.replace('C', "B")
            };

            let iface_type = iface
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("Import");

            // Processor rotation: Import faces right (-pi), Export faces left (0.0)
            let rotation: f64 = if iface_type == "Import" {
                -3.141_592_5
            } else {
                0.0
            };

            let proc_name = proc_spec
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&iface_name)
                .to_string();
            let proc_desc = proc_spec
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let processor = json!({
                "info": {
                    "id": proc_id,
                    "level": 1,
                    "name": proc_name,
                    "description": proc_desc,
                },
                "sources": [],
                "sinks": [],
                "parent": "S0",
                "complexity": { "Complex": { "adaptable": false, "evolveable": false } },
                "boundary": {
                    "info": {
                        "id": boundary_id,
                        "level": 1,
                        "name": "",
                        "description": "",
                    },
                    "porosity": 0.0_f64,
                    "perceptive_fuzziness": 0.0_f64,
                    "interfaces": [],
                    "parent_interface": iface_id,
                },
                "radius": 12.0_f64,
                "transform": {
                    "translation": [-1.72_f64, 0.0_f64],
                    "rotation": rotation,
                },
                "equivalence": "",
                "history": "",
                "transformation": "",
                "member_autonomy": 1.0_f64,
                "time_constant": "Second",
            });

            self.systems.push(processor);
        }
    }

    // -----------------------------------------------------------------------
    // Phase 5: Build external flows
    // -----------------------------------------------------------------------

    fn build_external_flows(&mut self) {
        let ext_flows: Vec<Value> = self
            .spec
            .get("external_flows")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        for ext_flow in &ext_flows {
            let flow_id = format!("F-1.{}", self.external_flow_counter);
            self.external_flow_counter += 1;

            let direction = ext_flow
                .get("direction")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let iface_name = ext_flow
                .get("interface")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let iface_id: Value = self
                .interface_ids
                .get(&iface_name)
                .map(|id| json!(id))
                .unwrap_or(Value::Null);

            let flow_name = ext_flow
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let flow_desc = ext_flow
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let substance = make_substance(ext_flow);
            let flow_type = ext_flow
                .get("interaction_type")
                .and_then(|v| v.as_str())
                .unwrap_or("Flow")
                .to_string();
            let amount = ext_flow
                .get("amount")
                .map(|v| json!(v.to_string().trim_matches('"').to_string()))
                .unwrap_or_else(|| json!("1"));
            let unit = ext_flow
                .get("unit")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let parameters = make_parameters(ext_flow);

            let flow = if direction == "in" {
                let usability = ext_flow
                    .get("usability")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Resource")
                    .to_string();
                let source_name = ext_flow
                    .get("source")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let source_id: Value = self
                    .source_ids
                    .get(&source_name)
                    .map(|id| json!(id))
                    .unwrap_or_else(|| json!(source_name));

                json!({
                    "info": {
                        "id": flow_id,
                        "level": -1,
                        "name": flow_name,
                        "description": flow_desc,
                    },
                    "substance": substance,
                    "type": flow_type,
                    "usability": usability,
                    "source": source_id,
                    "source_interface": null,
                    "sink": "S0",
                    "sink_interface": iface_id,
                    "amount": amount,
                    "unit": unit,
                    "parameters": parameters,
                })
            } else {
                let usability = ext_flow
                    .get("usability")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Product")
                    .to_string();
                let sink_name = ext_flow
                    .get("sink")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let sink_id: Value = self
                    .sink_ids
                    .get(&sink_name)
                    .map(|id| json!(id))
                    .unwrap_or_else(|| json!(sink_name));

                json!({
                    "info": {
                        "id": flow_id,
                        "level": -1,
                        "name": flow_name,
                        "description": flow_desc,
                    },
                    "substance": substance,
                    "type": flow_type,
                    "usability": usability,
                    "source": "S0",
                    "source_interface": iface_id,
                    "sink": sink_id,
                    "sink_interface": null,
                    "amount": amount,
                    "unit": unit,
                    "parameters": parameters,
                })
            };

            self.interactions.push(flow);
        }
    }

    // -----------------------------------------------------------------------
    // Phase 6: Build processor flows
    // -----------------------------------------------------------------------

    fn build_processor_flows(&mut self) {
        let proc_flows: Vec<Value> = self
            .spec
            .get("processor_flows")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        for pflow in &proc_flows {
            let flow_id = format!("F0.{}", self.internal_flow_counter);
            self.internal_flow_counter += 1;

            let processor_iface = pflow
                .get("processor_interface")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let target_name = pflow
                .get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let proc_id: Value = self
                .processor_ids
                .get(&processor_iface)
                .map(|id| json!(id))
                .unwrap_or(Value::Null);
            let target_id: Value = self
                .subsystem_ids
                .get(&target_name)
                .map(|id| json!(id))
                .unwrap_or(Value::Null);

            let iface_spec = self
                .interface_by_name
                .get(&processor_iface)
                .cloned()
                .unwrap_or_default();
            let iface_type = iface_spec
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("Import");

            let target_pos = self
                .subsystem_positions
                .get(&target_name)
                .copied()
                .unwrap_or((0.0, 0.0));

            let (source_id, sink_id, start_angle, end_angle) = if iface_type == "Import" {
                // Import processor: processor → subsystem (routes inward)
                (proc_id, target_id, 0.0f64, PI)
            } else {
                // Export processor: subsystem → processor (routes outward)
                let sa = angle_from_to(target_pos, (0.0, 0.0));
                (target_id, proc_id, sa, PI)
            };

            let flow_name = pflow
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let flow_desc = pflow
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let substance = make_substance(pflow);
            let flow_type = pflow
                .get("interaction_type")
                .and_then(|v| v.as_str())
                .unwrap_or("Flow")
                .to_string();
            let usability = pflow
                .get("usability")
                .and_then(|v| v.as_str())
                .unwrap_or("Resource")
                .to_string();
            let amount = pflow
                .get("amount")
                .map(|v| json!(v.to_string().trim_matches('"').to_string()))
                .unwrap_or_else(|| json!("1"));
            let unit = pflow
                .get("unit")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let parameters = make_parameters(pflow);

            let flow = json!({
                "info": {
                    "id": flow_id,
                    "level": 1,
                    "name": flow_name,
                    "description": flow_desc,
                },
                "substance": substance,
                "type": flow_type,
                "usability": usability,
                "source": source_id,
                "source_interface": null,
                "sink": sink_id,
                "sink_interface": null,
                "amount": amount,
                "unit": unit,
                "parameters": parameters,
                "endpoint_offset": {
                    "start_angle": start_angle,
                    "end_angle": end_angle,
                },
            });

            self.interactions.push(flow);
        }
    }

    // -----------------------------------------------------------------------
    // Phase 7: Build internal flows
    // -----------------------------------------------------------------------

    fn build_internal_flows(&mut self) {
        let int_flows: Vec<Value> = self
            .spec
            .get("internal_flows")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        for iflow in &int_flows {
            let flow_id = format!("F0.{}", self.internal_flow_counter);
            self.internal_flow_counter += 1;

            let source_name = iflow
                .get("source")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let sink_name = iflow
                .get("sink")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let source_id = self.resolve_name(&source_name);
            let sink_id = self.resolve_name(&sink_name);

            let source_pos = self.resolve_position(&source_name);
            let sink_pos = self.resolve_position(&sink_name);
            let start_angle = angle_from_to(source_pos, sink_pos);
            let end_angle = angle_from_to(sink_pos, source_pos);

            // Level: if both are level-2 siblings (share parent prefix), use 2
            let level = {
                let src_str = source_id.as_str().unwrap_or("");
                let snk_str = sink_id.as_str().unwrap_or("");
                let src_parts = strip_c_prefix_parts(src_str);
                let snk_parts = strip_c_prefix_parts(snk_str);
                if src_parts.len() >= 3
                    && snk_parts.len() >= 3
                    && src_parts[..2] == snk_parts[..2]
                {
                    2
                } else {
                    1
                }
            };

            let flow_name = iflow
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let flow_desc = iflow
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let substance = make_substance(iflow);
            let flow_type = iflow
                .get("interaction_type")
                .and_then(|v| v.as_str())
                .unwrap_or("Flow")
                .to_string();
            let usability = iflow
                .get("usability")
                .and_then(|v| v.as_str())
                .unwrap_or("Resource")
                .to_string();
            let amount = iflow
                .get("amount")
                .map(|v| json!(v.to_string().trim_matches('"').to_string()))
                .unwrap_or_else(|| json!("1"));
            let unit = iflow
                .get("unit")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let parameters = make_parameters(iflow);

            let flow = json!({
                "info": {
                    "id": flow_id,
                    "level": level,
                    "name": flow_name,
                    "description": flow_desc,
                },
                "substance": substance,
                "type": flow_type,
                "usability": usability,
                "source": source_id,
                "source_interface": null,
                "sink": sink_id,
                "sink_interface": null,
                "amount": amount,
                "unit": unit,
                "parameters": parameters,
                "endpoint_offset": {
                    "start_angle": start_angle,
                    "end_angle": end_angle,
                },
            });

            self.interactions.push(flow);
        }
    }

    // -----------------------------------------------------------------------
    // Assembly
    // -----------------------------------------------------------------------

    fn assemble(&self) -> Value {
        json!({
            "version": 1,
            "environment": self.environment,
            "systems": self.systems,
            "interactions": self.interactions,
            "hidden_entities": [],
        })
    }

    // -----------------------------------------------------------------------
    // Helpers: name and position resolution
    // -----------------------------------------------------------------------

    fn resolve_name(&self, name: &str) -> Value {
        // Direct subsystem match
        if let Some(id) = self.subsystem_ids.get(name) {
            return json!(id);
        }
        // Processor match by interface name
        if let Some(id) = self.processor_ids.get(name) {
            return json!(id);
        }
        // Processor match by processor_name
        if let Some(iface_name) = self.processor_name_to_iface.get(name) {
            if let Some(id) = self.processor_ids.get(iface_name) {
                return json!(id);
            }
        }
        // Parent/Child format: "Mining/Hash Production" → look up "Hash Production"
        if let Some(slash) = name.find('/') {
            let child_name = &name[slash + 1..];
            if let Some(id) = self.subsystem_ids.get(child_name) {
                return json!(id);
            }
        }
        json!(name)
    }

    fn resolve_position(&self, name: &str) -> (f64, f64) {
        if let Some(&pos) = self.subsystem_positions.get(name) {
            return pos;
        }
        // Processor name → interface name → position
        if let Some(iface_name) = self.processor_name_to_iface.get(name) {
            if let Some(&pos) = self.subsystem_positions.get(iface_name) {
                return pos;
            }
        }
        // Parent/Child format
        if let Some(slash) = name.find('/') {
            let child_name = &name[slash + 1..];
            if let Some(&pos) = self.subsystem_positions.get(child_name) {
                return pos;
            }
        }
        (0.0, 0.0)
    }

    /// Look up the computed angle for a named interface (mirrors Python's
    /// `_get_interface_angle`). Recomputes from partition counts since angles
    /// are not stored separately.
    fn get_interface_angle(&self, iface_name: &str) -> Option<f64> {
        let iface_specs: Vec<Value> = self
            .spec
            .get("interfaces")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let n_imports = iface_specs
            .iter()
            .filter(|i| i.get("type").and_then(|v| v.as_str()) == Some("Import"))
            .count();
        let n_exports = iface_specs
            .iter()
            .filter(|i| i.get("type").and_then(|v| v.as_str()) == Some("Export"))
            .count();

        let import_angles = compute_interface_angles(n_imports, "Import");
        let export_angles = compute_interface_angles(n_exports, "Export");

        let mut imp_idx = 0usize;
        let mut exp_idx = 0usize;

        for iface in &iface_specs {
            let name = iface.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let ty = iface.get("type").and_then(|v| v.as_str()).unwrap_or("Import");

            if name == iface_name {
                return if ty == "Import" {
                    import_angles.get(imp_idx).copied().or(Some(PI))
                } else {
                    export_angles.get(exp_idx).copied().or(Some(0.0))
                };
            }

            if ty == "Import" {
                imp_idx += 1;
            } else {
                exp_idx += 1;
            }
        }

        None
    }
}

// ---------------------------------------------------------------------------
// Free helpers: system construction
// ---------------------------------------------------------------------------

/// Construct a system JSON object with all required fields.
#[allow(clippy::too_many_arguments)]
fn make_system(
    sys_id: &str,
    level: i64,
    name: &str,
    description: &str,
    parent: &str,
    radius: f64,
    position: (f64, f64),
    complexity: &str,
    adaptable: bool,
    evolveable: bool,
    archetype: Option<&str>,
    agent: Option<&Value>,
    parent_interface: Option<&str>,
) -> Value {
    let boundary_id = sys_id
        .replacen('C', "B", 1)
        .replacen('S', "B", 1);

    let complexity_val = serialize_complexity(complexity, adaptable, evolveable);

    let pi_val: Value = match parent_interface {
        Some(pi) => json!(pi),
        None => Value::Null,
    };

    let mut system = json!({
        "info": {
            "id": sys_id,
            "level": level,
            "name": name,
            "description": description,
        },
        "sources": [],
        "sinks": [],
        "parent": parent,
        "complexity": complexity_val,
        "boundary": {
            "info": {
                "id": boundary_id,
                "level": level,
                "name": "",
                "description": "",
            },
            "porosity": 0.0_f64,
            "perceptive_fuzziness": 0.0_f64,
            "interfaces": [],
            "parent_interface": pi_val,
        },
        "radius": radius,
        "transform": {
            "translation": [position.0, position.1],
            "rotation": 0.0_f64,
        },
        "equivalence": "",
        "history": "",
        "transformation": "",
        "member_autonomy": 1.0_f64,
        "time_constant": "Second",
    });

    // Optional archetype
    if let Some(arch) = archetype {
        system["archetype"] = json!(arch);

        // Optional agent model (only when archetype == "Agent")
        if arch == "Agent" {
            if let Some(agent_val) = agent {
                let kind = agent_val
                    .get("kind")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Reactive");
                let capacity = agent_val
                    .get("agency_capacity")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5);
                system["agent"] = json!({
                    "kind": kind,
                    "agency_capacity": capacity,
                });
            }
        }
    }

    system
}

/// Serialize the complexity field to the BERT JSON format.
///
/// - `"Atomic"` → bare string `"Atomic"`
/// - `"Complex"` → `{"Complex": {"adaptable": bool, "evolveable": bool}}`
/// - `"Multiset"` or `"Multiset:N"` → `{"Multiset": N}`
fn serialize_complexity(complexity: &str, adaptable: bool, evolveable: bool) -> Value {
    if complexity == "Atomic" {
        return json!("Atomic");
    }
    if complexity == "Complex" {
        return json!({ "Complex": { "adaptable": adaptable, "evolveable": evolveable } });
    }
    if complexity.starts_with("Multiset") {
        let n: u64 = complexity
            .split(':')
            .nth(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        return json!({ "Multiset": n });
    }
    // Default
    json!({ "Complex": { "adaptable": adaptable, "evolveable": evolveable } })
}

// ---------------------------------------------------------------------------
// Free helpers: substance and parameters
// ---------------------------------------------------------------------------

fn make_substance(flow: &Value) -> Value {
    let stype = flow
        .get("substance_type")
        .and_then(|v| v.as_str())
        .unwrap_or("Message");
    let ssub = flow
        .get("substance_subtype")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    json!({ "sub_type": ssub, "type": stype })
}

fn make_parameters(flow: &Value) -> Vec<Value> {
    flow.get("parameters")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|p| {
                    let name = p
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let value = p
                        .get("value")
                        .map(|v| v.to_string().trim_matches('"').to_string())
                        .unwrap_or_default();
                    let unit = p
                        .get("unit")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    json!({ "name": name, "value": value, "unit": unit })
                })
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Free helpers: spatial layout
// ---------------------------------------------------------------------------

fn compute_source_positions(n: usize) -> Vec<(f64, f64)> {
    match n {
        0 => vec![],
        1 => vec![(-520.0, 0.0)],
        2 => vec![(-520.0, -150.0), (-520.0, 150.0)],
        3 => vec![(-520.0, 0.0), (-456.0, 200.0), (-456.0, -200.0)],
        _ => {
            let total_span = (n - 1) as f64 * 150.0;
            let start_y = -total_span / 2.0;
            (0..n)
                .map(|i| (-520.0, start_y + i as f64 * 150.0))
                .collect()
        }
    }
}

fn compute_sink_positions(n: usize) -> Vec<(f64, f64)> {
    match n {
        0 => vec![],
        1 => vec![(520.0, 0.0)],
        2 => vec![(520.0, -150.0), (520.0, 150.0)],
        3 => vec![(520.0, 0.0), (456.0, 200.0), (456.0, -200.0)],
        _ => {
            let total_span = (n - 1) as f64 * 150.0;
            let start_y = -total_span / 2.0;
            (0..n)
                .map(|i| (520.0, start_y + i as f64 * 150.0))
                .collect()
        }
    }
}

fn compute_level1_positions(n: usize) -> Vec<(f64, f64)> {
    match n {
        0 => vec![],
        1 => vec![(0.0, 0.0)],
        2 => vec![(-80.0, 0.0), (80.0, 0.0)],
        3 => vec![(-80.0, -60.0), (80.0, -60.0), (0.0, 100.0)],
        4 => vec![(0.0, -100.0), (-100.0, 0.0), (100.0, 0.0), (0.0, 100.0)],
        5 => {
            // Pentagon at 72-degree intervals, radius 120
            (0..5usize)
                .map(|i| {
                    let angle = -PI / 2.0 + i as f64 * (2.0 * PI / 5.0);
                    let x = round2(120.0 * angle.cos());
                    let y = round2(120.0 * angle.sin());
                    (x, y)
                })
                .collect()
        }
        _ => {
            // 6+: distribute at (360/N) degree intervals, radius 120
            (0..n)
                .map(|i| {
                    let angle = -PI / 2.0 + i as f64 * (2.0 * PI / n as f64);
                    let x = round2(120.0 * angle.cos());
                    let y = round2(120.0 * angle.sin());
                    (x, y)
                })
                .collect()
        }
    }
}

/// Compute absolute positions for N level-2 children inside a parent.
///
/// Returns absolute coords because `_subsystem_positions` stores absolute
/// coords for angle computation. The transform written to JSON will subtract
/// the parent position (`_build_subsystems`).
fn compute_level2_positions(n: usize, parent_pos: (f64, f64)) -> Vec<(f64, f64)> {
    let (px, py) = parent_pos;
    match n {
        0 => vec![],
        1 => vec![(px, py)],
        2 => vec![(px - 15.0, py), (px + 15.0, py)],
        3 => vec![
            (px - 20.0, py + 12.0),
            (px + 5.0, py + 12.0),
            (px + 20.0, py - 12.0),
        ],
        4 => vec![
            (px - 15.0, py - 15.0),
            (px + 15.0, py - 15.0),
            (px - 15.0, py + 15.0),
            (px + 15.0, py + 15.0),
        ],
        _ => {
            let r = 18.0_f64;
            (0..n)
                .map(|i| {
                    let angle = -PI / 2.0 + i as f64 * (2.0 * PI / n as f64);
                    let x = round2(px + r * angle.cos());
                    let y = round2(py + r * angle.sin());
                    (x, y)
                })
                .collect()
        }
    }
}

/// Compute evenly-spaced angles for N interfaces of a given type.
///
/// Import angles: evenly spaced in [2.5, 3.8] (left half of boundary)
/// Export angles: evenly spaced in [-0.5, 0.5] (right half of boundary)
fn compute_interface_angles(n: usize, iface_type: &str) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }

    if iface_type == "Import" {
        if n == 1 {
            return vec![PI];
        }
        let (start, end) = (2.5_f64, 3.8_f64);
        let step = (end - start) / (n - 1) as f64;
        (0..n).map(|i| start + i as f64 * step).collect()
    } else {
        // Export
        if n == 1 {
            return vec![0.0];
        }
        let (start, end) = (-0.5_f64, 0.5_f64);
        let step = (end - start) / (n - 1) as f64;
        (0..n).map(|i| start + i as f64 * step).collect()
    }
}

// ---------------------------------------------------------------------------
// Free helpers: geometry
// ---------------------------------------------------------------------------

/// Compute the angle (atan2) from `from_pos` toward `to_pos`.
fn angle_from_to(from_pos: (f64, f64), to_pos: (f64, f64)) -> f64 {
    let dx = to_pos.0 - from_pos.0;
    let dy = to_pos.1 - from_pos.1;
    if dx == 0.0 && dy == 0.0 {
        return 0.0;
    }
    dy.atan2(dx)
}

/// Round to 2 decimal places (matches Python's `round(x, 2)`).
fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

/// Split a BERT ID like "C0.1.2" into numeric parts ["0", "1", "2"].
/// Strips the leading 'C' or 'B' prefix before splitting on '.'.
fn strip_c_prefix_parts(id: &str) -> Vec<&str> {
    let stripped = if id.starts_with('C') || id.starts_with('B') {
        &id[1..]
    } else {
        id
    };
    stripped.split('.').collect()
}
