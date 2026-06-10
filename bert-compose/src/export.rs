//! Circuit → BERT JSON: the canvas saves as an ordinary WorldModel that the
//! editor opens and the Mesa bridge can simulate.
//!
//! Encoding follows the canonical pattern: each primitive node becomes an
//! Atomic subsystem carrying `AgentModel.primitives = [primitive]` (the same
//! encoding python/agents.py reads to pick its transfer function); wires
//! become internal flows; Source/Sink nodes become environment externals
//! whose flows connect to the wired subsystem directly.

use crate::circuit::{Circuit, NodeKind};
use bert_core::{
    AgentKind, AgentModel, Boundary, Complexity, Environment, ExternalEntity,
    ExternalEntityType, Id, IdType, Info, Interaction, InteractionType, InteractionUsability,
    Substance, System, Transform2d, WorldModel,
};
use std::collections::HashMap;

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

    let scale = 0.6; // canvas px → model px
    for (i, node) in circuit.nodes.iter().enumerate() {
        let (x, y) = (node.pos.x * scale, node.pos.y * scale);
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
                        cognitive_params: HashMap::new(),
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
            substance: Substance { sub_type: String::new(), ty: substance },
            ty: InteractionType::Flow,
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
            unit: String::new(),
            parameters: Vec::new(),
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
        c.wires.push(Wire { from: 0, to: 1 });
        c.wires.push(Wire { from: 1, to: 2 });
        c.nodes[0].param = 2.5; // asserted emission rate
        c.nodes[1].initial_storage = 12.0; // asserted starting stock

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
    }
}
