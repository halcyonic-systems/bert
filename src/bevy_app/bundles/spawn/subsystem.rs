use crate::bevy_app::bundles::{spawn_interface, SystemBundle};
use crate::bevy_app::components::*;
use crate::bevy_app::constants::*;
use crate::bevy_app::data_model::Complexity;
use crate::bevy_app::events::SubsystemDrag;
use crate::bevy_app::plugins::label::{
    add_name_label, Alignment, AutoContrastTextColor, CopyPositionArgs,
};
use crate::bevy_app::plugins::mouse_interaction::DragPosition;
use crate::bevy_app::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator,
};
use crate::bevy_app::utils::{
    compute_end_and_direction_from_subsystem, transform_from_point2d_and_direction,
};
use crate::plugins::label::{HorizontalAttachmentAnchor, VerticalAttachmentAnchor};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;

pub fn spawn_interface_subsystem(
    commands: &mut Commands,
    is_child_of_interface: bool,
    interface_type: InterfaceType,
    interface_entity: Entity,
    flow_interface_query: &Query<(
        Entity,
        &Flow,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartInterfaceConnection>,
    )>,
    system_query: &Query<(
        &Transform,
        &crate::bevy_app::components::System,
        &Name,
        &ElementDescription,
    )>,
    nesting_level_query: &Query<&NestingLevel>,
    focused_system: &Res<FocusedSystem>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    name: &str,
    description: &str,
) -> Entity {
    let mut angle = 0.0;
    let mut is_import_subsystem = false;
    let mut is_export_subsystem = false;
    let mut interface_subsystem = InterfaceSubsystem::new(interface_entity);

    for (_, flow, inflow_connection, outflow_connection) in flow_interface_query {
        if let Some(connection) = inflow_connection {
            if connection.target == interface_entity {
                angle = std::f32::consts::PI;
                is_import_subsystem = true;
                interface_subsystem.total_inflow += flow.amount;
                interface_subsystem.substance_type = flow.substance_type;
                interface_subsystem.is_useful = flow.usability.is_useful();
            }
        }
        if let Some(connection) = outflow_connection {
            if connection.target == interface_entity {
                is_export_subsystem = true;
                interface_subsystem.total_outflow += flow.amount;
                interface_subsystem.substance_type = flow.substance_type;
                interface_subsystem.is_useful = flow.usability.is_useful();
            }
        }
    }

    let parent_system = ***focused_system;

    let (z, pos) = if is_child_of_interface {
        (SUBSYSTEM_Z - INTERFACE_Z, SubsystemPosition::XFromRadius)
    } else {
        (
            SUBSYSTEM_Z,
            if matches!(interface_type, InterfaceType::Export) {
                SubsystemPosition::Right
            } else {
                angle += std::f32::consts::PI;
                angle %= 2.0 * std::f32::consts::PI;
                SubsystemPosition::Left
            },
        )
    };

    let (subsystem_entity, _, _) = spawn_subsystem_common(
        commands,
        system_query,
        nesting_level_query,
        meshes,
        zoom,
        name,
        description,
        angle,
        parent_system,
        z,
        pos,
    );

    let mut subsystem_commands = commands.entity(subsystem_entity);

    subsystem_commands.insert(interface_subsystem);

    if is_import_subsystem {
        subsystem_commands.insert(ImportSubsystem);
    }
    if is_export_subsystem {
        subsystem_commands.insert(ExportSubsystem);
    }

    let subsystem_entity = subsystem_commands.id();

    let mut interface_commands = commands.entity(interface_entity);
    interface_commands.insert(InterfaceSubsystemConnection {
        target: subsystem_entity,
    });

    if is_child_of_interface {
        interface_commands.add_child(subsystem_entity);
    } else {
        commands.entity(parent_system).add_child(subsystem_entity);
    }

    subsystem_entity
}

enum SubsystemPosition {
    XFromRadius,
    Right,
    Left,
    Position(Vec2),
}

fn spawn_subsystem_common(
    commands: &mut Commands,
    system_query: &Query<(
        &Transform,
        &crate::bevy_app::components::System,
        &Name,
        &ElementDescription,
    )>,
    nesting_level_query: &Query<&NestingLevel>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    name: &str,
    description: &str,
    angle: f32,
    parent_system: Entity,
    z: f32,
    position: SubsystemPosition,
) -> (Entity, f32, u16) {
    let parent = system_query
        .get(parent_system)
        .expect("focused system not found");

    let parent_radius = parent.1.radius;

    let radius = parent_radius * SUBSYSTEM_MIN_SCALING_FACTOR;

    let nesting_level = NestingLevel::current(parent_system, nesting_level_query) + 1;

    let position = match position {
        SubsystemPosition::XFromRadius => vec2(-radius * zoom, 0.0),
        SubsystemPosition::Left => vec2((radius - parent_radius) * zoom, 0.0),
        SubsystemPosition::Right => vec2((parent_radius - radius) * zoom, 0.0),
        SubsystemPosition::Position(position) => position,
    };

    (
        commands
            .spawn((
                ParentState {
                    name: parent.2.as_str().to_owned(),
                    description: parent.3.text.clone(),
                },
                Subsystem { parent_system },
                NestingLevel::new(nesting_level),
                SystemBundle::new(
                    position,
                    z,
                    radius,
                    angle,
                    Complexity::default(),
                    Default::default(),
                    meshes,
                    zoom,
                    nesting_level,
                    name,
                    description,
                    "",
                    "Second",
                ),
            ))
            .observe(
                |trigger: Trigger<DragPosition>, mut writer: EventWriter<SubsystemDrag>| {
                    writer.send(trigger.into());
                },
            )
            .id(),
        radius,
        nesting_level,
    )
}

pub fn spawn_subsystem(
    commands: &mut Commands,
    parent_system: Entity,
    system_query: &Query<(
        &Transform,
        &crate::bevy_app::components::System,
        &Name,
        &ElementDescription,
    )>,
    nesting_level_query: &Query<&NestingLevel>,
    flow_query: &Query<(&FlowCurve, &Flow)>,
    inflows: &[Entity],
    outflows: &[Entity],
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
    zoom: f32,
    name: &str,
    description: &str,
    position: Vec2,
) -> Entity {
    let (subsystem_entity, radius, nesting_level) = spawn_subsystem_common(
        commands,
        system_query,
        nesting_level_query,
        meshes,
        zoom,
        name,
        description,
        0.0,
        parent_system,
        SUBSYSTEM_Z,
        SubsystemPosition::Position(position),
    );

    let zoomed_radius = radius * zoom;

    commands.entity(parent_system).add_child(subsystem_entity);

    for inflow in inflows {
        let (flow_curve, flow) = flow_query.get(*inflow).expect("Inflow not found");

        let (_, dir) = compute_end_and_direction_from_subsystem(
            position,
            zoomed_radius,
            flow_curve.start,
            flow_curve.start_direction,
        );
        let transform = transform_from_point2d_and_direction(dir * zoomed_radius, dir);

        spawn_interface(
            commands,
            InterfaceType::Import,
            flow.substance_type,
            *inflow,
            &transform,
            nesting_level,
            subsystem_entity,
            fixed_system_element_geometries,
            zoom,
            true,
            meshes,
            tess,
            "Interface",
            "",
        );

        commands.entity(*inflow).insert(FlowEndConnection {
            target: subsystem_entity,
            target_type: EndTargetType::System,
        });
    }

    for outflow in outflows {
        let (flow_curve, flow) = flow_query.get(*outflow).expect("Outflow not found");

        let (_, dir) = compute_end_and_direction_from_subsystem(
            position,
            zoomed_radius,
            flow_curve.end,
            flow_curve.end_direction,
        );
        let transform = transform_from_point2d_and_direction(dir * zoomed_radius, dir);

        spawn_interface(
            commands,
            InterfaceType::Export,
            flow.substance_type,
            *outflow,
            &transform,
            nesting_level,
            subsystem_entity,
            fixed_system_element_geometries,
            zoom,
            true,
            meshes,
            tess,
            "Interface",
            "",
        );

        commands.entity(*outflow).insert(FlowStartConnection {
            target: subsystem_entity,
            target_type: StartTargetType::System,
        });
    }

    subsystem_entity
}

pub fn auto_spawn_subsystem_label(
    mut commands: Commands,
    subsystem_query: Query<(Entity, &NestingLevel), Added<Subsystem>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
) {
    for (subsystem, nesting_level) in subsystem_query.iter() {
        add_name_label(
            &mut commands,
            subsystem,
            vec2(100.0, 100.0),
            None,
            Some(CopyPositionArgs {
                offset: vec3(0.0, 0.0, 0.0),
                horizontal_alignment: Alignment::Center,
                vertical_alignment: Alignment::Center,
                horizontal_anchor: HorizontalAttachmentAnchor::default(),
                vertical_anchor: VerticalAttachmentAnchor::default(),
            }),
            false,
            &name_query,
            &asset_server,
            Some(AutoContrastTextColor::default()),
            *nesting_level,
        );
    }
}
