use crate::bundles::{
    spawn_external_entity_only, spawn_interaction_only, spawn_interface_only, spawn_main_system,
    SystemBundle,
};
use crate::constants::{EXTERNAL_ENTITY_Z, INTERFACE_Z, SUBSYSTEM_Z};
use crate::data_model::*;
use crate::events::SubsystemDrag;
use crate::plugins::file_dialog::ImportFileEvent;
use crate::plugins::mouse_interaction::DragPosition;
use crate::resources::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_eventlistener::event_listener::On;
use rust_decimal_macros::dec;

fn load_from_json(file_name: &str) -> WorldModel {
    let bytes = std::fs::read(file_name).expect("This shouldn't fail");
    serde_json::from_slice(&bytes).expect("This shouldn't fail")
}

/// Context for bookkeeping while we traverse the data model and spawn the entities and components.
struct Context {
    /// Maps the data model id to the spawned bevy entity
    id_to_entity: HashMap<Id, Entity>,
    /// Maps the data model id to wether this is an interface subsystem or not
    id_to_interface_subsystem: HashMap<Id, bool>,
    /// Maps the id of an external entity to the subsystance type of it's connecting interaction
    external_entity_id_to_substance: HashMap<Id, SubstanceType>,
    /// Maps the spawned bevy entity to wether there are (outgoing, ingoing) interactions connected
    entity_to_interface_interactions: HashMap<Entity, (bool, bool)>,
}

impl Context {
    fn new() -> Self {
        Self {
            id_to_entity: HashMap::new(),
            id_to_interface_subsystem: HashMap::new(),
            external_entity_id_to_substance: HashMap::new(),
            entity_to_interface_interactions: HashMap::new(),
        }
    }
}

pub fn load_world(
    mut commands: Commands,
    mut load_file_event_reader: EventReader<ImportFileEvent>,
    existing_elements_query: Query<Entity, With<SystemElement>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
) {
    for event in load_file_event_reader.read() {
        // clear the scene first
        for entity in &existing_elements_query {
            commands.entity(entity).despawn_recursive();
        }

        let selected_file = &**event;
        let world_model = load_from_json(selected_file.to_str().unwrap());

        let mut ctx = Context::new();

        // start by mapping all external entities to the substance type
        for interaction in &world_model.interactions {
            if matches!(interaction.sink.ty, IdType::Sink) {
                ctx.external_entity_id_to_substance
                    .insert(interaction.sink.clone(), interaction.substance.ty);
            }

            if matches!(interaction.source.ty, IdType::Source) {
                ctx.external_entity_id_to_substance
                    .insert(interaction.source.clone(), interaction.substance.ty);
            }
        }

        // then spawn everything

        spawn_systems_interfaces_and_external_entities(
            &mut commands,
            &mut ctx,
            &world_model,
            **zoom,
            &mut meshes,
            &mut stroke_tess,
            &mut fixed_system_element_geometries,
        );

        make_systems_parent_child_hierarchy(&mut commands, &mut ctx, &world_model);

        spawn_external_entities(
            &mut commands,
            &mut ctx,
            &world_model.environment,
            None,
            **zoom,
            &mut meshes,
            &mut stroke_tess,
            &mut fixed_system_element_geometries,
        );

        spawn_interactions(
            &mut commands,
            &mut ctx,
            &world_model,
            **zoom,
            &mut meshes,
            &mut stroke_tess,
        );
    }
}

fn spawn_interactions(
    commands: &mut Commands,
    ctx: &mut Context,
    world_model: &WorldModel,
    zoom: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    stroke_tess: &mut ResMut<StrokeTessellator>,
) {
    for interaction in &world_model.interactions {
        let nesting_level = interaction.info.level.max(0) as u16;
        let interaction_entity = spawn_interaction_only(
            commands,
            Flow {
                interaction_type: interaction.ty,
                substance_type: interaction.substance.ty,
                substance_sub_type: interaction.substance.sub_type.clone(),
                amount: interaction.amount,
                unit: interaction.unit.clone(),
                usability: interaction.usability,
                parameters: interaction.parameters.clone(),
            },
            FlowCurve::default(),
            &interaction.info.name,
            &interaction.info.description,
            false,
            nesting_level,
            NestingLevel::compute_scale(nesting_level, zoom),
            stroke_tess,
            meshes,
        );

        ctx.id_to_entity
            .insert(interaction.info.id.clone(), interaction_entity);

        let mut interaction_commands = commands.entity(interaction_entity);

        let mut system_id = Id {
            ty: IdType::System,
            indices: vec![],
        };

        let start_target = ctx.id_to_entity[&interaction.source];

        let target_type = if matches!(interaction.source.ty, IdType::Source) {
            StartTargetType::Source
        } else {
            system_id = interaction.source.clone();
            StartTargetType::System
        };

        interaction_commands.insert(FlowStartConnection {
            target: start_target,
            target_type,
        });

        let end_target = ctx.id_to_entity[&interaction.sink];

        let target_type = if matches!(interaction.sink.ty, IdType::Sink) {
            EndTargetType::Sink
        } else {
            system_id = interaction.sink.clone();
            EndTargetType::System
        };

        interaction_commands.insert(FlowEndConnection {
            target: end_target,
            target_type,
        });

        if let Some(interface_id) = &interaction.source_interface {
            interaction_commands.insert(FlowStartInterfaceConnection {
                target: ctx.id_to_entity[interface_id],
            });
        }

        if let Some(interface_id) = &interaction.sink_interface {
            interaction_commands.insert(FlowEndInterfaceConnection {
                target: ctx.id_to_entity[interface_id],
            });
        }

        for system in &world_model.systems {
            if system.info.id == system_id {
                if system.info.level != 0 {
                    commands
                        .entity(ctx.id_to_entity[&system.parent])
                        .add_child(interaction_entity);
                }
                break;
            }
        }
    }
}

fn spawn_systems_interfaces_and_external_entities(
    commands: &mut Commands,
    ctx: &mut Context,
    world_model: &WorldModel,
    zoom: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
) {
    for system in &world_model.systems {
        let nesting_level = system.info.level as u16;

        let (position, angle) = system
            .transform
            .map(|t| (t.translation, t.rotation))
            .unwrap_or((Vec2::ZERO, 0.0));

        let system_entity = if nesting_level == 0 {
            let system_entity = spawn_main_system(
                commands,
                position,
                angle,
                system.complexity,
                SystemBoundary {
                    porosity: system.boundary.porosity,
                    perceptive_fuzziness: system.boundary.perceptive_fuzziness,
                    name: system.boundary.info.name.clone(),
                    description: system.boundary.info.description.clone(),
                },
                zoom,
                &system.info.name,
                &system.info.description,
                meshes,
            );

            let focused_system = FocusedSystem::new(system_entity);
            commands.insert_resource(focused_system);

            system_entity
        } else {
            spawn_loaded_subsystem(
                ctx,
                commands,
                &system,
                nesting_level,
                position,
                angle,
                meshes,
                zoom,
            )
        };

        for interface in &system.boundary.interfaces {
            let interface_entity = spawn_loaded_interface(
                ctx,
                commands,
                interface,
                system,
                nesting_level,
                system_entity,
                zoom,
                meshes,
                stroke_tess,
                fixed_system_element_geometries,
            );

            ctx.id_to_entity
                .insert(interface.info.id.clone(), interface_entity);
        }

        ctx.id_to_entity
            .insert(system.info.id.clone(), system_entity);

        spawn_external_entities(
            commands,
            ctx,
            system,
            Some(system_entity),
            zoom,
            meshes,
            stroke_tess,
            fixed_system_element_geometries,
        );
    }
}

fn spawn_external_entities<S: HasSourcesAndSinks + HasInfo>(
    commands: &mut Commands,
    ctx: &mut Context,
    sources_and_sinks: &S,
    parent_entity: Option<Entity>,
    zoom: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
) {
    for ext_entity in sources_and_sinks
        .sources()
        .iter()
        .chain(sources_and_sinks.sinks().iter())
    {
        let (transform, initial_position) = ext_entity
            .transform
            .unwrap_or_default()
            .as_components(EXTERNAL_ENTITY_Z, zoom);

        let external_entity = spawn_external_entity_only(
            commands,
            ctx.external_entity_id_to_substance[&ext_entity.info.id],
            false,
            &ext_entity.info.name,
            &ext_entity.info.description,
            transform,
            initial_position,
            (sources_and_sinks.info().level + 1) as u16,
            zoom,
            fixed_system_element_geometries,
            meshes,
            stroke_tess,
        );

        ctx.id_to_entity
            .insert(ext_entity.info.id.clone(), external_entity);

        if let Some(parent_entity) = parent_entity {
            commands.entity(parent_entity).add_child(external_entity);
        }
    }
}

/// Go through all the systems create parent-child relationsships between them. If we find a first
/// level interface subsystem we add it as a child of the interface. Otherwise it's added as a
/// child to its parent system.
fn make_systems_parent_child_hierarchy(
    commands: &mut Commands,
    ctx: &mut Context,
    world_model: &WorldModel,
) {
    for system in &world_model.systems {
        if system.info.level == 0 {
            continue;
        }

        let system_entity = ctx.id_to_entity[&system.info.id];

        let parent_entity = if ctx.id_to_interface_subsystem[&system.info.id] {
            let interface_id = system
                .boundary
                .parent_interface
                .as_ref()
                .expect("There should be a parent interface");

            let interface_entity = ctx.id_to_entity[interface_id];
            commands
                .entity(interface_entity)
                .insert(InterfaceSubsystemConnection {
                    target: system_entity,
                });
            interface_entity
        } else {
            ctx.id_to_entity[&system.parent]
        };

        if let Some(&parent_interface) = system
            .boundary
            .parent_interface
            .as_ref()
            .and_then(|id| ctx.id_to_entity.get(id))
        {
            let mut system_commands = commands.entity(system_entity);
            system_commands.insert(InterfaceSubsystem {
                interface_entity: parent_interface,
                total_inflow: dec!(0),
                total_outflow: dec!(0),
                substance_type: Default::default(),
                is_useful: Default::default(),
            });

            let (import, export) = ctx.entity_to_interface_interactions[&parent_interface];
            if import {
                system_commands.insert(ImportSubsystem);
            }
            if export {
                system_commands.insert(ExportSubsystem);
            }
        }

        commands.entity(parent_entity).add_child(system_entity);

        commands.entity(system_entity).insert(Subsystem {
            parent_system: ctx.id_to_entity[&system.parent],
        });
    }
}

fn spawn_loaded_interface(
    ctx: &mut Context,
    commands: &mut Commands,
    interface: &Interface,
    system: &System,
    nesting_level: u16,
    system_entity: Entity,
    zoom: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
) -> Entity {
    let angle = interface.angle.unwrap_or(0.0);
    let pos = Vec2::from_angle(angle) * system.radius;
    let initial_position = InitialPosition::new(pos);
    let transform = Transform::from_translation((pos * zoom).extend(INTERFACE_Z))
        .with_rotation(Quat::from_rotation_z(angle));

    let interface_entity = spawn_interface_only(
        commands,
        SubstanceType::default(), // TODO : update from flow later
        nesting_level,
        system_entity,
        zoom,
        false,
        &interface.info.name,
        &interface.info.description,
        transform,
        initial_position,
        stroke_tess,
        meshes,
        fixed_system_element_geometries,
    );

    ctx.entity_to_interface_interactions.insert(
        interface_entity,
        (
            !interface.receives_from.is_empty(),
            !interface.exports_to.is_empty(),
        ),
    );

    interface_entity
}

fn spawn_loaded_subsystem(
    ctx: &mut Context,
    commands: &mut Commands,
    system: &&System,
    nesting_level: u16,
    position: Vec2,
    angle: f32,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
) -> Entity {
    let interface_subsystem = if let Some(parent_interface_id) = &system.boundary.parent_interface {
        parent_interface_id.indices == system.info.id.indices
    } else {
        false
    };

    ctx.id_to_interface_subsystem
        .insert(system.info.id.clone(), interface_subsystem);

    let z = if interface_subsystem {
        SUBSYSTEM_Z - INTERFACE_Z
    } else {
        SUBSYSTEM_Z
    };

    commands
        .spawn((
            NestingLevel::new(nesting_level),
            SystemBundle::new(
                position,
                z,
                system.radius,
                angle,
                system.complexity,
                SystemBoundary {
                    porosity: system.boundary.porosity,
                    perceptive_fuzziness: system.boundary.perceptive_fuzziness,
                    name: system.boundary.info.name.clone(),
                    description: system.boundary.info.description.clone(),
                },
                meshes,
                zoom,
                nesting_level,
                &system.info.name,
                &system.info.description,
            ),
            On::<DragPosition>::send_event::<SubsystemDrag>(),
        ))
        .id()
}
