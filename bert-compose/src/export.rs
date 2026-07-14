//! Circuit ↔ BERT JSON: the canvas saves as an ordinary WorldModel that the
//! editor opens and the Mesa bridge can simulate — and loads one back
//! (`from_world_model`), completing the round-trip.
//!
//! Encoding follows the canonical pattern: each primitive node becomes an
//! Atomic subsystem carrying `AgentModel.primitive` (the same
//! encoding python/agents.py reads to pick its transfer function); wires
//! become internal flows; Source/Sink nodes become environment externals
//! whose flows connect to the wired subsystem directly. Compose-only knobs
//! ride in the model's extensible fields: a buffer's release rate in
//! `cognitive_params["release_rate"]`, a gradient wire's conductance as an
//! Interaction parameter — so nothing is lost on the way back.

use crate::circuit::{Circuit, DeclaredSubstance, FlowMode, Node, NodeKind, Wire};
use bert_core::{
    AgentKind, AgentModel, Boundary, Complexity, Environment, ExternalEntity, ExternalEntityType,
    Id, IdType, Info, Interaction, InteractionType, InteractionUsability, Parameter,
    ProcessPrimitive, Substance, System, Transform2d, WorldModel,
};
use egui::pos2;
use std::collections::HashMap;

/// Canvas px → model px on save; the inverse on load.
const SCALE: f32 = 0.6;

fn id(ty: IdType, indices: &[i64]) -> Id {
    Id {
        ty,
        indices: indices.to_vec(),
    }
}

fn info(i: Id, level: i32, name: &str, description: &str) -> Info {
    Info {
        id: i,
        level,
        name: name.to_string(),
        description: description.to_string(),
    }
}

fn transform(x: f32, y: f32) -> Option<Transform2d> {
    Some(Transform2d {
        translation: bert_core::Vec2::new(x, y),
        rotation: 0.0,
    })
}

pub fn to_world_model(circuit: &Circuit, name: &str) -> WorldModel {
    let mut systems: Vec<System> = Vec::new();
    let mut interactions: Vec<Interaction> = Vec::new();
    let mut environment = Environment {
        info: info(id(IdType::Environment, &[-1]), -1, "Environment", ""),
        sources: Vec::new(),
        sinks: Vec::new(),
    };

    // Composite root.
    systems.push(System {
        info: info(
            id(IdType::System, &[0]),
            0,
            name,
            "Built in bert-compose: process primitives wired into a working circuit \
             (composition is unconditional — Systems/Mobus/Composition.lean).",
        ),
        sources: Vec::new(),
        sinks: Vec::new(),
        parent: id(IdType::Environment, &[-1]),
        complexity: Complexity::Complex {
            adaptable: false,
            evolveable: false,
        },
        boundary: Boundary {
            info: info(id(IdType::Boundary, &[0]), 0, "", ""),
            porosity: 0.0,
            perceptive_fuzziness: 0.0,
            interfaces: Vec::new(),
            parent_interface: None,
        },
        radius: 400.0,
        transform: transform(0.0, 0.0),
        equivalence: String::new(),
        history: String::new(),
        transformation: String::new(),
        member_autonomy: 1.0,
        time_constant: String::new(),
        archetype: None,
        agent: None,
    });

    // Map node index → its model id (subsystem or environment external).
    let mut node_id: HashMap<usize, Id> = HashMap::new();
    let (mut sub_n, mut src_n, mut sink_n) = (0i64, 0i64, 0i64);

    for (i, node) in circuit.nodes.iter().enumerate() {
        let (x, y) = (node.pos.x * SCALE, node.pos.y * SCALE);
        match node.kind {
            NodeKind::Source => {
                let eid = id(IdType::Source, &[-1, src_n]);
                src_n += 1;
                environment.sources.push(ExternalEntity {
                    info: info(eid.clone(), -1, &node.name, "bert-compose source"),
                    ty: ExternalEntityType::Source,
                    transform: transform(x, y),
                    equivalence: String::new(),
                    model: String::new(),
                    is_same_as_id: None,
                });
                node_id.insert(i, eid);
            }
            NodeKind::Sink => {
                let eid = id(IdType::Sink, &[-1, sink_n]);
                sink_n += 1;
                environment.sinks.push(ExternalEntity {
                    info: info(eid.clone(), -1, &node.name, "bert-compose sink"),
                    ty: ExternalEntityType::Sink,
                    transform: transform(x, y),
                    equivalence: String::new(),
                    model: String::new(),
                    is_same_as_id: None,
                });
                node_id.insert(i, eid);
            }
            NodeKind::Process(primitive) => {
                let sid = id(IdType::Subsystem, &[0, sub_n]);
                sub_n += 1;
                systems.push(System {
                    info: info(
                        sid.clone(),
                        1,
                        &node.name,
                        &format!("{primitive:?} work process (Mobus atomic primitive)"),
                    ),
                    sources: Vec::new(),
                    sinks: Vec::new(),
                    parent: id(IdType::System, &[0]),
                    complexity: Complexity::Atomic,
                    boundary: Boundary {
                        info: info(id(IdType::Boundary, &[0, sub_n - 1]), 1, "", ""),
                        porosity: 0.0,
                        perceptive_fuzziness: 0.0,
                        interfaces: Vec::new(),
                        parent_interface: None,
                    },
                    radius: 50.0,
                    transform: transform(x, y),
                    equivalence: String::new(),
                    history: String::new(),
                    transformation: String::new(),
                    member_autonomy: 1.0,
                    time_constant: String::new(),
                    archetype: None,
                    agent: Some(AgentModel {
                        kind: AgentKind::Reactive,
                        agency_capacity: node.param,
                        primitive: Some(primitive),
                        // Compose knobs with no canonical home ride in the
                        // extensible params so the round-trip is lossless.
                        cognitive_params: {
                            let mut p = HashMap::new();
                            if primitive == ProcessPrimitive::Buffering {
                                p.insert("release_rate".to_string(), node.release_rate as f64);
                                if node.capacity > 0.0 {
                                    p.insert("capacity".to_string(), node.capacity as f64);
                                }
                                if node.time_constant > 0.0 {
                                    p.insert(
                                        "time_constant".to_string(),
                                        node.time_constant as f64,
                                    );
                                }
                                if node.maintenance > 0.0 {
                                    p.insert("maintenance".to_string(), node.maintenance as f64);
                                }
                            }
                            if primitive == ProcessPrimitive::Inverting && node.setpoint != 1.0 {
                                p.insert("setpoint".to_string(), node.setpoint as f64);
                            }
                            if primitive == ProcessPrimitive::Modulating && node.back_pressure {
                                p.insert("back_pressure".to_string(), 1.0);
                            }
                            p
                        },
                        process_configs: Vec::new(),
                        initial_state: if node.initial_storage > 0.0 {
                            HashMap::from([(
                                "storage".to_string(),
                                serde_json::json!(node.initial_storage),
                            )])
                        } else {
                            HashMap::new()
                        },
                        network_config: None,
                    }),
                });
                node_id.insert(i, sid);
            }
        }
    }

    for (k, wire) in circuit.wires.iter().enumerate() {
        let from = &circuit.nodes[wire.from];
        let substance = circuit.wire_substance(wire);
        let env_level = matches!(circuit.nodes[wire.from].kind, NodeKind::Source)
            || matches!(circuit.nodes[wire.to].kind, NodeKind::Sink);
        let usability = if matches!(circuit.nodes[wire.from].kind, NodeKind::Source) {
            InteractionUsability::Resource
        } else {
            InteractionUsability::Product
        };
        interactions.push(Interaction {
            info: info(
                id(IdType::Flow, &[if env_level { -1 } else { 0 }, k as i64]),
                if env_level { -1 } else { 1 },
                &format!("{} → {}", from.name, circuit.nodes[wire.to].name),
                "",
            ),
            // The declared substance name rides in sub_type — the kernel
            // field that existed for exactly this — over the conserved base.
            substance: Substance {
                sub_type: from.out_substance.name.clone(),
                ty: substance,
            },
            // Gradient (field-driven) flows export as BERT's Force interaction
            // — the redemption of InteractionType::Force: it now means "a flow
            // whose rate is a potential gradient," not a label without dynamics.
            ty: if wire.mode == crate::circuit::FlowMode::Gradient {
                InteractionType::Force
            } else {
                InteractionType::Flow
            },
            usability,
            source: node_id[&wire.from].clone(),
            source_interface: None,
            sink: node_id[&wire.to].clone(),
            sink_interface: None,
            // Source-fed flows carry the asserted emission rate.
            amount: bert_core::rust_decimal::Decimal::try_from(
                if matches!(circuit.nodes[wire.from].kind, NodeKind::Source) {
                    circuit.nodes[wire.from].param
                } else {
                    1.0
                },
            )
            .unwrap_or(bert_core::rust_decimal::Decimal::ONE),
            unit: from.out_substance.unit.clone(),
            // Gradient conductance (k) rides as a flow parameter.
            parameters: if wire.mode == FlowMode::Gradient {
                vec![Parameter {
                    name: "conductance".to_string(),
                    value: wire.conductance.to_string(),
                    ..Default::default()
                }]
            } else {
                Vec::new()
            },
            smart_parameters: Vec::new(),
            endpoint_offset: None,
        });
    }

    WorldModel {
        version: 1,
        // Compose's dynamics live in the engine, not the dynamical-face string
        // slots, so exports declare the Operational rung they actually occupy.
        mode: Some(bert_core::Mode::Operational),
        environment,
        systems,
        interactions,
        hidden_entities: Vec::new(),
    }
}

/// The model's display name = its root (level-0) system.
pub fn model_name(model: &WorldModel) -> String {
    model
        .systems
        .iter()
        .find(|s| s.info.level == 0)
        .map(|s| s.info.name.clone())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "Loaded model".to_string())
}

/// Build a circuit from the seam payload alone. Every executor-relevant datum
/// comes from the [`OperationalSpec`], so the loader cannot disagree with the
/// contract that produced it — agreement by construction, not by test.
/// Canvas-only concerns (node positions) are the caller's overlay.
pub fn from_spec(spec: &bert_core::operational::OperationalSpec) -> Circuit {
    let mut c = Circuit::default();
    let mut ids: Vec<(Id, usize)> = Vec::new();
    let grid = |i: usize| {
        pos2(
            380.0 + (i % 4) as f32 * 160.0,
            300.0 + (i / 4) as f32 * 140.0,
        )
    };

    for t in &spec.sources {
        let mut node = Node::new(NodeKind::Source, ids.len() + 1, grid(ids.len()));
        node.name = t.name.clone();
        ids.push((t.id.clone(), c.nodes.len()));
        c.nodes.push(node);
    }
    for t in &spec.sinks {
        let mut node = Node::new(NodeKind::Sink, ids.len() + 1, grid(ids.len()));
        node.name = t.name.clone();
        ids.push((t.id.clone(), c.nodes.len()));
        c.nodes.push(node);
    }
    for p in &spec.processes {
        let mut node = Node::new(
            NodeKind::Process(p.primitive),
            ids.len() + 1,
            grid(ids.len()),
        );
        node.name = p.name.clone();
        node.param = p.agency_capacity;
        if let Some(s) = p.initial_storage {
            node.initial_storage = s as f32;
            node.storage = s as f32;
        }
        if let Some(&r) = p.cognitive_params.get("release_rate") {
            node.release_rate = r as f32;
        }
        if let Some(&cap) = p.cognitive_params.get("capacity") {
            node.capacity = cap as f32;
        }
        if let Some(&sp) = p.cognitive_params.get("setpoint") {
            node.setpoint = sp as f32;
        }
        if let Some(&tc) = p.cognitive_params.get("time_constant") {
            node.time_constant = tc as f32;
        }
        if let Some(&m) = p.cognitive_params.get("maintenance") {
            node.maintenance = m as f32;
        }
        if p.cognitive_params.contains_key("back_pressure") {
            node.back_pressure = true;
        }
        ids.push((p.id.clone(), c.nodes.len()));
        c.nodes.push(node);
    }

    let idx_of = |id: &Id| ids.iter().find(|(i, _)| i == id).map(|(_, n)| *n);
    // How many pushed flows leave each resolved sender, and whether any
    // gradient flow does: a lone pushed outflow is the degenerate case where
    // node rate = wire rate, so `param` stays the carrier (and the inspector's
    // rate knob stays live); per-wire rates engage exactly when a source has
    // SEVERAL pushed outflows — where a single node rate cannot represent
    // them (bert#111).
    let mut pushed_out: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    let mut gradient_out: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for f in &spec.flows {
        if let Some(from) = idx_of(&f.source) {
            if f.conductance.is_some() {
                gradient_out.insert(from);
            } else {
                *pushed_out.entry(from).or_insert(0) += 1;
            }
        }
    }

    let mut substance_named: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for f in &spec.flows {
        // The contract guarantees endpoint resolution; a miss here would mean
        // a spec not produced by validate_operational.
        let (Some(from), Some(to)) = (idx_of(&f.source), idx_of(&f.sink)) else {
            continue;
        };
        let mut wire = match f.conductance {
            Some(k) => Wire::gradient(from, to, k as f32),
            None => Wire::new(from, to),
        };
        // Substance rides the wire — spec flows are quantified per-edge and a
        // multi-outflow sender's flows may differ (bert#111 sibling). The
        // FIRST flow also names the sender's display substance; later flows
        // must not overwrite it (the old last-wire-wins).
        wire.substance_override = Some(f.substance);
        if substance_named.insert(from) {
            c.nodes[from].out_substance = DeclaredSubstance {
                name: f.substance_name.clone(),
                base: f.substance,
                unit: f.unit.clone(),
            };
        }
        // Source-fed flows carry the asserted emission rate. Rate is an edge
        // attribute (Mobus Eq. 4.5) — with several pushed outflows each
        // amount rides ITS wire, so no rate collapses onto the node
        // (bert#111). A single pushed outflow keeps the old node-param
        // carrier (same behavior, live inspector knob), and a gradient
        // flow's amount is the source's fixed potential, genuinely per-node.
        if matches!(c.nodes[from].kind, NodeKind::Source) {
            match wire.mode {
                FlowMode::Pushed => {
                    // Series forcing (#16): an observed series governs emission
                    // per tick. It rides the WIRE (not the node param) so it
                    // survives even a lone outflow, and takes precedence over
                    // the scalar in the kernel (`wire_declared_rate`). The mean
                    // `amount` is still assigned below as the unforced fallback.
                    if let Some(series) = &f.rate_series {
                        wire.rate_series = Some(series.iter().map(|&v| v as f32).collect());
                    }
                    let lone_pushed = pushed_out.get(&from).copied().unwrap_or(0) == 1
                        && !gradient_out.contains(&from);
                    if lone_pushed {
                        c.nodes[from].param = f.amount as f32;
                    } else {
                        wire.rate = Some(f.amount as f32);
                    }
                }
                FlowMode::Gradient => c.nodes[from].param = f.amount as f32,
            }
        }
        c.wires.push(wire);
    }
    c
}

/// BERT JSON → Circuit: the inverse of [`to_world_model`], routed through the
/// seam contract. `validate_operational` either yields the spec — from which
/// [`from_spec`] builds the circuit — or the load fails with every refusal
/// reason. The loader has no projection logic of its own left to drift:
/// positions are the only thing read off the model here, because the canvas
/// cares where nodes sit and the contract deliberately does not.
pub fn from_world_model(model: &WorldModel) -> Result<Circuit, String> {
    let spec = bert_core::operational::validate_operational(model).map_err(|errs| {
        errs.iter()
            .map(|e| format!("{}: {}", e.location, e.reason))
            .collect::<Vec<_>>()
            .join("\n")
    })?;
    let mut c = from_spec(&spec);
    if c.nodes.is_empty() {
        return Err("model has no sources, sinks, or primitive subsystems".to_string());
    }

    // Canvas overlay: node order in from_spec mirrors the model's declaration
    // order (env sources, env sinks, level-1 systems), so positions zip.
    let transforms: Vec<&Option<Transform2d>> = model
        .environment
        .sources
        .iter()
        .map(|e| &e.transform)
        .chain(model.environment.sinks.iter().map(|e| &e.transform))
        .chain(
            model
                .systems
                .iter()
                .filter(|s| s.info.level > 0)
                .map(|s| &s.transform),
        )
        .collect();
    for (node, t) in c.nodes.iter_mut().zip(transforms) {
        if let Some(t) = t {
            node.pos = pos2(t.translation.x / SCALE, t.translation.y / SCALE);
        }
    }
    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::{Node, Wire};
    use bert_core::validate::{validate, Severity};
    use bert_core::ProcessPrimitive;
    use egui::pos2;

    /// bert#111: per-flow amounts survive the seam INTO EXECUTION. A source
    /// with two pushed, differently-quantified outflows keeps both rates on
    /// their wires (no last-wire-wins), the FIRST flow names the display
    /// substance, and the built circuit delivers each wire at its own rate.
    /// (The prior coverage stopped at the spec — "the imported amount reaches
    /// the spec" — which is exactly how the collapse went unseen.)
    #[test]
    fn from_spec_keeps_per_flow_source_rates_through_execution() {
        use bert_core::operational::{OperationalFlow, OperationalSpec, OperationalTerminal};
        use bert_core::{Id, IdType, SubstanceType};

        let id = |ty: IdType, n: i64| Id {
            ty,
            indices: vec![n],
        };
        let flow = |name: &str, snk: i64, amount: f64| OperationalFlow {
            name: name.into(),
            source: id(IdType::Source, 0),
            sink: id(IdType::Sink, snk),
            substance_name: name.into(),
            substance: SubstanceType::Material,
            unit: "tok/mo".into(),
            amount,
            conductance: None,
            interface_routing: None,
            rate_series: None,
        };
        let spec = OperationalSpec {
            processes: vec![],
            sources: vec![OperationalTerminal {
                id: id(IdType::Source, 0),
                name: "Provider".into(),
            }],
            sinks: vec![
                OperationalTerminal {
                    id: id(IdType::Sink, 0),
                    name: "Dev channel".into(),
                },
                OperationalTerminal {
                    id: id(IdType::Sink, 1),
                    name: "Enterprise channel".into(),
                },
            ],
            flows: vec![flow("dev tokens", 0, 12.0), flow("ent tokens", 1, 1.0)],
        };

        let mut c = from_spec(&spec);
        assert_eq!(c.wires[0].rate, Some(12.0), "first flow's rate on its wire");
        assert_eq!(c.wires[1].rate, Some(1.0), "second flow's rate survives");
        assert_eq!(
            c.nodes[0].out_substance.name, "dev tokens",
            "display substance named by the FIRST flow, not the last"
        );
        for _ in 0..3 {
            c.step();
        }
        assert!(
            (c.wire_amount(0) - 12.0).abs() < 1e-3,
            "declared rate EXECUTES: {}",
            c.wire_amount(0)
        );
        assert!(
            (c.wire_amount(1) - 1.0).abs() < 1e-3,
            "sibling keeps its own rate: {}",
            c.wire_amount(1)
        );
    }

    /// bert-lenses#16, the seam hop: a forced flow's observed series survives
    /// spec → circuit AND executes tick by tick. Complements the pure-circuit
    /// `forced_wire_delivers_series_tick_by_tick` — the #111 lesson is that a
    /// series can reach the wire yet fail to run, so both levels are tested.
    #[test]
    fn from_spec_carries_and_executes_a_forced_series() {
        use bert_core::operational::{OperationalFlow, OperationalSpec, OperationalTerminal};
        use bert_core::{Id, IdType, SubstanceType};

        let id = |ty: IdType, n: i64| Id { ty, indices: vec![n] };
        let spec = OperationalSpec {
            processes: vec![],
            sources: vec![OperationalTerminal {
                id: id(IdType::Source, 0),
                name: "Provider".into(),
            }],
            sinks: vec![OperationalTerminal {
                id: id(IdType::Sink, 0),
                name: "Dev channel".into(),
            }],
            flows: vec![OperationalFlow {
                name: "forced tokens".into(),
                source: id(IdType::Source, 0),
                sink: id(IdType::Sink, 0),
                substance_name: "tokens".into(),
                substance: SubstanceType::Material,
                unit: "tok/mo".into(),
                amount: 999.0, // the mean must NOT govern — the series does
                conductance: None,
                interface_routing: None,
                rate_series: Some(vec![4.0, 8.0, 0.5]),
            }],
        };

        let mut c = from_spec(&spec);
        assert_eq!(
            c.wires[0].rate_series,
            Some(vec![4.0, 8.0, 0.5]),
            "the series survives spec → circuit"
        );
        let expected = [4.0f32, 8.0, 0.5, 0.5]; // last value held past the horizon
        for (t, want) in expected.iter().enumerate() {
            c.step();
            assert!(
                (c.wire_amount(0) - want).abs() < 1e-3,
                "tick {t}: forced flow delivers {want}, got {}",
                c.wire_amount(0)
            );
        }
    }

    #[test]
    fn emitted_model_validates_and_round_trips() {
        let mut c = Circuit::default();
        c.nodes
            .push(Node::new(NodeKind::Source, 1, pos2(-200.0, 0.0)));
        c.nodes.push(Node::new(
            NodeKind::Process(ProcessPrimitive::Buffering),
            2,
            pos2(0.0, 0.0),
        ));
        c.nodes.push(Node::new(NodeKind::Sink, 3, pos2(200.0, 0.0)));
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::new(1, 2));
        c.nodes[0].param = 2.5; // asserted emission rate
        c.nodes[1].initial_storage = 12.0; // asserted starting stock
        c.nodes[0].out_substance = crate::circuit::DeclaredSubstance::named(
            "water",
            bert_core::SubstanceType::Material,
            "L",
        );

        let model = to_world_model(&c, "Touchable Circuit");
        let errors: Vec<_> = validate(&model)
            .issues
            .into_iter()
            .filter(|i| i.severity == Severity::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "emitted model must validate: {errors:#?}"
        );

        let json = serde_json::to_string_pretty(&model).unwrap();
        let reloaded: WorldModel = serde_json::from_str(&json).unwrap();
        assert_eq!(reloaded.systems.len(), 2); // root + buffer
        assert_eq!(reloaded.interactions.len(), 2);
        let agent = reloaded.systems[1]
            .agent
            .as_ref()
            .expect("primitive encoding");
        assert_eq!(agent.primitive, Some(ProcessPrimitive::Buffering));
        assert_eq!(
            agent.initial_state.get("storage").and_then(|v| v.as_f64()),
            Some(12.0),
            "asserted stock survives as initial_state (what Mesa seeds)"
        );
        let src_flow = &reloaded.interactions[0];
        assert_eq!(
            src_flow.amount.to_string(),
            "2.5",
            "emission rate on the flow"
        );
        assert_eq!(
            src_flow.substance.sub_type, "water",
            "declared name rides in sub_type"
        );
        assert_eq!(
            src_flow.substance.ty,
            bert_core::SubstanceType::Material,
            "over its base"
        );
        assert_eq!(src_flow.unit, "L", "declared unit on the interaction");
    }

    /// The seam as a contract: every model compose exports satisfies
    /// bert-core's `validate_operational`, the two gates agree on refusals,
    /// and the projected spec drives a circuit whose ledger balances.
    #[test]
    fn export_satisfies_the_seam_contract() {
        use bert_core::operational::validate_operational;

        let mut c = Circuit::default();
        c.nodes
            .push(Node::new(NodeKind::Source, 1, pos2(-200.0, 0.0)));
        c.nodes.push(Node::new(
            NodeKind::Process(ProcessPrimitive::Buffering),
            2,
            pos2(0.0, 0.0),
        ));
        c.nodes.push(Node::new(NodeKind::Sink, 3, pos2(200.0, 0.0)));
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::gradient(1, 2, 0.42));
        c.nodes[0].param = 2.5;
        c.nodes[1].initial_storage = 12.0;

        let mut model = to_world_model(&c, "Seam");
        let spec = validate_operational(&model).expect("compose exports satisfy the contract");
        assert_eq!(spec.processes.len(), 1);
        assert_eq!(spec.sources.len(), 1);
        assert_eq!(spec.sinks.len(), 1);
        assert_eq!(spec.flows.len(), 2);
        assert_eq!(
            spec.flows[1].conductance,
            Some(0.42),
            "gradient conductance crosses the seam typed"
        );

        // The projected model executes and conserves.
        let mut r = from_world_model(&model).expect("loads");
        for _ in 0..30 {
            r.step();
        }
        assert!(r.balance().abs() < 1e-3, "projected circuit conserves");

        // Representational rungs refuse: the hard gate, loud by design.
        model.mode = Some(bert_core::Mode::Core);
        assert!(
            validate_operational(&model).is_err(),
            "Klir rung must refuse execution"
        );
        model.mode = Some(bert_core::Mode::Structural);
        assert!(
            validate_operational(&model).is_err(),
            "Bunge rung must refuse execution"
        );
        model.mode = None;

        // Proto-gate agreement: what the contract refuses, the loader refuses.
        model.systems[1].agent = None;
        assert!(validate_operational(&model).is_err());
        assert!(from_world_model(&model).is_err());
    }

    /// An Inverting node's setpoint and a Modulating node's back-pressure flag
    /// survive the JSON round-trip (cognitive_params, same path as capacity).
    #[test]
    fn setpoint_and_backpressure_round_trip() {
        let mut c = Circuit::default();
        c.nodes.push(Node::new(
            NodeKind::Process(ProcessPrimitive::Inverting),
            1,
            pos2(0.0, 0.0),
        ));
        c.nodes.push(Node::new(
            NodeKind::Process(ProcessPrimitive::Modulating),
            2,
            pos2(60.0, 0.0),
        ));
        c.nodes.push(Node::new(NodeKind::Sink, 3, pos2(120.0, 0.0)));
        c.nodes[0].setpoint = 3.5;
        c.nodes[1].back_pressure = true;
        c.wires.push(Wire::new(0, 1)); // inverting → modulating (control)
        c.wires.push(Wire::new(1, 2)); // modulating → sink
        let model: WorldModel =
            serde_json::from_str(&serde_json::to_string(&to_world_model(&c, "SP")).unwrap())
                .unwrap();
        let r = from_world_model(&model).expect("loads");
        let inv = r
            .nodes
            .iter()
            .find(|n| n.kind == NodeKind::Process(ProcessPrimitive::Inverting))
            .expect("inverting survives");
        assert_eq!(inv.setpoint, 3.5, "setpoint survives via cognitive_params");
        let valve = r
            .nodes
            .iter()
            .find(|n| n.kind == NodeKind::Process(ProcessPrimitive::Modulating))
            .expect("modulating survives");
        assert!(
            valve.back_pressure,
            "back-pressure survives via cognitive_params"
        );
    }

    /// Save → Load round-trip: every knob the canvas can set survives —
    /// kinds, names, rates, stocks, release, substances, gradient mode and
    /// conductance — and the loaded circuit behaves identically.
    #[test]
    fn save_load_round_trip_is_lossless() {
        let mut c = Circuit::default();
        c.nodes
            .push(Node::new(NodeKind::Source, 1, pos2(-200.0, 0.0)));
        c.nodes.push(Node::new(
            NodeKind::Process(ProcessPrimitive::Buffering),
            2,
            pos2(0.0, 0.0),
        ));
        c.nodes.push(Node::new(
            NodeKind::Process(ProcessPrimitive::Buffering),
            3,
            pos2(120.0, 80.0),
        ));
        c.nodes.push(Node::new(NodeKind::Sink, 4, pos2(200.0, 0.0)));
        c.nodes[0].param = 2.5;
        c.nodes[0].out_substance =
            DeclaredSubstance::named("water", bert_core::SubstanceType::Material, "L");
        c.nodes[1].name = "Tank".to_string();
        c.nodes[1].initial_storage = 12.0;
        c.nodes[1].storage = 12.0;
        c.nodes[1].release_rate = 1.4;
        c.nodes[1].capacity = 20.0;
        c.nodes[1].time_constant = 4.0;
        c.nodes[1].maintenance = 0.3;
        c.nodes[1].out_substance =
            DeclaredSubstance::named("water", bert_core::SubstanceType::Material, "L");
        c.nodes[2].release_rate = 0.0;
        c.wires.push(Wire::new(0, 1));
        c.wires.push(Wire::gradient(1, 2, 0.42));
        c.wires.push(Wire::new(1, 3));

        let json = serde_json::to_string(&to_world_model(&c, "Round Trip")).unwrap();
        let model: WorldModel = serde_json::from_str(&json).unwrap();
        assert_eq!(model_name(&model), "Round Trip");
        let mut r = from_world_model(&model).expect("loads");

        assert_eq!(r.nodes.len(), 4);
        assert_eq!(r.wires.len(), 3);
        let tank = r
            .nodes
            .iter()
            .find(|n| n.name == "Tank")
            .expect("name survives");
        assert_eq!(tank.initial_storage, 12.0);
        assert_eq!(
            tank.release_rate, 1.4,
            "release rate survives via cognitive_params"
        );
        assert_eq!(
            tank.capacity, 20.0,
            "capacity survives via cognitive_params"
        );
        assert_eq!(
            tank.time_constant, 4.0,
            "time constant survives via cognitive_params"
        );
        assert_eq!(
            tank.maintenance, 0.3,
            "maintenance survives via cognitive_params"
        );
        assert_eq!(tank.out_substance.name, "water");
        assert_eq!(tank.out_substance.unit, "L");
        let grad = r
            .wires
            .iter()
            .find(|w| w.mode == FlowMode::Gradient)
            .expect("mode survives");
        assert_eq!(
            grad.conductance, 0.42,
            "conductance survives via flow parameter"
        );
        let src = r.nodes.iter().find(|n| n.kind == NodeKind::Source).unwrap();
        assert_eq!(src.param, 2.5, "emission rate survives");

        // Behavioral identity: same physics on both sides of the trip.
        // (Load reorders nodes — env entities first — so match by name.)
        for _ in 0..30 {
            c.step();
            r.step();
        }
        for a in &c.nodes {
            let b = r
                .nodes
                .iter()
                .find(|n| n.name == a.name)
                .expect("node survives");
            assert!(
                (a.storage - b.storage).abs() < 1e-4 && (a.total - b.total).abs() < 1e-4,
                "loaded circuit diverges at {}: {} vs {}",
                a.name,
                a.storage,
                b.storage
            );
        }
        assert!(r.balance().abs() < 1e-3, "loaded circuit conserves");
    }
}
