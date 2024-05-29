use crate::components::*;
use crate::data_model::Interaction;
use crate::data_model::*;
use crate::plugins::file_dialog::ExportFileEvent;
use bevy::core::Name;
use bevy::prelude::*;
use bevy::utils::HashMap;

struct Context {
    parent_id_to_count: HashMap<Id, i64>,
    entity_to_id: HashMap<Entity, Id>,
    interactions: Vec<Interaction>,
    entity_to_interaction_idx: HashMap<Entity, usize>,
}

impl Context {
    fn new() -> Self {
        Self {
            parent_id_to_count: HashMap::new(),
            entity_to_id: HashMap::new(),
            interactions: vec![],
            entity_to_interaction_idx: HashMap::new(),
        }
    }

    fn next_id(&mut self, entity: Entity, ty: IdType, parent_idx: &[i64]) -> Id {
        let count = self
            .parent_id_to_count
            .entry(Id {
                ty,
                indices: parent_idx.to_vec(),
            })
            .or_insert(0);

        let mut indices = parent_idx.to_vec();
        indices.push(*count);

        *count += 1;

        let id = Id { ty, indices };
        self.entity_to_id.insert(entity, id.clone());
        id
    }

    fn interaction_mut_by_entity(&mut self, entity: Entity) -> &mut Interaction {
        let idx = self.entity_to_interaction_idx[&entity];
        &mut self.interactions[idx]
    }
}

pub fn save_world(
    mut save_file_event_reader: EventReader<ExportFileEvent>,
    name_and_description_query: Query<(&Name, &ElementDescription)>,
    transform_query: Query<(&Transform, &InitialPosition)>,
    parent_query: Query<&Parent>,
    main_system_info_query: Query<
        (Entity, &crate::components::System, &SystemEnvironment),
        Without<Subsystem>,
    >,
    subsystem_query: Query<(Entity, &crate::components::System, &Subsystem)>,
    flow_query: Query<(
        Entity,
        &Flow,
        &FlowStartConnection,
        &FlowEndConnection,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
    )>,
    interface_query: Query<(&crate::components::Interface, &Transform)>,
    external_entity_query: Query<&crate::components::ExternalEntity>,
) {
    for event in save_file_event_reader.read() {
        let (system_entity, system_component, environment) = main_system_info_query
            .get_single()
            .expect("System of interest should exist");

        let mut ctx = Context::new();
        let mut entity_to_system = HashMap::<Entity, crate::data_model::System>::new();

        let mut environment = Environment {
            info: Info {
                id: Id {
                    ty: IdType::Environment,
                    indices: vec![-1],
                },
                name: environment.name.clone(),
                description: environment.description.clone(),
                level: -1,
            },
            sources: vec![],
            sinks: vec![],
        };

        build_system(
            system_entity,
            system_component,
            Id {
                ty: IdType::System,
                indices: vec![0],
            },
            0,
            environment.info.id.clone(),
            None,
            &name_and_description_query,
            &transform_query,
            &mut ctx,
            &mut entity_to_system,
        );

        let system = entity_to_system
            .get_mut(&system_entity)
            .expect("Should exist");

        build_interfaces_interaction_and_external_entities(
            &mut ctx,
            system_entity,
            system,
            &mut environment,
            &name_and_description_query,
            &transform_query,
            &flow_query,
            &interface_query,
            &external_entity_query,
        );

        // TODO : connect interaction interface links

        build_subsystems(
            &mut ctx,
            system_entity,
            &name_and_description_query,
            &transform_query,
            &parent_query,
            &subsystem_query,
            &flow_query,
            &interface_query,
            &external_entity_query,
            &mut entity_to_system,
        );

        for (
            flow_entity,
            _,
            _,
            _,
            flow_start_interface_connection,
            flow_end_interface_connection,
        ) in &flow_query
        {
            let source_interface =
                flow_start_interface_connection.map(|c| ctx.entity_to_id[&c.target].clone());
            let sink_interface =
                flow_end_interface_connection.map(|c| ctx.entity_to_id[&c.target].clone());

            let interaction = ctx.interaction_mut_by_entity(flow_entity);
            interaction.source_interface = source_interface;
            interaction.sink_interface = sink_interface;
        }

        let model = WorldModel {
            version: CURRENT_FILE_VERSION,
            systems: entity_to_system.into_values().collect(),
            interactions: ctx.interactions,
            environment,
        };

        let save_file = &**event;

        save_to_json(&model, save_file.to_str().unwrap());
    }
}

fn build_subsystems(
    mut ctx: &mut Context,
    parent_system_entity: Entity,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
    transform_query: &Query<(&Transform, &InitialPosition)>,
    parent_query: &Query<&Parent>,
    subsystem_query: &Query<(Entity, &crate::components::System, &Subsystem)>,
    flow_query: &Query<(
        Entity,
        &Flow,
        &FlowStartConnection,
        &FlowEndConnection,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
    )>,
    interface_query: &Query<(&crate::components::Interface, &Transform)>,
    external_entity_query: &Query<&crate::components::ExternalEntity>,
    mut entity_to_system: &mut HashMap<Entity, System>,
) {
    let mut not_interface_subsystems = vec![];
    let mut system_entities = vec![];

    for (subsystem_entity, system_component, subsystem) in subsystem_query {
        if subsystem.parent_system == parent_system_entity {
            system_entities.push(subsystem_entity);

            let mut parent_entity = parent_query
                .get(subsystem_entity)
                .expect("Subsystem should have a parent")
                .get();

            if interface_query.get(parent_entity).is_ok() {
                let interface_id = &ctx.entity_to_id[&parent_entity];
                let indices = interface_id.indices.clone();
                let (last_index, parent_indices) = indices.split_last().expect("Should exist");

                ctx.parent_id_to_count
                    .entry(Id {
                        ty: IdType::Subsystem,
                        indices: parent_indices.to_vec(),
                    })
                    .and_modify(|count| *count = (*count).max(last_index + 1))
                    .or_insert(last_index + 1);

                let system = entity_to_system
                    .get_mut(&parent_system_entity)
                    .expect("Should exist");

                let level = system.info.level + 1;
                let parent_id = system.info.id.clone();

                build_system(
                    subsystem_entity,
                    system_component,
                    Id {
                        ty: IdType::Subsystem,
                        indices,
                    },
                    level,
                    parent_id,
                    Some(interface_id.clone()),
                    &name_and_description_query,
                    &transform_query,
                    &mut ctx,
                    &mut entity_to_system,
                );
            } else {
                let mut parent_interface_id = None;

                while let Ok(parent) = parent_query.get(parent_entity) {
                    parent_entity = parent.get();

                    if interface_query.get(parent_entity).is_ok() {
                        parent_interface_id = Some(ctx.entity_to_id[&parent_entity].clone());
                        break;
                    }
                }

                not_interface_subsystems.push((
                    subsystem_entity,
                    system_component,
                    parent_interface_id,
                ));
            }
        }
    }

    for (subsystem_entity, system_component, parent_interface_id) in not_interface_subsystems {
        let system = entity_to_system
            .get_mut(&parent_system_entity)
            .expect("Should exist");

        let id = ctx.next_id(subsystem_entity, IdType::Subsystem, &system.info.id.indices);

        let level = system.info.level + 1;
        let parent_id = system.info.id.clone();

        build_system(
            subsystem_entity,
            system_component,
            id,
            level,
            parent_id,
            parent_interface_id,
            name_and_description_query,
            transform_query,
            ctx,
            &mut entity_to_system,
        );
    }

    let mut parent_system = entity_to_system
        .remove(&parent_system_entity)
        .expect("Should exist");

    for system_entity in &system_entities {
        let system = entity_to_system
            .get_mut(system_entity)
            .expect("Should exist");

        build_interfaces_interaction_and_external_entities(
            ctx,
            *system_entity,
            system,
            &mut parent_system,
            name_and_description_query,
            transform_query,
            flow_query,
            interface_query,
            external_entity_query,
        );
    }

    entity_to_system.insert(parent_system_entity, parent_system);

    for system_entity in system_entities {
        build_subsystems(
            ctx,
            system_entity,
            name_and_description_query,
            transform_query,
            parent_query,
            subsystem_query,
            flow_query,
            interface_query,
            external_entity_query,
            entity_to_system,
        );
    }
}

fn build_interfaces_interaction_and_external_entities<P: HasInfo + HasSourcesAndSinks>(
    ctx: &mut Context,
    system_entity: Entity,
    system: &mut System,
    parent: &mut P,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
    transform_query: &Query<(&Transform, &InitialPosition)>,
    flow_query: &Query<(
        Entity,
        &Flow,
        &FlowStartConnection,
        &FlowEndConnection,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
    )>,
    interface_query: &Query<(&crate::components::Interface, &Transform)>,
    external_entity_query: &Query<&crate::components::ExternalEntity>,
) {
    for (
        flow_entity,
        flow,
        flow_start_connection,
        flow_end_connection,
        flow_start_interface_connection,
        flow_end_interface_connection,
    ) in flow_query
    {
        if flow_start_connection.target == system_entity {
            let interface_index =
                if let Some(interface_connection) = flow_start_interface_connection {
                    build_interface(
                        ctx,
                        crate::data_model::InterfaceType::Export,
                        system,
                        interface_connection,
                        name_and_description_query,
                        interface_query,
                    )
                } else {
                    continue;
                };

            let sink_id = match flow_end_connection.target_type {
                EndTargetType::Sink => {
                    let sink_entity = flow_end_connection.target;

                    if let Some(id) = ctx.entity_to_id.get(&sink_entity) {
                        id.clone()
                    } else {
                        let sink = build_external_entity(
                            ctx,
                            sink_entity,
                            ExternalEntityType::Sink,
                            IdType::Sink,
                            &*parent,
                            name_and_description_query,
                            transform_query,
                            external_entity_query,
                        );

                        let id = sink.info.id.clone();
                        parent.sinks_mut().push(sink);
                        id
                    }
                }
                EndTargetType::System => ctx.entity_to_id[&flow_end_connection.target].clone(),
            };

            system.boundary.interfaces[interface_index]
                .exports_to
                .push(sink_id.clone());

            if !ctx.entity_to_id.contains_key(&flow_entity) {
                build_interaction(
                    ctx,
                    flow_entity,
                    flow,
                    parent,
                    system.info.id.clone(),
                    sink_id.clone(),
                    name_and_description_query,
                );
            }
        } else if flow_end_connection.target == system_entity {
            let interface_index = if let Some(interface_connection) = flow_end_interface_connection
            {
                build_interface(
                    ctx,
                    crate::data_model::InterfaceType::Import,
                    system,
                    interface_connection,
                    name_and_description_query,
                    interface_query,
                )
            } else {
                continue;
            };

            let source_id = match flow_start_connection.target_type {
                StartTargetType::Source => {
                    let source_entity = flow_start_connection.target;

                    if let Some(id) = ctx.entity_to_id.get(&source_entity) {
                        id.clone()
                    } else {
                        let source = build_external_entity(
                            ctx,
                            source_entity,
                            ExternalEntityType::Source,
                            IdType::Source,
                            &*parent,
                            name_and_description_query,
                            transform_query,
                            external_entity_query,
                        );

                        let id = source.info.id.clone();
                        parent.sources_mut().push(source);
                        id
                    }
                }
                StartTargetType::System => ctx.entity_to_id[&flow_start_connection.target].clone(),
            };

            system.boundary.interfaces[interface_index]
                .receives_from
                .push(source_id.clone());

            if !ctx.entity_to_id.contains_key(&flow_entity) {
                build_interaction(
                    ctx,
                    flow_entity,
                    flow,
                    parent,
                    source_id.clone(),
                    system.info.id.clone(),
                    name_and_description_query,
                );
            }
        }
    }
}

fn build_interface<C: Connection>(
    ctx: &mut Context,
    ty: crate::data_model::InterfaceType,
    system: &mut System,
    interface_connection: &C,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
    interface_query: &Query<(&crate::components::Interface, &Transform)>,
) -> usize {
    let interface_entity = interface_connection.target();

    let (interface, interface_transform) =
        interface_query.get(interface_entity).expect("Should exist");

    system
        .boundary
        .interfaces
        .push(crate::data_model::Interface {
            info: info_from_entity(
                interface_entity,
                ctx.next_id(interface_entity, IdType::Interface, &system.info.id.indices),
                system.info.level + 1,
                &name_and_description_query,
            ),
            protocol: interface.protocol.clone(),
            ty, // TODO : hybrid
            exports_to: vec![],
            receives_from: vec![],
            angle: Some(interface_transform.right().truncate().to_angle()),
        });

    system.boundary.interfaces.len() - 1
}

fn build_interaction<P: HasInfo>(
    ctx: &mut Context,
    flow_entity: Entity,
    flow: &Flow,
    parent: &P,
    source_id: Id,
    sink_id: Id,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
) {
    let parent_level = parent.info().level;

    let interaction = Interaction {
        info: info_from_entity(
            flow_entity,
            ctx.next_id(flow_entity, IdType::Flow, &parent.info().id.indices),
            if parent_level == -1 {
                -1
            } else {
                parent_level + 1
            },
            &name_and_description_query,
        ),
        substance: Substance {
            sub_type: flow.substance_sub_type.clone(),
            ty: flow.substance_type,
        },
        ty: flow.interaction_type,
        usability: flow.usability,
        source: source_id,
        sink: sink_id,
        source_interface: None,
        sink_interface: None,
        amount: flow.amount,
        unit: flow.unit.clone(),
        parameters: flow.parameters.clone(),
    };

    ctx.interactions.push(interaction);
    ctx.entity_to_interaction_idx
        .insert(flow_entity, ctx.interactions.len() - 1);
}

fn build_external_entity<P: HasInfo>(
    ctx: &mut Context,
    entity: Entity,
    ty: ExternalEntityType,
    id_type: IdType,
    parent: &P,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
    transform_query: &Query<(&Transform, &InitialPosition)>,
    external_entity_query: &Query<&crate::components::ExternalEntity>,
) -> crate::data_model::ExternalEntity {
    let id = ctx.next_id(entity, id_type, &parent.info().id.indices);

    let external_entity_component = external_entity_query.get(entity).expect("Should exist");

    let parent_level = parent.info().level;

    crate::data_model::ExternalEntity {
        info: info_from_entity(
            entity,
            id.clone(),
            if parent_level == -1 {
                -1
            } else {
                parent_level + 1
            },
            &name_and_description_query,
        ),
        ty,
        transform: transform2d_from_entity(entity, &transform_query),
        equivalence: external_entity_component.equivalence.clone(),
        model: external_entity_component.model.clone(),
    }
}

fn build_system(
    system_entity: Entity,
    system: &crate::components::System,
    id: Id,
    level: i32,
    parent_id: Id,
    parent_interface: Option<Id>,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
    transform_query: &Query<(&Transform, &InitialPosition)>,
    ctx: &mut Context,
    entities_to_systems: &mut HashMap<Entity, System>,
) {
    let boundary = Boundary {
        info: Info {
            id: Id {
                ty: IdType::Boundary,
                indices: id.indices.clone(),
            },
            level,
            name: system.boundary.name.clone(),
            description: system.boundary.description.clone(),
        },
        porosity: system.boundary.porosity,
        perceptive_fuzziness: system.boundary.perceptive_fuzziness,
        interfaces: vec![],
        parent_interface,
    };

    let root_system = crate::data_model::System {
        info: info_from_entity(system_entity, id, level, &name_and_description_query),
        parent: parent_id,
        complexity: system.complexity,
        boundary,
        sources: vec![], // TODO
        sinks: vec![],   // TODO
        radius: system.radius,
        transform: transform2d_from_entity(system_entity, transform_query),
        equivalence: system.equivalence.clone(),
        history: system.history.clone(),
        transformation: system.transformation.clone(),
        member_autonomy: 1.0,
        time_constant: system.time_unit.clone(),
    };

    ctx.entity_to_id
        .insert(system_entity, root_system.info.id.clone());
    entities_to_systems.insert(system_entity, root_system);
}

fn transform2d_from_entity(
    entity: Entity,
    transform_query: &Query<(&Transform, &InitialPosition)>,
) -> Option<Transform2d> {
    Some(Transform2d::from(
        transform_query
            .get(entity)
            .expect("System should have transforms"),
    ))
}

fn info_from_entity(
    entity: Entity,
    id: Id,
    level: i32,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
) -> Info {
    let (name, description) = name_and_description_query
        .get(entity)
        .expect("Should exist");

    Info {
        id,
        level,
        name: name.to_string(),
        description: description.text.clone(),
    }
}

fn save_to_json(world_model: &WorldModel, file_name: &str) {
    let json = serde_json::to_string(world_model).expect("This shouldn't fail");
    std::fs::write(file_name, json).expect("This shouldn't fail");
}
