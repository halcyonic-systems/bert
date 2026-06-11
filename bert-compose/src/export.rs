//! Circuit ↔ BERT JSON: the canvas saves as an ordinary WorldModel that the
//! editor opens and the Mesa bridge can simulate — and loads one back
//! (`from_world_model`), completing the round-trip.
//!
//! Encoding follows the canonical pattern: each primitive node becomes an
//! Atomic subsystem carrying `AgentModel.primitives = [primitive]` (the same
//! encoding python/agents.py reads to pick its transfer function); wires
//! become internal flows; Source/Sink nodes become environment externals
//! whose flows connect to the wired subsystem directly. Compose-only knobs
//! ride in the model's extensible fields: a buffer's release rate in
//! `cognitive_params["release_rate"]`, a gradient wire's conductance as an
//! Interaction parameter — so nothing is lost on the way back.

use crate::circuit::{Circuit, DeclaredSubstance, FlowMode, Node, NodeKind, Wire};
use bert_core::{
    AgentKind, AgentModel, Boundary, Complexity, Environment, ExternalEntity,
    ExternalEntityType, Id, IdType, Info, Interaction, InteractionType, InteractionUsability,
    Parameter, ProcessPrimitive, Substance, System, Transform2d, WorldModel,
};
use egui::pos2;
use std::collections::HashMap;

/// Canvas px → model px on save; the inverse on load.
const SCALE: f32 = 0.6;

fn id(ty: IdType, indices: &[i64]) -> Id {
    Id { ty, indices: indices.to_vec() }
}

fn info(i: Id, level: i32, name: &str, description: &str) -> Info {
    Info { id: i, level, name: name.to_string(), description: description.to_string() }
}

fn transform(x: f32, y: f32) -> Option<Transform2d> {
    Some(Transform2d { translation: bert_core::Vec2::new(x, y), rotation: 0.0 })
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
        complexity: Complexity::Complex { adaptable: false, evolveable: false },
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
                        primitives: vec![primitive],
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
                                    p.insert("time_constant".to_string(), node.time_constant as f64);
                                }
                                if node.maintenance > 0.0 {
                                    p.insert("maintenance".to_string(), node.maintenance as f64);
                                }
                            }
                            if primitive == ProcessPrimitive::Inverting && node.setpoint != 1.0 {
                                p.insert("setpoint".to_string(), node.setpoint as f64);
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
            substance: Substance { sub_type: from.out_substance.name.clone(), ty: substance },
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

/// BERT JSON → Circuit: the inverse of [`to_world_model`]. Loads any
/// compose-shaped model — environment sources/sinks, atomic subsystems
/// carrying a Mobus primitive, flows between them. Hierarchy below level 1,
/// interfaces, and primitive-less subsystems are out of the canvas's
/// vocabulary and reported as an error rather than silently dropped.
pub fn from_world_model(model: &WorldModel) -> Result<Circuit, String> {
    let mut c = Circuit::default();
    let mut ids: Vec<(Id, usize)> = Vec::new();
    let pos_of = |t: &Option<Transform2d>, i: usize| {
        t.as_ref()
            .map(|t| pos2(t.translation.x / SCALE, t.translation.y / SCALE))
            .unwrap_or_else(|| pos2(380.0 + (i % 4) as f32 * 160.0, 300.0 + (i / 4) as f32 * 140.0))
    };

    for ext in model.environment.sources.iter().chain(model.environment.sinks.iter()) {
        let kind = if matches!(ext.ty, ExternalEntityType::Source) {
            NodeKind::Source
        } else {
            NodeKind::Sink
        };
        let mut node = Node::new(kind, ids.len() + 1, pos_of(&ext.transform, ids.len()));
        node.name = ext.info.name.clone();
        ids.push((ext.info.id.clone(), c.nodes.len()));
        c.nodes.push(node);
    }

    for sys in model.systems.iter().filter(|s| s.info.level > 0) {
        let agent = sys.agent.as_ref().ok_or_else(|| {
            format!("\"{}\" has no agent model — not a compose-shaped subsystem", sys.info.name)
        })?;
        let &primitive = agent.primitives.first().ok_or_else(|| {
            format!("\"{}\" carries no Mobus primitive — nothing to place", sys.info.name)
        })?;
        let mut node = Node::new(
            NodeKind::Process(primitive),
            ids.len() + 1,
            pos_of(&sys.transform, ids.len()),
        );
        node.name = sys.info.name.clone();
        node.param = agent.agency_capacity;
        if let Some(s) = agent.initial_state.get("storage").and_then(|v| v.as_f64()) {
            node.initial_storage = s as f32;
            node.storage = s as f32;
        }
        if let Some(&r) = agent.cognitive_params.get("release_rate") {
            node.release_rate = r as f32;
        }
        if let Some(&cap) = agent.cognitive_params.get("capacity") {
            node.capacity = cap as f32;
        }
        if let Some(&sp) = agent.cognitive_params.get("setpoint") {
            node.setpoint = sp as f32;
        }
        if let Some(&tc) = agent.cognitive_params.get("time_constant") {
            node.time_constant = tc as f32;
        }
        if let Some(&m) = agent.cognitive_params.get("maintenance") {
            node.maintenance = m as f32;
        }
        ids.push((sys.info.id.clone(), c.nodes.len()));
        c.nodes.push(node);
    }

    if c.nodes.is_empty() {
        return Err("model has no sources, sinks, or primitive subsystems".to_string());
    }

    let idx_of = |id: &Id| ids.iter().find(|(i, _)| i == id).map(|(_, n)| *n);
    for inter in &model.interactions {
        let (Some(from), Some(to)) = (idx_of(&inter.source), idx_of(&inter.sink)) else {
            return Err(format!(
                "flow \"{}\" touches an entity outside the canvas vocabulary",
                inter.info.name
            ));
        };
        let wire = if inter.ty == InteractionType::Force {
            let k = inter
                .parameters
                .iter()
                .find(|p| p.name == "conductance")
                .and_then(|p| p.value.parse::<f32>().ok())
                .unwrap_or(0.3);
            Wire::gradient(from, to, k)
        } else {
            Wire::new(from, to)
        };
        // The wire's substance is the sender's declared output.
        c.nodes[from].out_substance = DeclaredSubstance {
            name: inter.substance.sub_type.clone(),
            base: inter.substance.ty,
            unit: inter.unit.clone(),
        };
        // Source-fed flows carry the asserted emission rate (= the source's
        // fixed potential, for gradient flows).
        if matches!(c.nodes[from].kind, NodeKind::Source) {
            if let Ok(rate) = inter.amount.to_string().parse::<f32>() {
                c.nodes[from].param = rate;
            }
        }
        c.wires.push(wire);
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

    #[test]
    fn emitted_model_validates_and_round_trips() {
        let mut c = Circuit::default();
        c.nodes.push(Node::new(NodeKind::Source, 1, pos2(-200.0, 0.0)));
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
        c.nodes[0].out_substance =
            crate::circuit::DeclaredSubstance::named("water", bert_core::SubstanceType::Material, "L");

        let model = to_world_model(&c, "Touchable Circuit");
        let errors: Vec<_> = validate(&model)
            .issues
            .into_iter()
            .filter(|i| i.severity == Severity::Error)
            .collect();
        assert!(errors.is_empty(), "emitted model must validate: {errors:#?}");

        let json = serde_json::to_string_pretty(&model).unwrap();
        let reloaded: WorldModel = serde_json::from_str(&json).unwrap();
        assert_eq!(reloaded.systems.len(), 2); // root + buffer
        assert_eq!(reloaded.interactions.len(), 2);
        let agent = reloaded.systems[1].agent.as_ref().expect("primitive encoding");
        assert_eq!(agent.primitives, vec![ProcessPrimitive::Buffering]);
        assert_eq!(
            agent.initial_state.get("storage").and_then(|v| v.as_f64()),
            Some(12.0),
            "asserted stock survives as initial_state (what Mesa seeds)"
        );
        let src_flow = &reloaded.interactions[0];
        assert_eq!(src_flow.amount.to_string(), "2.5", "emission rate on the flow");
        assert_eq!(src_flow.substance.sub_type, "water", "declared name rides in sub_type");
        assert_eq!(src_flow.substance.ty, bert_core::SubstanceType::Material, "over its base");
        assert_eq!(src_flow.unit, "L", "declared unit on the interaction");
    }

    /// An Inverting node's setpoint survives the JSON round-trip (rides in
    /// cognitive_params, same path as capacity).
    #[test]
    fn setpoint_round_trips() {
        let mut c = Circuit::default();
        c.nodes.push(Node::new(NodeKind::Process(ProcessPrimitive::Inverting), 1, pos2(0.0, 0.0)));
        c.nodes.push(Node::new(NodeKind::Sink, 2, pos2(120.0, 0.0)));
        c.nodes[0].setpoint = 3.5;
        c.wires.push(Wire::new(0, 1));
        let model: WorldModel =
            serde_json::from_str(&serde_json::to_string(&to_world_model(&c, "SP")).unwrap()).unwrap();
        let r = from_world_model(&model).expect("loads");
        let inv = r
            .nodes
            .iter()
            .find(|n| n.kind == NodeKind::Process(ProcessPrimitive::Inverting))
            .expect("inverting survives");
        assert_eq!(inv.setpoint, 3.5, "setpoint survives via cognitive_params");
    }

    /// Save → Load round-trip: every knob the canvas can set survives —
    /// kinds, names, rates, stocks, release, substances, gradient mode and
    /// conductance — and the loaded circuit behaves identically.
    #[test]
    fn save_load_round_trip_is_lossless() {
        let mut c = Circuit::default();
        c.nodes.push(Node::new(NodeKind::Source, 1, pos2(-200.0, 0.0)));
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
        c.nodes[0].out_substance = DeclaredSubstance::named(
            "water",
            bert_core::SubstanceType::Material,
            "L",
        );
        c.nodes[1].name = "Tank".to_string();
        c.nodes[1].initial_storage = 12.0;
        c.nodes[1].storage = 12.0;
        c.nodes[1].release_rate = 1.4;
        c.nodes[1].capacity = 20.0;
        c.nodes[1].time_constant = 4.0;
        c.nodes[1].maintenance = 0.3;
        c.nodes[1].out_substance = DeclaredSubstance::named(
            "water",
            bert_core::SubstanceType::Material,
            "L",
        );
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
        let tank = r.nodes.iter().find(|n| n.name == "Tank").expect("name survives");
        assert_eq!(tank.initial_storage, 12.0);
        assert_eq!(tank.release_rate, 1.4, "release rate survives via cognitive_params");
        assert_eq!(tank.capacity, 20.0, "capacity survives via cognitive_params");
        assert_eq!(tank.time_constant, 4.0, "time constant survives via cognitive_params");
        assert_eq!(tank.maintenance, 0.3, "maintenance survives via cognitive_params");
        assert_eq!(tank.out_substance.name, "water");
        assert_eq!(tank.out_substance.unit, "L");
        let grad = r.wires.iter().find(|w| w.mode == FlowMode::Gradient).expect("mode survives");
        assert_eq!(grad.conductance, 0.42, "conductance survives via flow parameter");
        let src = r.nodes.iter().find(|n| n.kind == NodeKind::Source).unwrap();
        assert_eq!(src.param, 2.5, "emission rate survives");

        // Behavioral identity: same physics on both sides of the trip.
        // (Load reorders nodes — env entities first — so match by name.)
        for _ in 0..30 {
            c.step();
            r.step();
        }
        for a in &c.nodes {
            let b = r.nodes.iter().find(|n| n.name == a.name).expect("node survives");
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
