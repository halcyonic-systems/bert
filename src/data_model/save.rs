use crate::components::*;
use crate::data_model::Interaction;
use crate::data_model::*;
use bevy::core::Name;
use bevy::prelude::*;
use bevy::utils::HashMap;

struct Context {
    parent_id_to_count: HashMap<Id, i64>,
    entity_to_id: HashMap<Entity, Id>,
    interactions: Vec<Interaction>,
}

impl Context {
    fn new() -> Self {
        Self {
            parent_id_to_count: HashMap::new(),
            entity_to_id: HashMap::new(),
            interactions: vec![],
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
}

pub fn save_world(
    state: Res<State<FileExportState>>,
    mut next_state: ResMut<NextState<FileExportState>>,
    save_file_query: Query<&SaveFile>,
    name_and_description_query: Query<(&Name, &ElementDescription)>,
    transform_query: Query<(&Transform, &InitialPosition)>,
    children_query: Query<&Children>,
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
    let (system_entity, system_component, environment) = main_system_info_query
        .get_single()
        .expect("System of interest should exist");

    let mut sources = vec![];
    let mut sinks = vec![];

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
        sources,
        sinks,
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

    let mut not_interface_subsystems = vec![];

    for (subsystem_entity, system_component, subsystem) in &subsystem_query {
        if subsystem.parent_system == system_entity {
            let parent_entity = parent_query
                .get(subsystem_entity)
                .expect("Subsystem should have a parent")
                .get();

            if interface_query.get(parent_entity).is_ok() {
                let indices = ctx.entity_to_id[&parent_entity].indices.clone();
                let (last_index, parent_indices) = indices.split_last().expect("Should exist");

                ctx.parent_id_to_count
                    .entry(Id {
                        ty: IdType::Subsystem,
                        indices: parent_indices.to_vec(),
                    })
                    .and_modify(|count| *count = (*count).max(last_index + 1))
                    .or_insert(last_index + 1);

                let system = entity_to_system
                    .get_mut(&system_entity)
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
                    &name_and_description_query,
                    &transform_query,
                    &mut ctx,
                    &mut entity_to_system,
                );
            } else {
                not_interface_subsystems.push((subsystem_entity, system_component));
            }
        }
    }

    for (subsystem_entity, system_component) in not_interface_subsystems {
        let system = entity_to_system
            .get_mut(&system_entity)
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
            &name_and_description_query,
            &transform_query,
            &mut ctx,
            &mut entity_to_system,
        );
    }

    // TODO : build the surroundings of all the subsystems

    let model = WorldModel {
        version: CURRENT_FILE_VERSION,
        systems: entity_to_system.into_values().collect(),
        interactions: ctx.interactions,
        environment,
    };

    let save_file = save_file_query
        .get_single()
        .expect("there should only be 1 selected file");

    save_to_json(&model, save_file.path_buf.to_str().unwrap());
    next_state.set(state.get().next());
}

fn build_interfaces_interaction_and_external_entities<P: HasInfo + HasSourcesAndSinks>(
    mut ctx: &mut Context,
    system_entity: Entity,
    system: &mut System,
    mut parent: &mut P,
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
    interface_query: &Query<(&Interface, &Transform)>,
    external_entity_query: &Query<&ExternalEntity>,
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
                        &mut ctx,
                        crate::data_model::InterfaceType::Export,
                        system,
                        interface_connection,
                        &name_and_description_query,
                        &interface_query,
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
                            &mut ctx,
                            sink_entity,
                            ExternalEntityType::Sink,
                            IdType::Sink,
                            &*parent,
                            &name_and_description_query,
                            &transform_query,
                            &external_entity_query,
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
                    &mut ctx,
                    flow_entity,
                    flow,
                    parent,
                    system.info.id.clone(),
                    sink_id.clone(),
                    &name_and_description_query,
                );
            }
        } else if flow_end_connection.target == system_entity {
            let interface_index = if let Some(interface_connection) = flow_end_interface_connection
            {
                build_interface(
                    &mut ctx,
                    crate::data_model::InterfaceType::Import,
                    system,
                    interface_connection,
                    &name_and_description_query,
                    &interface_query,
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
                            &mut ctx,
                            source_entity,
                            ExternalEntityType::Source,
                            IdType::Source,
                            &*parent,
                            &name_and_description_query,
                            &transform_query,
                            &external_entity_query,
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
                    &mut ctx,
                    flow_entity,
                    flow,
                    parent,
                    source_id.clone(),
                    system.info.id.clone(),
                    &name_and_description_query,
                );
            }
        }
    }
}

fn build_interface<C: Connection>(
    mut ctx: &mut Context,
    ty: crate::data_model::InterfaceType,
    system: &mut System,
    interface_connection: &C,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
    interface_query: &Query<(&Interface, &Transform)>,
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
    let interaction = Interaction {
        info: info_from_entity(
            flow_entity,
            ctx.next_id(flow_entity, IdType::Flow, &parent.info().id.indices),
            parent.info().level + 1,
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
        amount: flow.amount,
        unit: flow.unit.clone(),
        parameters: flow.parameters.clone(),
    };

    ctx.interactions.push(interaction)
}

fn build_external_entity<P: HasInfo>(
    ctx: &mut Context,
    entity: Entity,
    ty: ExternalEntityType,
    id_type: IdType,
    parent: &P,
    name_and_description_query: &Query<(&Name, &ElementDescription)>,
    transform_query: &Query<(&Transform, &InitialPosition)>,
    external_entity_query: &Query<&ExternalEntity>,
) -> crate::data_model::ExternalEntity {
    let id = ctx.next_id(entity, id_type, &parent.info().id.indices);

    let external_entity_component = external_entity_query.get(entity).expect("Should exist");

    crate::data_model::ExternalEntity {
        info: info_from_entity(
            entity,
            id.clone(),
            parent.info().level + 1,
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
    };

    let root_system = crate::data_model::System {
        info: info_from_entity(system_entity, id, level, &name_and_description_query),
        parent: parent_id,
        complexity: system.complexity,
        boundary,
        sources: vec![], // TODO
        sinks: vec![],   // TODO
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
