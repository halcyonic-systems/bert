//! The Compose seam as a protocol: `validate_operational` either projects a
//! [`WorldModel`] into an [`OperationalSpec`] — everything a conservation-
//! faithful simulator needs — or refuses with the complete list of
//! [`OperationalError`]s. There is no partial projection: a model that loses
//! information on the way to execution is rejected, not trimmed.
//!
//! This lifts the gate that previously lived implicitly in bert-compose's
//! `from_world_model` error paths into the kernel crate, so any executor
//! (compose today, Mesa via the bridge, others later) checks the same
//! contract. Representational modes fail loudly by design: Klir's Core and
//! Bunge's Structural rungs commit to no flow semantics, so there is nothing
//! to execute — the refusal cites the rung, not a defect.
//!
//! What the contract deliberately tolerates: dead ends (a process with no
//! outgoing flow). The compose engine defines evaporation semantics for them
//! (`dead_ends()` surfaces, the ledger stays exact), so they are a modeling
//! choice, not an execution hazard. Isolated components are different — a
//! component no flow touches would ride along silently, so it is an error.

use crate::validate::{validate_mode, Severity};
use crate::{
    Id, IdType, InteractionType, Mode, ProcessPrimitive, SubstanceType, WorldModel,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Why a model may not be executed, with the seam's location grammar
/// (`systems[i]`, `interactions[k]`, `mode`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalError {
    pub location: String,
    pub reason: String,
    pub hint: Option<String>,
}

impl OperationalError {
    fn new(
        location: impl Into<String>,
        reason: impl Into<String>,
        hint: Option<&str>,
    ) -> Self {
        Self {
            location: location.into(),
            reason: reason.into(),
            hint: hint.map(str::to_string),
        }
    }
}

/// A work process the executor must instantiate: one Mobus primitive with its
/// asserted parameters. Mirrors what compose reads out of `AgentModel`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalProcess {
    pub id: Id,
    pub name: String,
    pub primitive: ProcessPrimitive,
    pub agency_capacity: f32,
    pub cognitive_params: HashMap<String, f64>,
    pub initial_storage: Option<f64>,
}

/// A boundary terminal: an environment source or sink a flow may cross.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalTerminal {
    pub id: Id,
    pub name: String,
}

/// A directed, substance-typed flow between projected endpoints.
/// `conductance` is `Some` exactly for gradient (Force) flows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalFlow {
    pub name: String,
    pub source: Id,
    pub sink: Id,
    pub substance_name: String,
    pub substance: SubstanceType,
    pub unit: String,
    pub amount: f64,
    pub conductance: Option<f64>,
}

/// The executable projection of a [`WorldModel`]: the seam payload.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OperationalSpec {
    pub processes: Vec<OperationalProcess>,
    pub sources: Vec<OperationalTerminal>,
    pub sinks: Vec<OperationalTerminal>,
    pub flows: Vec<OperationalFlow>,
}

/// Project `model` into an [`OperationalSpec`], or refuse with every reason.
///
/// The checks, in the order they accumulate:
/// 1. Mode gate — `Core` (Klir) and `Structural` (Bunge) are representational
///    rungs and never execute.
/// 2. The `Operational` rung's own validity: on-ness plus irreflexivity, via
///    [`validate_mode`].
/// 3. Projection totality — every level-1 system carries a primitive, no
///    hierarchy below level 1, every flow endpoint lands on a projected node,
///    boundary crossings run in the declared direction, gradient flows carry
///    a conductance, and no component is isolated from the flow graph.
pub fn validate_operational(
    model: &WorldModel,
) -> Result<OperationalSpec, Vec<OperationalError>> {
    let mut errors: Vec<OperationalError> = Vec::new();

    match model.mode() {
        Mode::Core => errors.push(OperationalError::new(
            "mode",
            "Core (Klir) commits to things and dependency only — a representational \
             rung with no flow semantics to execute",
            Some("Author the model at the Operational rung, or export from compose"),
        )),
        Mode::Structural => errors.push(OperationalError::new(
            "mode",
            "Structural (Bunge) commits to bonded composition, not typed flows — \
             a representational rung with no flow semantics to execute",
            Some("Author the model at the Operational rung, or export from compose"),
        )),
        Mode::Operational | Mode::Full => {}
    }

    for issue in validate_mode(model, Mode::Operational)
        .issues
        .into_iter()
        .filter(|i| i.severity == Severity::Error)
    {
        errors.push(OperationalError::new(issue.location, issue.message, None));
    }

    let mut spec = OperationalSpec::default();
    let mut projected: HashSet<Id> = HashSet::new();

    for ext in &model.environment.sources {
        projected.insert(ext.info.id.clone());
        spec.sources.push(OperationalTerminal {
            id: ext.info.id.clone(),
            name: ext.info.name.clone(),
        });
    }
    for ext in &model.environment.sinks {
        projected.insert(ext.info.id.clone());
        spec.sinks.push(OperationalTerminal {
            id: ext.info.id.clone(),
            name: ext.info.name.clone(),
        });
    }

    for (i, sys) in model.systems.iter().enumerate() {
        if sys.info.level <= 0 {
            continue;
        }
        if sys.info.level > 1 {
            errors.push(OperationalError::new(
                format!("systems[{i}]"),
                format!(
                    "\"{}\" sits at level {} — hierarchy below level 1 has no \
                     executable reading in the flat circuit vocabulary",
                    sys.info.name, sys.info.level
                ),
                Some("Flatten the model to one level of work processes first"),
            ));
            continue;
        }
        let Some(agent) = sys.agent.as_ref() else {
            errors.push(OperationalError::new(
                format!("systems[{i}]"),
                format!(
                    "\"{}\" has no agent model — nothing states which work process it is",
                    sys.info.name
                ),
                Some("Attach an AgentModel carrying a Mobus primitive"),
            ));
            continue;
        };
        let Some(&primitive) = agent.primitives.first() else {
            errors.push(OperationalError::new(
                format!("systems[{i}]"),
                format!(
                    "\"{}\" carries no Mobus primitive — nothing to instantiate",
                    sys.info.name
                ),
                Some("Set AgentModel.primitives to the process kind"),
            ));
            continue;
        };
        projected.insert(sys.info.id.clone());
        spec.processes.push(OperationalProcess {
            id: sys.info.id.clone(),
            name: sys.info.name.clone(),
            primitive,
            agency_capacity: agent.agency_capacity,
            cognitive_params: agent.cognitive_params.clone(),
            initial_storage: agent.initial_state.get("storage").and_then(|v| v.as_f64()),
        });
    }

    let mut touched: HashSet<Id> = HashSet::new();
    for (k, ix) in model.interactions.iter().enumerate() {
        let loc = format!("interactions[{k}]");
        let mut flow_ok = true;
        if ix.source_interface.is_some() || ix.sink_interface.is_some() {
            errors.push(OperationalError::new(
                &loc,
                format!(
                    "flow \"{}\" routes through a boundary interface — interface \
                     routing has no executable reading yet (bert#108: the \
                     component → work-process mapping is an open design problem)",
                    ix.info.name
                ),
                Some("Attach the flow directly to the process endpoints, or wait for 4.2"),
            ));
            flow_ok = false;
        }
        if ix.source.ty == IdType::Sink {
            errors.push(OperationalError::new(
                &loc,
                format!(
                    "flow \"{}\" leaves a sink — boundary crossings run in the \
                     declared direction only",
                    ix.info.name
                ),
                Some("A sink terminates flow; swap the endpoints or the entity type"),
            ));
            flow_ok = false;
        }
        if ix.sink.ty == IdType::Source {
            errors.push(OperationalError::new(
                &loc,
                format!(
                    "flow \"{}\" enters a source — boundary crossings run in the \
                     declared direction only",
                    ix.info.name
                ),
                Some("A source originates flow; swap the endpoints or the entity type"),
            ));
            flow_ok = false;
        }
        for (end, which) in [(&ix.source, "source"), (&ix.sink, "sink")] {
            if !projected.contains(end) {
                errors.push(OperationalError::new(
                    &loc,
                    format!(
                        "flow \"{}\" {which} does not resolve to a projected node \
                         (a level-1 work process or an environment terminal)",
                        ix.info.name
                    ),
                    None,
                ));
                flow_ok = false;
            }
        }
        let conductance = if ix.ty == InteractionType::Force {
            let k = ix
                .parameters
                .iter()
                .find(|p| p.name == "conductance")
                .and_then(|p| p.value.parse::<f64>().ok());
            if k.is_none() {
                errors.push(OperationalError::new(
                    &loc,
                    format!(
                        "gradient flow \"{}\" carries no parsable conductance — \
                         a Force interaction is under-specified without its k",
                        ix.info.name
                    ),
                    Some("Add a `conductance` parameter to the flow"),
                ));
                flow_ok = false;
            }
            k
        } else {
            None
        };
        if !flow_ok {
            continue;
        }
        touched.insert(ix.source.clone());
        touched.insert(ix.sink.clone());
        spec.flows.push(OperationalFlow {
            name: ix.info.name.clone(),
            source: ix.source.clone(),
            sink: ix.sink.clone(),
            substance_name: ix.substance.sub_type.clone(),
            substance: ix.substance.ty,
            unit: ix.unit.clone(),
            amount: ix.amount.to_string().parse::<f64>().unwrap_or(1.0),
            conductance,
        });
    }

    for p in &spec.processes {
        if !touched.contains(&p.id) {
            errors.push(OperationalError::new(
                format!("process \"{}\"", p.name),
                "no flow touches this component — it would ride along in the \
                 circuit without participating"
                    .to_string(),
                Some("Wire it into the flow graph or remove it"),
            ));
        }
    }

    if errors.is_empty() {
        Ok(spec)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AgentKind, AgentModel, Boundary, Complexity, Environment, ExternalEntity,
        ExternalEntityType, Info, Interaction, InteractionUsability, Parameter, Substance,
        System, CURRENT_FILE_VERSION,
    };

    fn id(ty: IdType, indices: &[i64]) -> Id {
        Id {
            ty,
            indices: indices.to_vec(),
        }
    }

    fn info(i: Id, level: i32, name: &str) -> Info {
        Info {
            id: i,
            level,
            name: name.to_string(),
            description: String::new(),
        }
    }

    fn system(indices: &[i64], name: &str, parent: Id, agent: Option<AgentModel>) -> System {
        let level = indices.len() as i32 - 1;
        System {
            info: info(id(IdType::Subsystem, indices), level, name),
            sources: vec![],
            sinks: vec![],
            parent,
            complexity: Complexity::Atomic,
            boundary: Boundary {
                info: info(id(IdType::Boundary, indices), level, ""),
                porosity: 0.0,
                perceptive_fuzziness: 0.0,
                interfaces: vec![],
                parent_interface: None,
            },
            radius: 50.0,
            transform: None,
            equivalence: String::new(),
            history: String::new(),
            transformation: String::new(),
            member_autonomy: 1.0,
            time_constant: String::new(),
            archetype: None,
            agent,
        }
    }

    fn agent(primitive: ProcessPrimitive) -> AgentModel {
        AgentModel {
            kind: AgentKind::Reactive,
            agency_capacity: 1.0,
            primitives: vec![primitive],
            cognitive_params: HashMap::new(),
            process_configs: vec![],
            initial_state: HashMap::new(),
            network_config: None,
        }
    }

    fn flow(idx: i64, name: &str, source: Id, sink: Id) -> Interaction {
        Interaction {
            info: info(id(IdType::Flow, &[idx]), 0, name),
            substance: Substance {
                sub_type: "water".to_string(),
                ty: SubstanceType::Material,
            },
            ty: InteractionType::Flow,
            usability: InteractionUsability::Product,
            source,
            source_interface: None,
            sink,
            sink_interface: None,
            amount: crate::rust_decimal::Decimal::ONE,
            unit: "L".to_string(),
            parameters: vec![],
            smart_parameters: vec![],
            endpoint_offset: None,
        }
    }

    /// Source → buffer → sink: the smallest hand-crafted Mobus model.
    fn mobus_model() -> WorldModel {
        let env_id = id(IdType::Environment, &[-1]);
        let root = id(IdType::System, &[0]);
        let src = id(IdType::Source, &[-1, 0]);
        let snk = id(IdType::Sink, &[-1, 0]);
        let buf = id(IdType::Subsystem, &[0, 0]);
        let mut root_sys = system(&[0], "Root", env_id.clone(), None);
        root_sys.info.id = root.clone();
        root_sys.info.level = 0;
        root_sys.parent = env_id.clone();
        WorldModel {
            version: CURRENT_FILE_VERSION,
            mode: Some(Mode::Operational),
            environment: Environment {
                info: info(env_id, -1, "Environment"),
                sources: vec![ExternalEntity {
                    info: info(src.clone(), -1, "Well"),
                    ty: ExternalEntityType::Source,
                    transform: None,
                    equivalence: String::new(),
                    model: String::new(),
                    is_same_as_id: None,
                }],
                sinks: vec![ExternalEntity {
                    info: info(snk.clone(), -1, "Drain"),
                    ty: ExternalEntityType::Sink,
                    transform: None,
                    equivalence: String::new(),
                    model: String::new(),
                    is_same_as_id: None,
                }],
            },
            systems: vec![
                root_sys,
                system(&[0, 0], "Tank", root, Some(agent(ProcessPrimitive::Buffering))),
            ],
            interactions: vec![
                flow(0, "well → tank", src, buf.clone()),
                flow(1, "tank → drain", buf, snk),
            ],
            hidden_entities: vec![],
        }
    }

    #[test]
    fn mobus_model_projects() {
        let spec = validate_operational(&mobus_model()).expect("projects clean");
        assert_eq!(spec.processes.len(), 1);
        assert_eq!(spec.processes[0].primitive, ProcessPrimitive::Buffering);
        assert_eq!(spec.sources.len(), 1);
        assert_eq!(spec.sinks.len(), 1);
        assert_eq!(spec.flows.len(), 2);
        assert_eq!(spec.flows[0].substance, SubstanceType::Material);
        assert_eq!(spec.flows[0].unit, "L");
    }

    #[test]
    fn klir_mode_refuses_with_cited_rung() {
        let mut m = mobus_model();
        m.mode = Some(Mode::Core);
        let errs = validate_operational(&m).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| e.location == "mode" && e.reason.contains("Core (Klir)")),
            "the refusal names the rung: {errs:#?}"
        );
    }

    #[test]
    fn bunge_mode_refuses_with_cited_rung() {
        let mut m = mobus_model();
        m.mode = Some(Mode::Structural);
        let errs = validate_operational(&m).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| e.location == "mode" && e.reason.contains("Structural (Bunge)")),
            "the refusal names the rung: {errs:#?}"
        );
    }

    #[test]
    fn missing_primitive_is_named() {
        let mut m = mobus_model();
        m.systems[1].agent.as_mut().unwrap().primitives.clear();
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs.iter().any(|e| e.reason.contains("Tank")
            && e.reason.contains("no Mobus primitive")));
    }

    #[test]
    fn missing_agent_is_named() {
        let mut m = mobus_model();
        m.systems[1].agent = None;
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs
            .iter()
            .any(|e| e.reason.contains("Tank") && e.reason.contains("no agent model")));
    }

    #[test]
    fn deep_hierarchy_refused() {
        let mut m = mobus_model();
        let parent = m.systems[1].info.id.clone();
        m.systems.push(system(
            &[0, 0, 0],
            "Inner",
            parent,
            Some(agent(ProcessPrimitive::Propelling)),
        ));
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs.iter().any(|e| e.reason.contains("level 2")));
    }

    #[test]
    fn reversed_boundary_crossing_refused() {
        let mut m = mobus_model();
        let src = m.environment.sources[0].info.id.clone();
        let snk = m.environment.sinks[0].info.id.clone();
        m.interactions.push(flow(2, "backwash", snk, src));
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs.iter().any(|e| e.reason.contains("leaves a sink")));
        assert!(errs.iter().any(|e| e.reason.contains("enters a source")));
    }

    #[test]
    fn gradient_flow_needs_conductance() {
        let mut m = mobus_model();
        m.interactions[1].ty = InteractionType::Force;
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs
            .iter()
            .any(|e| e.reason.contains("no parsable conductance")));

        m.interactions[1].parameters.push(Parameter {
            name: "conductance".to_string(),
            value: "0.42".to_string(),
            ..Default::default()
        });
        let spec = validate_operational(&m).expect("conductance satisfies the contract");
        assert_eq!(spec.flows[1].conductance, Some(0.42));
    }

    #[test]
    fn isolated_component_refused() {
        let mut m = mobus_model();
        let root = m.systems[0].info.id.clone();
        m.systems.push(system(
            &[0, 1],
            "Stray",
            root,
            Some(agent(ProcessPrimitive::Propelling)),
        ));
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs
            .iter()
            .any(|e| e.location.contains("Stray") && e.reason.contains("no flow touches")));
    }

    #[test]
    fn interface_routing_refused_citing_108() {
        let mut m = mobus_model();
        m.interactions[0].sink_interface = Some(id(IdType::Interface, &[0, 0, 0]));
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs
            .iter()
            .any(|e| e.reason.contains("bert#108") && e.reason.contains("interface")));
    }

    #[test]
    fn spec_round_trips_as_json() {
        let spec = validate_operational(&mobus_model()).expect("projects");
        let json = serde_json::to_string(&spec).expect("serializes");
        let back: OperationalSpec = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(spec, back, "the seam payload survives JSON unchanged");
    }

    #[test]
    fn self_loop_refused_via_operational_rung() {
        let mut m = mobus_model();
        let buf = m.systems[1].info.id.clone();
        m.interactions.push(flow(2, "ouroboros", buf.clone(), buf));
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs.iter().any(|e| e.reason.contains("k ≠ o")));
    }
}
