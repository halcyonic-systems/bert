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
//!
//! # Interface routing lowers, it does not refuse (bert#108)
//!
//! A flow routed through a boundary interface used to refuse: the component →
//! work-process mapping was an open question. bert#108 settled it (Option 3,
//! identity default): an interface is a component sited in the boundary B, and
//! by Mobus every component recapitulates the fundamental work processes, so
//! the routing *lowers* to a work process already in compose's palette — an
//! [`InterfacePrimitive::Impeding`] filter (with back-pressure) by default.
//!
//! Unparameterized — always, at this rung — the transfer characteristic is the
//! identity: the degenerate zero-width conduit. So the flow still attaches
//! directly to its declared endpoints and runs exactly as if the interface
//! were absent; the lowering is recorded on [`OperationalFlow::interface_routing`]
//! as provenance, nothing more. **This is an EDGE annotation, not a node
//! insertion** (the 4.2 implementation call): the flow keeps its two declared
//! endpoints and gains a marker, rather than splicing an intermediary node
//! between them. The edge form makes the identity default *exactly* the flow
//! attaching directly (same endpoints, same one-tick latency, same ledger) —
//! a spliced node would shift the trajectory by a tick and so would not be
//! observationally equivalent, which the contract requires. The marker is the
//! seam where a future parameterized characteristic would attach (promoting to
//! a real Impeding node); 4.2 ships only the identity default.

use crate::validate::{validate_mode, Severity};
use crate::{Id, IdType, InteractionType, Mode, ProcessPrimitive, SubstanceType, WorldModel};
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
    fn new(location: impl Into<String>, reason: impl Into<String>, hint: Option<&str>) -> Self {
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

/// The work process a boundary interface recapitulates when an interface-routed
/// flow is lowered for execution (bert#108). Mobus: every component, interfaces
/// included, recapitulates the fundamental work processes; an interface sited in
/// the boundary B is an Impeding process — a filter with back-pressure — by
/// default. Unparameterized the characteristic is the identity (the degenerate
/// zero-width conduit), so at this rung the choice of primitive is provenance
/// only; it names where a future transfer characteristic would attach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfacePrimitive {
    /// A filter with back-pressure (Mobus). The bert#108 default.
    Impeding,
    /// A control-gated conduit.
    Modulating,
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
    /// `Some` iff this flow lowered from boundary-interface routing (bert#108),
    /// naming the work process the interface recapitulates. Unparameterized (the
    /// only form 4.2 ships) it is the identity/zero-width conduit, so the flow
    /// still attaches directly to `source`/`sink` and executors build a single
    /// direct wire — the marker carries the #108 lineage and nothing dynamical.
    #[serde(default)]
    pub interface_routing: Option<InterfacePrimitive>,
    /// Series forcing (bert-lenses#16): `Some` iff this flow carries an
    /// OBSERVED emission series (a `series` parameter on the interaction),
    /// making the source emit `o_Src(t) = f(q,t)` tick by tick instead of the
    /// scalar `amount` mean — Mobus's naked-interface output function. Rides
    /// `Interaction.parameters` exactly as `conductance` does. `skip` when
    /// `None` so an unforced spec serializes and hashes byte-for-byte as
    /// before (the model-1 receipt stays reproducible).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_series: Option<Vec<f64>>,
    /// Multi-timescale (rung 3): `Some(n)` iff this flow's series is sampled on
    /// a SLOWER clock — one value every `n` fast ticks, zero-order-held between
    /// (the channel's Δt = n × base Δt, Mobus's per-node `Δt_{i,l}`, ch4
    /// §4.3.3.6). Rides a `dt_stride` parameter on the interaction, as
    /// `rate_series` rides `series`. `None` = every tick. `skip` when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_stride: Option<u32>,
}

/// The executable projection of a [`WorldModel`]: the seam payload.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OperationalSpec {
    pub processes: Vec<OperationalProcess>,
    pub sources: Vec<OperationalTerminal>,
    pub sinks: Vec<OperationalTerminal>,
    pub flows: Vec<OperationalFlow>,
}

impl OperationalSpec {
    /// A content hash over everything that determines a run — the whole spec.
    /// A structural edit (a new process, a rewired flow, a changed primitive)
    /// moves the hash; a cosmetic edit that never reaches the projection (a
    /// canvas position) leaves it fixed. This is the key a recorded run `H`
    /// hangs on: `H` is valid for exactly the spec that hashes to this value,
    /// so a later structural edit surfaces the recording as stale rather than
    /// letting the old trajectory pose as current.
    ///
    /// Canonical by construction: serializing through `serde_json::Value`
    /// (whose object maps are sorted) makes the digest independent of the
    /// `HashMap` iteration order in `cognitive_params`.
    pub fn content_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let canonical = serde_json::to_value(self)
            .map(|v| v.to_string())
            .unwrap_or_default();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        canonical.hash(&mut hasher);
        hasher.finish()
    }
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
pub fn validate_operational(model: &WorldModel) -> Result<OperationalSpec, Vec<OperationalError>> {
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
        let Some(primitive) = agent.primitive else {
            errors.push(OperationalError::new(
                format!("systems[{i}]"),
                format!(
                    "\"{}\" carries no Mobus primitive — nothing to instantiate",
                    sys.info.name
                ),
                Some("Set AgentModel.primitive to the process kind"),
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
        // bert#108: interface routing lowers, it does not refuse. An interface
        // is a component sited in the boundary B; by Mobus it recapitulates a
        // work process, so the routing lowers to an Impeding filter. Identity
        // default at this rung — the flow still attaches directly to its
        // declared endpoints (the endpoint/direction checks below run on
        // `ix.source`/`ix.sink` regardless), and this only records the lineage.
        // (Interface-reference integrity is enforced upstream by `validate`;
        // a dangling interface id still errors there.)
        let interface_routing = (ix.source_interface.is_some() || ix.sink_interface.is_some())
            .then_some(InterfacePrimitive::Impeding);
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
        // Series forcing (#16): an observed emission series rides a `series`
        // parameter (comma-separated), read exactly as `conductance` is. An
        // empty/unparsable list is treated as absent, so a forced flow always
        // carries at least one tick — the mean `amount` remains the fallback.
        let rate_series = ix
            .parameters
            .iter()
            .find(|p| p.name == "series")
            .map(|p| {
                p.value
                    .split(',')
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .collect::<Vec<f64>>()
            })
            .filter(|s| !s.is_empty());
        // Multi-timescale (#rung 3): the channel's Δt as an integer multiple of
        // the base tick — read exactly as `series` is; >1 means slower.
        let dt_stride = ix
            .parameters
            .iter()
            .find(|p| p.name == "dt_stride")
            .and_then(|p| p.value.trim().parse::<u32>().ok())
            .filter(|n| *n > 1);
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
            interface_routing,
            rate_series,
            dt_stride,
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
        ExternalEntityType, Info, Interaction, InteractionUsability, Interface, InterfaceType,
        Parameter, Substance, System, CURRENT_FILE_VERSION,
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
            primitive: Some(primitive),
            cognitive_params: HashMap::new(),
            process_configs: vec![],
            initial_state: HashMap::new(),
            network_config: None,
        }
    }

    fn iface(iid: Id, ty: InterfaceType) -> Interface {
        Interface {
            info: info(iid, 1, "i"),
            protocol: String::new(),
            ty,
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(0.0),
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
                system(
                    &[0, 0],
                    "Tank",
                    root,
                    Some(agent(ProcessPrimitive::Buffering)),
                ),
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
        m.systems[1].agent.as_mut().unwrap().primitive = None;
        let errs = validate_operational(&m).unwrap_err();
        assert!(errs
            .iter()
            .any(|e| e.reason.contains("Tank") && e.reason.contains("no Mobus primitive")));
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

    /// bert#108: interface routing LOWERS, it does not refuse. A flow routed
    /// through a real (resolvable) boundary interface now projects, carrying
    /// its Impeding lowering as provenance. The lineage survives here (the
    /// `108` in the name and the `InterfacePrimitive` doc); the interim refusal
    /// is gone. (Renamed from `interface_routing_refused_citing_108`.)
    fn routed_mobus_model() -> WorldModel {
        let mut m = mobus_model();
        // Attach an Import interface to the Tank's boundary so the routing
        // resolves (a dangling id would still fail `validate`), then route the
        // well → tank flow through it.
        let iface_id = id(IdType::Interface, &[0, 0, 0]);
        m.systems[1]
            .boundary
            .interfaces
            .push(iface(iface_id.clone(), InterfaceType::Import));
        m.interactions[0].sink_interface = Some(iface_id);
        m
    }

    #[test]
    fn interface_routing_lowers_citing_108() {
        let spec = validate_operational(&routed_mobus_model())
            .expect("bert#108: interface routing lowers, it no longer refuses");
        let routed = spec
            .flows
            .iter()
            .find(|f| f.name == "well → tank")
            .expect("the routed flow projects");
        assert_eq!(
            routed.interface_routing,
            Some(InterfacePrimitive::Impeding),
            "the flow records its bert#108 Impeding lowering"
        );
        let plain = spec
            .flows
            .iter()
            .find(|f| f.name == "tank → drain")
            .unwrap();
        assert_eq!(
            plain.interface_routing, None,
            "an un-routed flow carries no lowering"
        );
    }

    #[test]
    fn interface_lowering_is_identity_default() {
        // Unparameterized, the lowering is observationally the flow attaching
        // directly: identical spec but for the provenance marker. This is what
        // "ZERO re-authoring" means — strip the marker and the two projections
        // are the same run.
        let plain = validate_operational(&mobus_model()).expect("projects");
        let routed = validate_operational(&routed_mobus_model()).expect("projects");
        let strip = |mut s: OperationalSpec| {
            for f in &mut s.flows {
                f.interface_routing = None;
            }
            s
        };
        assert_eq!(
            strip(plain),
            strip(routed),
            "identity lowering changes nothing but the provenance marker"
        );
    }

    #[test]
    fn content_hash_tracks_structural_change() {
        let a = validate_operational(&mobus_model()).expect("projects");
        let b = validate_operational(&mobus_model()).expect("projects");
        assert_eq!(a.content_hash(), b.content_hash(), "same spec, same hash");
        let mut m = mobus_model();
        m.systems[1].agent.as_mut().unwrap().primitive = Some(ProcessPrimitive::Propelling);
        let c = validate_operational(&m).expect("projects");
        assert_ne!(
            a.content_hash(),
            c.content_hash(),
            "a structural edit (changed primitive) moves the hash"
        );
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
