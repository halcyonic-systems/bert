use crate::bundles::{
    despawn_create_button, despawn_create_button_with_component, spawn_create_button,
    spawn_external_entity, spawn_inflow, spawn_interface, spawn_interface_subsystem, spawn_outflow,
};
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;
use num_traits::float::FloatConst;

macro_rules! interface_create_button {
    ($fn_name:ident, $flow:ty, $interface_connection:ty, $terminal_connection:ty, $button_type:expr) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &Transform, &$flow),
                (
                    Without<$interface_connection>,
                    Without<$terminal_connection>,
                    Without<FlowInterfaceButton>,
                ),
            >,
            focused_system: Res<FocusedSystem>,
            asset_server: Res<AssetServer>,
        ) {
            for (entity, transform, flow) in &query {
                if flow.system != **focused_system {
                    continue;
                }

                let direction = transform.right().truncate();

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: $button_type,
                        connection_source: entity,
                        system: **focused_system,
                    },
                    transform.translation.truncate() - direction * 64.0,
                    direction.to_angle(),
                    &asset_server,
                );
            }
        }
    };
}

interface_create_button!(
    add_outflow_interface_create_button,
    Outflow,
    OutflowInterfaceConnection,
    OutflowSinkConnection,
    CreateButtonType::ExportInterface
);
interface_create_button!(
    add_inflow_interface_create_button,
    Inflow,
    InflowInterfaceConnection,
    InflowSourceConnection,
    CreateButtonType::ImportInterface
);

macro_rules! external_entity_create_button {
    ($fn_name:ident, $flow:ty, $interface_connection:ty, $terminal_connection:ty, $button_type:expr) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &Transform, &$flow),
                (
                    With<$interface_connection>,
                    Without<$terminal_connection>,
                    Without<FlowOtherEndButton>,
                ),
            >,
            focused_system: Res<FocusedSystem>,
            asset_server: Res<AssetServer>,
        ) {
            for (entity, transform, flow) in &query {
                if flow.system != **focused_system {
                    continue;
                }

                let direction = transform.right().truncate();

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: $button_type,
                        connection_source: entity,
                        system: **focused_system,
                    },
                    transform.translation.truncate() + direction * 64.0,
                    direction.to_angle(),
                    &asset_server,
                );
            }
        }
    };
}

external_entity_create_button!(
    add_source_create_button,
    Inflow,
    InflowInterfaceConnection,
    InflowSourceConnection,
    CreateButtonType::Source
);
external_entity_create_button!(
    add_sink_create_button,
    Outflow,
    OutflowInterfaceConnection,
    OutflowSinkConnection,
    CreateButtonType::Sink
);

pub fn add_first_outflow_create_button(
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    button_query: Query<&CreateButton>,
    flow_system_query: Query<
        &Outflow,
        Or<(
            Without<OutflowSinkConnection>,
            Without<OutflowInterfaceConnection>,
        )>,
    >,
    flow_interface_query: Query<
        (&Outflow, &OutflowInterfaceConnection),
        With<InflowSourceConnection>,
    >,
    transform_query: Query<&GlobalTransform>,
    asset_server: Res<AssetServer>,
) {
    if !focused_system.is_changed() {
        return;
    }

    let focused_system = **focused_system;

    for button in &button_query {
        if button.system == focused_system && matches!(button.ty, CreateButtonType::Outflow) {
            return;
        }
    }

    for outflow in &flow_system_query {
        if outflow.system == focused_system {
            return;
        }
    }

    // find button angle

    let system_center = transform_query
        .get(focused_system)
        .expect("System should have a Transform")
        .translation();

    let mut existing_interfaces = HashSet::new();

    for (outflow, flow_interface_connection) in &flow_interface_query {
        if outflow.system == focused_system {
            existing_interfaces.insert(flow_interface_connection.target);
        }
    }

    let mut min_angle = f32::PI();

    for interface in existing_interfaces {
        let interface_pos = transform_query
            .get(interface)
            .expect("Interface should have a Transform")
            .translation();

        let angle = (interface_pos - system_center).truncate().to_angle();

        min_angle = min_angle.min(angle);
    }

    info!("System center: {}", system_center);
    info!("Min angle: {}", min_angle);

    min_angle -= f32::FRAC_PI_8();

    spawn_create_button(
        &mut commands,
        CreateButton {
            ty: CreateButtonType::Outflow,
            connection_source: focused_system,
            system: focused_system,
        },
        vec2(system_center.x + 32.0, system_center.y),
        min_angle,
        &asset_server,
    );
}

pub fn add_consecutive_outflow_create_button(
    mut commands: Commands,
    query: Query<(&Transform, &Outflow), Added<OutflowSinkConnection>>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, outflow)) = query.get_single() {
        let system_entity = outflow.system;

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Outflow,
                connection_source: system_entity,
                system: **focused_system,
            },
            vec2(128.0, transform.translation.y - 70.0),
            0.0,
            &asset_server,
        );
    }
}

pub fn add_first_inflow_create_button(
    mut commands: Commands,
    changed_query: Query<Entity, Or<(Added<OutflowSinkConnection>, Changed<Outflow>)>>,
    query: Query<&Outflow, With<OutflowSinkConnection>>,
    inflow_query: Query<&Inflow>,
    button_query: Query<(Entity, &CreateButton)>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let mut system_to_outflow_usabilities = HashMap::new();

    for outflow in &query {
        system_to_outflow_usabilities
            .entry(outflow.system)
            .or_insert_with(HashSet::new)
            .insert(outflow.usability);
    }

    'outer: for (system_entity, outflow_usabilities) in system_to_outflow_usabilities {
        if outflow_usabilities.len() > 1 {
            for inflow in inflow_query.iter() {
                if inflow.system == system_entity {
                    continue 'outer;
                }
            }

            for (_, button) in button_query.iter() {
                if matches!(button.ty, CreateButtonType::Inflow)
                    && button.connection_source == system_entity
                {
                    continue 'outer;
                }
            }

            spawn_create_button(
                &mut commands,
                CreateButton {
                    ty: CreateButtonType::Inflow,
                    connection_source: system_entity,
                    system: **focused_system,
                },
                vec2(-128.0, 100.0),
                0.0,
                &asset_server,
            );
        } else {
            for (entity, button) in &button_query {
                if matches!(button.ty, CreateButtonType::Inflow)
                    && button.connection_source == system_entity
                {
                    despawn_create_button_with_component(&mut commands, entity, button)
                }
            }
        }
    }
}

pub fn add_consecutive_inflow_create_button(
    mut commands: Commands,
    query: Query<(&Transform, &Inflow), Added<InflowSourceConnection>>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((transform, inflow)) = query.get_single() {
        let system_entity = inflow.system;

        spawn_create_button(
            &mut commands,
            CreateButton {
                ty: CreateButtonType::Inflow,
                connection_source: system_entity,
                system: **focused_system,
            },
            vec2(-128.0, transform.translation.y - 70.0),
            0.0,
            &asset_server,
        );
    }
}

pub fn add_interface_subsystem_create_buttons(
    mut commands: Commands,
    changed_query: Query<
        Entity,
        Or<(
            Added<InflowSourceConnection>,
            Added<OutflowSinkConnection>,
            Changed<Inflow>,
            Changed<Outflow>,
        )>,
    >,
    flow_query: Query<
        (Option<&Outflow>, Option<&Inflow>),
        (
            Or<(With<InflowSourceConnection>, With<OutflowSinkConnection>)>,
            Or<(With<Inflow>, With<Outflow>)>,
        ),
    >,
    flow_interface_query: Query<
        (
            Option<&Inflow>,
            Option<&Outflow>,
            Option<&InflowInterfaceConnection>,
            Option<&OutflowInterfaceConnection>,
        ),
        Or<(With<InflowSourceConnection>, With<OutflowSinkConnection>)>,
    >,
    interface_query: Query<
        &Transform,
        (
            Without<InterfaceSubsystemButton>,
            Without<InterfaceSubsystemConnection>,
        ),
    >,
    interface_button_query: Query<&InterfaceSubsystemButton>,
    button_query: Query<&CreateButton>,
    focused_system: Res<FocusedSystem>,
    asset_server: Res<AssetServer>,
) {
    // TODO : also detect removal

    if changed_query.is_empty() {
        return;
    }

    let mut flow_usabilities = HashSet::new();

    for (outflow, inflow) in &flow_query {
        match (inflow, outflow) {
            (Some(inflow), None) => {
                if inflow.system != **focused_system {
                    continue;
                }
                flow_usabilities.insert(GeneralUsability::Inflow(inflow.usability));
            }
            (None, Some(outflow)) => {
                if outflow.system != **focused_system {
                    continue;
                }
                flow_usabilities.insert(GeneralUsability::Outflow(outflow.usability));
            }
            (Some(inflow), Some(outflow)) => {
                if inflow.system == **focused_system {
                    flow_usabilities.insert(GeneralUsability::Inflow(inflow.usability));
                } else if outflow.system == **focused_system {
                    flow_usabilities.insert(GeneralUsability::Outflow(outflow.usability));
                }
            }
            _ => unreachable!("Outflow and inflow can't both be None"),
        }
    }

    for (inflow, outflow, inflow_interface_connection, outflow_interface_connection) in
        &flow_interface_query
    {
        let interface_entity = match (inflow, outflow) {
            (Some(inflow), None) => {
                if inflow.system != **focused_system {
                    continue;
                }

                inflow_interface_connection
                    .expect("Should be there because we have an Inflow")
                    .target
            }
            (None, Some(outflow)) => {
                if outflow.system != **focused_system {
                    continue;
                }

                outflow_interface_connection
                    .expect("Should be there because we have an Outflow")
                    .target
            }
            (Some(inflow), Some(outflow)) => {
                if inflow.system == **focused_system {
                    inflow_interface_connection
                        .expect("Should be there because we have an Inflow")
                        .target
                } else if outflow.system == **focused_system {
                    outflow_interface_connection
                        .expect("Should be there because we have an Outflow")
                        .target
                } else {
                    continue;
                }
            }
            _ => unreachable!("Outflow and inflow can't both be None"),
        };

        if flow_usabilities.len() > 3 {
            if let Ok(transform) = interface_query.get(interface_entity) {
                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: CreateButtonType::InterfaceSubsystem,
                        connection_source: interface_entity,
                        system: **focused_system,
                    },
                    transform.translation.truncate(),
                    0.0,
                    &asset_server,
                );
            }
        } else {
            if let Ok(interface_button) = interface_button_query.get(interface_entity) {
                despawn_create_button(&mut commands, interface_button.button_entity, &button_query);
            }
        }
    }
}

pub fn change_focused_system(
    selected_query: Query<
        (Entity, &PickSelection),
        (
            Changed<PickSelection>,
            Or<(With<crate::components::System>, With<Subsystem>)>,
        ),
    >,
    button_query: Query<&CreateButton>,
    mut focused_system: ResMut<FocusedSystem>,
) {
    for (entity, selection) in &selected_query {
        if selection.is_selected {
            for button in &button_query {
                if button.system == **focused_system
                    && matches!(button.ty, CreateButtonType::InterfaceSubsystem)
                {
                    return;
                }
            }

            **focused_system = entity;
        }
    }
}

pub fn remove_unfocused_system_buttons(
    mut commands: Commands,
    focused_system: Res<FocusedSystem>,
    previous_focused_system: Local<Option<Entity>>,
    button_query: Query<(Entity, &CreateButton)>,
) {
    if !focused_system.is_changed() || Some(**focused_system) == *previous_focused_system {
        return;
    }

    let focused_system = **focused_system;

    for (entity, button) in &button_query {
        if button.system != focused_system {
            despawn_create_button_with_component(&mut commands, entity, button);
        }
    }
}

pub fn on_create_button_click(
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    button_query: Query<(&CreateButton, &GlobalTransform)>,
    only_button_query: Query<&CreateButton>,
    flow_interface_query: Query<
        (
            Entity,
            Option<&InflowInterfaceConnection>,
            Option<&OutflowInterfaceConnection>,
        ),
        Or<(With<Inflow>, With<Outflow>)>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (button, transform) = button_query
        .get(event.target)
        .expect("After on click this has to exist");

    match button.ty {
        CreateButtonType::ImportInterface => spawn_interface(
            &mut commands,
            InterfaceType::Import,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::ExportInterface => spawn_interface(
            &mut commands,
            InterfaceType::Export,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::Inflow => spawn_inflow(
            &mut commands,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::Outflow => spawn_outflow(
            &mut commands,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::Source => spawn_external_entity(
            &mut commands,
            InterfaceType::Import,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::Sink => spawn_external_entity(
            &mut commands,
            InterfaceType::Export,
            button.connection_source,
            transform.translation().truncate(),
            &mut meshes,
            &mut materials,
        ),
        CreateButtonType::InterfaceSubsystem => spawn_interface_subsystem(
            &mut commands,
            button.connection_source,
            &flow_interface_query,
            &mut meshes,
            &mut materials,
        ),
    }

    despawn_create_button(&mut commands, event.target, &only_button_query);
}

pub fn apply_zoom(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            Option<&ScaleWithZoom>,
            Option<&InitialPosition>,
        ),
        Without<Camera>,
    >,
    zoom: Res<Zoom>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (entity, mut transform, scale_with_zoom, initial_position) in &mut query {
        let initial_position = if let Some(initial_position) = initial_position {
            **initial_position
        } else {
            let pos2d = transform.translation.truncate();
            commands.entity(entity).insert(InitialPosition::new(pos2d));
            pos2d
        };

        transform.translation = (initial_position * **zoom).extend(transform.translation.z);

        if let Some(scale_with_zoom) = scale_with_zoom {
            transform.scale = (**scale_with_zoom * **zoom).extend(transform.scale.z);
        }
    }
}

pub fn apply_zoom_to_stroke(
    mut query: Query<(&mut Stroke, &ZoomIndependentStrokeWidth)>,
    zoom: Res<Zoom>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (mut stroke, width) in &mut query {
        stroke.options.line_width = **width / **zoom;
    }
}
