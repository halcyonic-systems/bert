use crate::components::*;
use crate::data_model::*;
use bevy::core::Name;
use bevy::prelude::*;

pub fn save_world(
    state: Res<State<FileExportState>>,
    mut next_state: ResMut<NextState<FileExportState>>,
    save_file_query: Query<&SaveFile>,
    main_system_info_query: Query<
        (
            Entity,
            &Name,
            &ElementDescription,
            &crate::components::System,
            &SystemEnvironment,
            &Transform,
            &InitialPosition,
        ),
        Without<Subsystem>,
    >,
    inflow_query: Query<(
        &Name,
        &ElementDescription,
        &Flow,
        &FlowEndConnection,
        &FlowStartConnection,
        &FlowEndInterfaceConnection,
    )>,
    outflow_query: Query<(
        &Name,
        &ElementDescription,
        &Flow,
        &FlowStartConnection,
        &FlowEndConnection,
        &FlowStartInterfaceConnection,
    )>,
    interface_query: Query<(
        &Name,
        &ElementDescription,
        &crate::components::Interface,
        &Transform,
    )>,
    external_entity_query: Query<(&Name, &ElementDescription, &Transform, &InitialPosition)>,
    // subsystem_query: Query<
    //     (Entity, &Name, &crate::components::ElementDescription, &crate::components::SystemElement),
    //     (With<crate::components::Subsystem>, Without<crate::components::SystemOfInterest>)
    // >
) {
    let (system_entity, name, description, system, environment, transform, initial_position) =
        main_system_info_query
            .get_single()
            .expect("System of interest should exist");

    let mut interfaces = vec![];
    let mut external_interactions = vec![];
    let mut sources = vec![];
    let mut sinks = vec![];

    for (name, description, flow, flow_end_connection, source_connection, interface_connection) in
        &inflow_query
    {
        process_external_inflow(
            &interface_query,
            &external_entity_query,
            system_entity,
            &mut interfaces,
            &mut external_interactions,
            &mut sources,
            name,
            description,
            flow,
            flow_end_connection,
            source_connection,
            interface_connection,
        );
    }

    for (name, description, flow, flow_start_connection, sink_connection, interface_connection) in
        &outflow_query
    {
        process_external_outflow(
            &interface_query,
            &external_entity_query,
            system_entity,
            &mut interfaces,
            &mut external_interactions,
            &mut sinks,
            name,
            description,
            flow,
            flow_start_connection,
            sink_connection,
            interface_connection,
        );
    }

    let boundary = Boundary {
        info: Info {
            id: Id {
                ty: IdType::Boundary,
                indices: vec![0],
            },
            level: 0,
            name: system.boundary.name.clone(),
            description: system.boundary.description.clone(),
        },
        porosity: system.boundary.porosity,
        perceptive_fuzziness: system.boundary.perceptive_fuzziness,
        interfaces,
    };

    let environment = Environment {
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

    let system_of_interest = crate::data_model::System {
        info: Info {
            id: Id {
                ty: IdType::System,
                indices: vec![0],
            },
            level: 0,
            name: name.to_string(),
            description: description.text.clone(),
        },
        parent: None,
        complexity: system.complexity,
        environment,
        boundary,
        internal_interactions: vec![], // TODO
        external_interactions,
        components: vec![],
        transform: Some(Transform2d::from((transform, initial_position))),
    };

    let model = WorldModel { system_of_interest };

    let save_file = save_file_query
        .get_single()
        .expect("there should only be 1 selected file");

    save_to_json(&model, save_file.path_buf.to_str().unwrap());
    next_state.set(state.get().next());
}

macro_rules! process_external_flow {
    (
        $fn_name:ident,
        $flow_conn_ty:ty,
        $interface_connection_ty:ty,
        $external_entity_connection_ty:ty,
        $interaction_ty:tt,
        $interface_ty:expr,
        $external_entity_ty:expr,
        $id_ty:expr,
        $external_entity_field:tt,
        $usability_ty:tt
    ) => {
        fn $fn_name(
            interface_query: &Query<(
                &Name,
                &ElementDescription,
                &crate::components::Interface,
                &Transform,
            )>,
            external_entity_query: &Query<(
                &Name,
                &ElementDescription,
                &Transform,
                &InitialPosition,
            )>,
            system_entity: Entity,
            interfaces: &mut Vec<crate::data_model::Interface>,
            interactions: &mut Vec<crate::data_model::Interaction>,
            external_entities: &mut Vec<crate::data_model::ExternalEntity>,
            name: &Name,
            description: &ElementDescription,
            flow: &Flow,
            flow_system_connection: &$flow_conn_ty,
            source_connection: &$external_entity_connection_ty,
            interface_connection: &$interface_connection_ty,
        ) {
            if flow_system_connection.target == system_entity {
                let interaction_id = Id {
                    ty: IdType::Flow,
                    indices: vec![-1, interactions.len() as i64],
                };
                let external_entity_id = Id {
                    ty: $id_ty,
                    indices: vec![-1, external_entities.len() as i64],
                };

                interactions.push(crate::data_model::Interaction {
                    info: Info {
                        id: interaction_id.clone(),
                        name: name.to_string(),
                        description: description.text.clone(),
                        level: -1,
                    },
                    substance: Substance {
                        sub_type: None, // TODO
                        ty: flow.substance_type,
                    },
                    ty: crate::data_model::InteractionType::$interaction_ty {
                        usability: $usability_ty::from_useful(flow.is_useful),
                    },
                    external_entity: external_entity_id.clone(),
                    amount: flow.amount,
                    unit: flow.unit.clone(),
                    time_unit: flow.time_unit.clone(),
                });

                let (interface_name, interface_description, interface, interface_transform) =
                    interface_query
                        .get(interface_connection.target)
                        .expect("Interface should exist");

                let mut interface = crate::data_model::Interface {
                    info: Info {
                        id: Id {
                            ty: IdType::Interface,
                            indices: vec![0, interfaces.len() as i64],
                        },
                        name: interface_name.to_string(),
                        description: interface_description.text.clone(),
                        level: 1,
                    },
                    protocol: interface.protocol.clone(),
                    ty: $interface_ty,     // TODO: hybrid
                    receives_from: vec![], // TODO : multiple
                    exports_to: vec![],    // TODO
                    angle: Some(interface_transform.right().truncate().to_angle()),
                };

                interface
                    .$external_entity_field
                    .push(external_entity_id.clone());

                interfaces.push(interface);

                let (
                    external_entity_name,
                    external_entity_description,
                    external_entity_transform,
                    initial_position,
                ) = external_entity_query
                    .get(source_connection.target)
                    .expect("External entity should exist");

                external_entities.push(crate::data_model::ExternalEntity {
                    info: Info {
                        id: external_entity_id,
                        name: external_entity_name.to_string(),
                        description: external_entity_description.text.clone(),
                        level: -1,
                    },
                    ty: $external_entity_ty,
                    interactions: vec![interaction_id], // TODO : multiple
                    transform: Some(Transform2d::from((
                        external_entity_transform,
                        initial_position,
                    ))),
                });
            }
        }
    };
}

process_external_flow!(
    process_external_inflow,
    FlowEndConnection,
    FlowEndInterfaceConnection,
    FlowStartConnection,
    Inflow,
    crate::data_model::InterfaceType::Import,
    ExternalEntityType::Source,
    IdType::Source,
    receives_from,
    InflowUsability
);

process_external_flow!(
    process_external_outflow,
    FlowStartConnection,
    FlowStartInterfaceConnection,
    FlowEndConnection,
    Outflow,
    crate::data_model::InterfaceType::Export,
    ExternalEntityType::Sink,
    IdType::Sink,
    exports_to,
    OutflowUsability
);

fn save_to_json(world_model: &WorldModel, file_name: &str) {
    let json = serde_json::to_string(world_model).expect("This shouldn't fail");
    std::fs::write(file_name, json).expect("This shouldn't fail");
}
