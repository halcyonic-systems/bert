use crate::bundles::{spawn_complete_inflow, spawn_complete_outflow, spawn_main_system};
use crate::constants::MAIN_SYSTEM_RADIUS;
use crate::data_model::*;
use crate::resources::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

fn load_from_json(file_name: &str) -> WorldModel {
    let bytes = std::fs::read(file_name).expect("This shouldn't fail");
    serde_json::from_slice(&bytes).expect("This shouldn't fail")
}

pub fn load_world(
    mut commands: Commands,
    state: Res<State<FileImportState>>,
    mut next_state: ResMut<NextState<FileImportState>>,
    selected_file_query: Query<&SelectedFile>,
    existing_elements_query: Query<Entity, With<SystemElement>>,
    subsystem_query: Query<&Subsystem>,
    nesting_query: Query<&NestingLevel>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    // for entity in &existing_elements_query {
    //     commands.entity(entity).despawn_recursive();
    // }
    //
    // let selected_file = selected_file_query
    //     .get_single()
    //     .expect("there should only be 1 selected file");
    //
    // let world_model = load_from_json(selected_file.path_buf.to_str().unwrap());
    //
    // let system = world_model.systems[0].clone();
    //
    // let (center, angle) = system
    //     .transform
    //     .map(|t| (t.translation, t.rotation))
    //     .unwrap_or((Vec2::ZERO, 0.0));
    //
    // let system_entity = spawn_main_system(
    //     &mut commands,
    //     center,
    //     angle,
    //     system.complexity,
    //     SystemBoundary {
    //         porosity: system.boundary.porosity,
    //         perceptive_fuzziness: system.boundary.perceptive_fuzziness,
    //         name: system.boundary.info.name.clone(),
    //         description: system.boundary.info.description.clone(),
    //     },
    //     **zoom,
    //     &system.info.name,
    //     &system.info.description,
    //     &mut meshes,
    // );
    //
    // let focused_system = FocusedSystem::new(system_entity);
    // commands.insert_resource(focused_system);
    //
    // // TODO : multiple connections
    // let mut external_entity_to_interaction = HashMap::new();
    // // let mut external_entity_to_interface = HashMap::new();
    // let mut interface_to_interaction = HashMap::new();
    // let mut id_to_interaction = HashMap::new();
    // let mut id_to_external_entity = HashMap::new();
    //
    // for interaction in world_model.interactions {
    //     external_entity_to_interaction.insert(
    //         interaction.external_entity.clone(),
    //         interaction.info.id.clone(),
    //     );
    //
    //     id_to_interaction.insert(interaction.info.id.clone(), interaction);
    // }
    //
    // for external_entity in system
    //     .environment
    //     .sinks
    //     .into_iter()
    //     .chain(system.environment.sources.into_iter())
    // {
    //     id_to_external_entity.insert(external_entity.info.id.clone(), external_entity);
    // }
    //
    // for interface in &system.boundary.interfaces {
    //     let external_entity = match interface.ty {
    //         InterfaceType::Export => interface.exports_to[0].clone(),
    //         InterfaceType::Import => interface.receives_from[0].clone(),
    //         InterfaceType::Hybrid => todo!(),
    //     };
    //
    //     let interaction = external_entity_to_interaction
    //         .get(&external_entity)
    //         .expect("Interface must have an interaction");
    //
    //     interface_to_interaction.insert(interface.info.id.clone(), interaction.clone());
    // }
    //
    // for interface in &system.boundary.interfaces {
    //     let interaction_id = interface_to_interaction
    //         .get(&interface.info.id)
    //         .expect("Interface must have an interaction");
    //
    //     let interaction = id_to_interaction
    //         .get(interaction_id)
    //         .expect("Interaction must exist");
    //
    //     let _ = match interface.ty {
    //         InterfaceType::Export => {
    //             let external_entity = id_to_external_entity
    //                 .get(&interface.exports_to[0]) // TODO: multiple connections
    //                 .expect("Export interface must have an external entity");
    //
    //             spawn_complete_outflow(
    //                 &mut commands,
    //                 focused_system,
    //                 &subsystem_query,
    //                 &nesting_query,
    //                 &mut meshes,
    //                 &mut stroke_tess,
    //                 &mut fixed_system_element_geometries,
    //                 **zoom,
    //                 interface.angle.unwrap_or(0.0), // TODO: layout algorithm if angle is not provided
    //                 MAIN_SYSTEM_RADIUS,
    //                 interaction.substance.ty,
    //                 if let InteractionType::Outflow { usability } = interaction.ty {
    //                     usability
    //                 } else {
    //                     unreachable!("Export interface must have an outflow interaction")
    //                 },
    //                 interaction.amount,
    //                 &interaction.unit,
    //                 &interaction.time_unit,
    //                 &interface.info.name,
    //                 &interface.info.description,
    //                 &interaction.info.name,
    //                 &interaction.info.description,
    //                 &external_entity.info.name,
    //                 &external_entity.info.description,
    //                 external_entity.transform.as_ref(),
    //             )
    //         }
    //         InterfaceType::Import => {
    //             let external_entity = id_to_external_entity
    //                 .get(&interface.receives_from[0]) // TODO: multiple connections
    //                 .expect("Export interface must have an external entity");
    //
    //             spawn_complete_inflow(
    //                 &mut commands,
    //                 focused_system,
    //                 &subsystem_query,
    //                 &nesting_query,
    //                 &mut meshes,
    //                 &mut stroke_tess,
    //                 &mut fixed_system_element_geometries,
    //                 **zoom,
    //                 interface.angle.unwrap_or(0.0), // TODO: layout algorithm if angle is not provided
    //                 MAIN_SYSTEM_RADIUS,
    //                 interaction.substance.ty,
    //                 if let InteractionType::Inflow { usability } = interaction.ty {
    //                     usability
    //                 } else {
    //                     unreachable!("Import interface must have an inflow interaction")
    //                 },
    //                 interaction.amount,
    //                 &interaction.unit,
    //                 &interaction.time_unit,
    //                 &interface.info.name,
    //                 &interface.info.description,
    //                 &interaction.info.name,
    //                 &interaction.info.description,
    //                 &external_entity.info.name,
    //                 &external_entity.info.description,
    //                 external_entity.transform.as_ref(),
    //             )
    //         }
    //         InterfaceType::Hybrid => todo!(),
    //     };
    // }
    // next_state.set(state.get().next());
}
