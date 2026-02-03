use crate::bevy_app::components::{
    EndTargetType, FlowCurve, FlowEndConnection, FlowEndInterfaceConnection, FlowStartConnection,
    FlowStartInterfaceConnection, InitialPosition, InterfaceSubsystem, StartTargetType,
};
use crate::bevy_app::constants::INTERFACE_WIDTH_HALF;
use crate::bevy_app::systems::compute_smooth_flow_terminal_direction;
use bevy::prelude::*;

pub fn ui_transform_from_button(
    button_transform: &Transform,
    z: f32,
    move_right: f32,
    zoom: f32,
) -> (Transform, InitialPosition) {
    let position = button_transform.translation.truncate() / zoom;
    let right = button_transform.right().truncate();

    let position = position + right * move_right;
    (
        Transform::from_translation((position * zoom).extend(z))
            .with_rotation(button_transform.rotation),
        InitialPosition::new(position),
    )
}

/// Computes flow endpoint position and direction for interface connections.
///
/// # Parameters
/// - `invert_direction`: If true, reverses direction (for N network flows pointing inward)
///
/// # Direction Logic
/// - **G network** (Interface ↔ ExternalEntity): `invert_direction = false` → Arrow points outward
/// - **N network** (Interface ↔ Subsystem): `invert_direction = true` → Arrow points inward
pub fn compute_end_and_direction_from_system_child(
    system_child: Entity,
    transform_query: &Query<&Transform>,
    parent_query: &Query<&ChildOf>,
    flow_parent: Option<Entity>,
    scale: f32,
    invert_direction: bool,
) -> (Vec2, Vec2) {
    let combined_transform = combined_transform_of_entity_until_ancestor(
        system_child,
        flow_parent,
        transform_query,
        parent_query,
    );

    let right = combined_transform.right().truncate();
    let direction = if invert_direction { -right } else { right };

    (
        combined_transform.translation.truncate() + right * INTERFACE_WIDTH_HALF * scale,
        direction,
    )
}

pub fn compute_end_and_direction_from_subsystem(
    system_pos: Vec2,
    system_radius: f32,
    other_end: Vec2,
    other_end_direction: Vec2,
) -> (Vec2, Vec2) {
    let raw_direction = compute_smooth_flow_terminal_direction(
        system_pos,
        other_end,
        other_end_direction,
        FlowCurve::compute_tangent_length_from_points(system_pos, other_end),
    );

    // Use normalize_or_zero to avoid NaN, then fall back to a default direction
    // if the result is zero (e.g., when other_end == system_pos on initial load)
    let direction = raw_direction.normalize_or_zero();
    let direction = if direction == Vec2::ZERO {
        // Fall back to direction pointing toward other_end, or Vec2::X if coincident
        (other_end - system_pos).normalize_or(Vec2::X)
    } else {
        direction
    };

    (system_pos + direction * system_radius, direction)
}

pub fn combined_transform_of_entity_until_ancestor(
    entity: Entity,
    ancestor: Option<Entity>,
    transform_query: &Query<&Transform>,
    parent_query: &Query<&ChildOf>,
) -> Transform {
    let mut combined_transform = *transform_query
        .get(entity)
        .expect("Entity should have a Transform");
    let mut parent_entity = parent_query
        .get(entity)
        .expect("Entity should have a Parent")
        .parent();

    loop {
        let parent_transform = transform_query
            .get(parent_entity)
            .expect("Parent should have a Transform");

        combined_transform = parent_transform.mul_transform(combined_transform);

        if let Ok(parent) = parent_query.get(parent_entity) {
            parent_entity = parent.parent();
        } else {
            break;
        }

        if Some(parent_entity) == ancestor {
            break;
        }
    }

    combined_transform
}

macro_rules! all_flow_connected_systems {
    (
        $fn_name:ident,
        $conn_ty:ty,
        $interface_conn_ty:ty,
        $target_ty:tt
    ) => {
        pub fn $fn_name(
            flow: (Option<&$conn_ty>, Option<&$interface_conn_ty>),
            interface_subsystem_query: &Query<(Entity, &InterfaceSubsystem)>,
        ) -> Vec<Entity> {
            let mut connected_systems = vec![];

            let (connection, interface_connection) = flow;

            if let Some(connection) = connection {
                if matches!(connection.target_type, $target_ty::System) {
                    connected_systems.push(connection.target);
                }
            }

            if let Some(interface_connection) = interface_connection {
                let interface_entity = interface_connection.target;
                for (subsystem_entity, interface_subsystem) in interface_subsystem_query {
                    if interface_subsystem.interface_entity == interface_entity {
                        connected_systems.push(subsystem_entity);
                    }
                }
            }

            connected_systems
        }
    };
}

all_flow_connected_systems!(
    all_flow_end_connected_systems,
    FlowEndConnection,
    FlowEndInterfaceConnection,
    EndTargetType
);
all_flow_connected_systems!(
    all_flow_start_connected_systems,
    FlowStartConnection,
    FlowStartInterfaceConnection,
    StartTargetType
);

pub fn transform_from_point2d_and_direction(point2d: Vec2, direction: Vec2) -> Transform {
    Transform::from_translation(point2d.extend(0.0))
        .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x)))
}
