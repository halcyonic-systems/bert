use crate::bevy_app::components::*;
use crate::bevy_app::constants::EXTERNAL_ENTITY_WIDTH_HALF;
use crate::bevy_app::events::*;
use crate::bevy_app::resources::Zoom;
use crate::bevy_app::utils::{
    compute_end_and_direction_from_subsystem, compute_end_and_direction_from_system_child,
};
use bevy::prelude::*;

pub fn drag_subsystem(
    mut events: EventReader<SubsystemDrag>,
    mut subsystem_query: Query<(&mut Transform, &Subsystem, Option<&InterfaceSubsystem>)>,
    system_query: Query<&crate::bevy_app::components::System>,
    zoom: Res<Zoom>,
) {
    for event in events.read() {
        let (mut transform, subsystem, interface_subsystem) = subsystem_query
            .get_mut(event.target)
            .expect("Subsystem should exist");

        // TODO : drag interface subsystems
        if interface_subsystem.is_some() {
            continue;
        }

        transform.translation.x = event.position.x;
        transform.translation.y = event.position.y;

        let parent_system = system_query
            .get(subsystem.parent_system)
            .expect("Parent system has to exist");
        transform.translation = transform
            .translation
            .truncate()
            .clamp_length_max(parent_system.radius * **zoom)
            .extend(transform.translation.z);
    }
}

pub fn drag_external_entity(
    mut events: EventReader<ExternalEntityDrag>,
    mut transform_query: Query<&mut Transform>,
    flow_query: Query<(&FlowStartConnection, &FlowEndConnection, &FlowCurve), With<FlowCurve>>,
    subsystem_query: Query<&Subsystem>,
    system_query: Query<&crate::bevy_app::components::System>,
    zoom: Res<Zoom>,
) {
    for event in events.read() {
        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x = event.position.x;
        transform.translation.y = event.position.y;

        let mut system = Entity::PLACEHOLDER;
        let mut other_end = Vec2::ZERO;
        let mut other_end_direction = Vec2::ZERO;
        let mut tangent_len = 0.0;

        for (start_connection, end_connection, flow_curve) in &flow_query {
            if start_connection.target == event.target {
                debug_assert!(end_connection.target_type == EndTargetType::System);
                system = end_connection.target;
                other_end = flow_curve.end;
                other_end_direction = flow_curve.end_direction;
                tangent_len = flow_curve.compute_tangent_length();
                break;
            } else if end_connection.target == event.target {
                debug_assert!(start_connection.target_type == StartTargetType::System);
                system = start_connection.target;
                other_end = flow_curve.start;
                other_end_direction = flow_curve.start_direction;
                tangent_len = flow_curve.compute_tangent_length();
                break;
            }
        }

        transform.rotation = compute_external_entity_rotation(
            event.position,
            other_end,
            other_end_direction,
            tangent_len,
        );

        if let Ok(subsystem) = subsystem_query.get(system) {
            let parent_system = system_query
                .get(subsystem.parent_system)
                .expect("Parent system has to exist");
            transform.translation = transform
                .translation
                .truncate()
                .clamp_length_max(parent_system.radius * **zoom)
                .extend(transform.translation.z);
        }
    }
}

pub fn update_flow_from_external_entity(
    external_entity_query: Query<
        (Entity, &Transform, &NestingLevel),
        (With<ExternalEntity>, Changed<Transform>),
    >,
    mut flow_query: Query<(&mut FlowCurve, &FlowStartConnection, &FlowEndConnection)>,
    zoom: Res<Zoom>,
) {
    for (target, transform, nesting_level) in &external_entity_query {
        for (mut flow_curve, flow_start_connection, flow_end_connection) in &mut flow_query {
            let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

            if flow_start_connection.target == target {
                let right = transform.right().truncate();
                flow_curve.start =
                    transform.translation.truncate() - right * EXTERNAL_ENTITY_WIDTH_HALF * scale;
                flow_curve.start_direction = -right;
            } else if flow_end_connection.target == target {
                let right = transform.right().truncate();
                flow_curve.end =
                    transform.translation.truncate() - right * EXTERNAL_ENTITY_WIDTH_HALF * scale;
                flow_curve.end_direction = -right;
            } else {
                continue;
            }
        }
    }
}

/// Updates flow curves when interfaces move.
///
/// Phase 3A+: Handles directional arrow rendering for N vs G networks:
/// - **G network** (Interface ↔ ExternalEntity): Arrows point outward (toward environment)
/// - **N network** (Interface ↔ Subsystem): Arrows point inward (toward system center)
/// - **Interface ↔ Interface**: Arrows point toward each other (special case)
pub fn update_flow_from_interface(
    interface_query: Query<(Entity, &NestingLevel), (With<Interface>, Changed<GlobalTransform>)>,
    all_interface_query: Query<(Entity, &NestingLevel), With<Interface>>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>,
    mut flow_query: Query<(
        &mut FlowCurve,
        Option<&Parent>,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
        Option<&FlowStartConnection>,
        Option<&FlowEndConnection>,
    )>,
    zoom: Res<Zoom>,
) {
    for (target, nesting_level) in &interface_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

        for (
            mut flow_curve,
            flow_parent,
            flow_start_interface_conn,
            flow_end_interface_conn,
            flow_start_conn,
            flow_end_conn,
        ) in &mut flow_query
        {
            let flow_parent = flow_parent.map(|p| p.get());

            // Check if this is Interface ↔ Interface connection
            let is_interface_to_interface =
                flow_start_interface_conn.is_some() && flow_end_interface_conn.is_some();

            if is_interface_to_interface {
                let start_interface = flow_start_interface_conn.unwrap();
                let end_interface = flow_end_interface_conn.unwrap();

                // Only update if the changed interface is one of the endpoints
                if start_interface.target == target || end_interface.target == target {
                    if let (Ok((start_target, start_nesting)), Ok((end_target, end_nesting))) = (
                        all_interface_query.get(start_interface.target),
                        all_interface_query.get(end_interface.target),
                    ) {
                        let start_scale = NestingLevel::compute_scale(**start_nesting, **zoom);
                        let end_scale = NestingLevel::compute_scale(**end_nesting, **zoom);

                        // Get positions (outer edges of interfaces)
                        let (start_pos, _) = compute_end_and_direction_from_system_child(
                            start_target,
                            &transform_query,
                            &parent_query,
                            flow_parent,
                            start_scale,
                            false,
                        );
                        let (end_pos, _) = compute_end_and_direction_from_system_child(
                            end_target,
                            &transform_query,
                            &parent_query,
                            flow_parent,
                            end_scale,
                            false,
                        );

                        // Compute directions pointing toward each other
                        let to_end = (end_pos - start_pos).normalize_or_zero();
                        let to_start = -to_end;

                        flow_curve.start = start_pos;
                        flow_curve.start_direction = to_end;
                        flow_curve.end = end_pos;
                        flow_curve.end_direction = to_start;
                    }
                }
            } else {
                // Standard handling: Interface ↔ Subsystem or Interface ↔ ExternalEntity
                if let Some(flow_end_interface_conn) = flow_end_interface_conn {
                    if flow_end_interface_conn.target == target {
                        // Determine if this is N network (invert direction) or G network (normal)
                        // N network: Interface connected to System (not external entity)
                        // G network: Interface connected to Source/Sink (external entity)
                        let is_n_network = flow_start_conn
                            .map(|c| !c.target_is_external_entity())
                            .unwrap_or(false);

                        let (end, dir) = compute_end_and_direction_from_system_child(
                            target,
                            &transform_query,
                            &parent_query,
                            flow_parent,
                            scale,
                            is_n_network, // Invert direction for N network
                        );
                        flow_curve.end = end;
                        flow_curve.end_direction = dir;
                    }
                }
                if let Some(flow_start_interface_conn) = flow_start_interface_conn {
                    if flow_start_interface_conn.target == target {
                        // Determine if this is N network (invert direction) or G network (normal)
                        let is_n_network = flow_end_conn
                            .map(|c| !c.target_is_external_entity())
                            .unwrap_or(false);

                        let (start, dir) = compute_end_and_direction_from_system_child(
                            target,
                            &transform_query,
                            &parent_query,
                            flow_parent,
                            scale,
                            is_n_network, // Invert direction for N network
                        );
                        flow_curve.start = start;
                        flow_curve.start_direction = dir;
                    }
                }
            }
        }
    }
}

/// Initializes flow curves for newly-added flows with interface connections.
///
/// This handles the case where flows are loaded from a file - the regular
/// `update_flow_from_interface` system uses `Changed<GlobalTransform>` which
/// may not fire on first load in web WASM due to transform propagation timing.
///
/// Special handling for Interface ↔ Interface connections:
/// - Both ends connect to interfaces on different subsystems
/// - Directions must point toward each other, not toward subsystem centers
pub fn initialize_flow_curves_on_load(
    interface_query: Query<(Entity, &NestingLevel), With<Interface>>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>,
    mut flow_query: Query<
        (
            &mut FlowCurve,
            Option<&Parent>,
            Option<&FlowStartInterfaceConnection>,
            Option<&FlowEndInterfaceConnection>,
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
        ),
        Or<(
            Added<FlowStartInterfaceConnection>,
            Added<FlowEndInterfaceConnection>,
        )>,
    >,
    zoom: Res<Zoom>,
) {
    for (
        mut flow_curve,
        flow_parent,
        flow_start_interface_conn,
        flow_end_interface_conn,
        flow_start_conn,
        flow_end_conn,
    ) in &mut flow_query
    {
        let flow_parent = flow_parent.map(|p| p.get());

        // Check if this is Interface ↔ Interface connection
        let is_interface_to_interface =
            flow_start_interface_conn.is_some() && flow_end_interface_conn.is_some();

        if is_interface_to_interface {
            // Interface ↔ Interface: compute positions first, then directions toward each other
            let start_interface = flow_start_interface_conn.unwrap();
            let end_interface = flow_end_interface_conn.unwrap();

            if let (Ok((start_target, start_nesting)), Ok((end_target, end_nesting))) = (
                interface_query.get(start_interface.target),
                interface_query.get(end_interface.target),
            ) {
                let start_scale = NestingLevel::compute_scale(**start_nesting, **zoom);
                let end_scale = NestingLevel::compute_scale(**end_nesting, **zoom);

                // Get positions (outer edges of interfaces)
                let (start_pos, _) = compute_end_and_direction_from_system_child(
                    start_target,
                    &transform_query,
                    &parent_query,
                    flow_parent,
                    start_scale,
                    false, // direction doesn't matter, we'll override
                );
                let (end_pos, _) = compute_end_and_direction_from_system_child(
                    end_target,
                    &transform_query,
                    &parent_query,
                    flow_parent,
                    end_scale,
                    false,
                );

                // Compute directions pointing toward each other
                let to_end = (end_pos - start_pos).normalize_or_zero();
                let to_start = -to_end;

                flow_curve.start = start_pos;
                flow_curve.start_direction = to_end;
                flow_curve.end = end_pos;
                flow_curve.end_direction = to_start;
            }
        } else {
            // Standard handling: Interface ↔ Subsystem or Interface ↔ ExternalEntity

            // Handle end interface connection
            if let Some(flow_end_interface_conn) = flow_end_interface_conn {
                if let Ok((target, nesting_level)) =
                    interface_query.get(flow_end_interface_conn.target)
                {
                    let scale = NestingLevel::compute_scale(**nesting_level, **zoom);
                    let is_n_network = flow_start_conn
                        .map(|c| !c.target_is_external_entity())
                        .unwrap_or(false);

                    let (end, dir) = compute_end_and_direction_from_system_child(
                        target,
                        &transform_query,
                        &parent_query,
                        flow_parent,
                        scale,
                        is_n_network,
                    );
                    flow_curve.end = end;
                    flow_curve.end_direction = dir;
                }
            }

            // Handle start interface connection
            if let Some(flow_start_interface_conn) = flow_start_interface_conn {
                if let Ok((target, nesting_level)) =
                    interface_query.get(flow_start_interface_conn.target)
                {
                    let scale = NestingLevel::compute_scale(**nesting_level, **zoom);
                    let is_n_network = flow_end_conn
                        .map(|c| !c.target_is_external_entity())
                        .unwrap_or(false);

                    let (start, dir) = compute_end_and_direction_from_system_child(
                        target,
                        &transform_query,
                        &parent_query,
                        flow_parent,
                        scale,
                        is_n_network,
                    );
                    flow_curve.start = start;
                    flow_curve.start_direction = dir;
                }
            }
        }
    }
}

pub fn update_flow_from_subsystem_without_interface(
    system_query: Query<
        (
            Entity,
            &GlobalTransform,
            &crate::bevy_app::components::System,
        ),
        Or<(
            Changed<GlobalTransform>,
            Changed<crate::bevy_app::components::System>,
        )>,
    >,
    mut flow_query: Query<
        (
            &mut FlowCurve,
            &GlobalTransform,
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
            Option<&FlowStartInterfaceConnection>,
            Option<&FlowEndInterfaceConnection>,
        ),
        Or<(
            Without<FlowStartInterfaceConnection>,
            Without<FlowEndInterfaceConnection>,
        )>,
    >,
    zoom: Res<Zoom>,
) {
    for (target, system_transform, system) in &system_query {
        for (
            mut flow_curve,
            flow_transform,
            flow_start_connection,
            flow_end_connection,
            flow_start_interface_connection,
            flow_end_interface_connection,
        ) in &mut flow_query
        {
            let flow_transform_inverse = flow_transform.affine().inverse();
            let system_pos = flow_transform_inverse
                .transform_point3(system_transform.translation())
                .truncate();

            if let (Some(flow_end_connection), None) =
                (flow_end_connection, flow_end_interface_connection)
            {
                if flow_end_connection.target == target {
                    let (end, end_direction) = compute_end_and_direction_from_subsystem(
                        system_pos,
                        system.radius * **zoom,
                        flow_curve.start,
                        flow_curve.start_direction,
                    );

                    flow_curve.end = end;
                    flow_curve.end_direction = end_direction;
                }
            }

            if let (Some(flow_start_connection), None) =
                (flow_start_connection, flow_start_interface_connection)
            {
                if flow_start_connection.target == target {
                    let (start, start_direction) = compute_end_and_direction_from_subsystem(
                        system_pos,
                        system.radius * **zoom,
                        flow_curve.end,
                        flow_curve.end_direction,
                    );

                    flow_curve.start = start;
                    flow_curve.start_direction = start_direction;
                }
            }
        }
    }
}

pub fn drag_interface(
    mut events: EventReader<InterfaceDrag>,
    mut transform_query: Query<&mut Transform, Without<crate::bevy_app::components::System>>,
    parent_query: Query<&Parent>,
    system_query: Query<&crate::bevy_app::components::System>,
    flow_query: Query<(
        &FlowCurve,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
        &FlowStartConnection,
        &FlowEndConnection,
    )>,
    zoom: Res<Zoom>,
) {
    for event in events.read() {
        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x = event.position.x;
        transform.translation.y = event.position.y;

        let parent_system_entity = parent_query
            .get(event.target)
            .expect("Parent should exist")
            .get();

        let system = system_query
            .get(parent_system_entity)
            .expect("System should have a Transform");

        let interface_pos = transform.translation.truncate();

        let mut pos = interface_pos;
        pos *= system.radius * **zoom / pos.length();

        transform.rotation = Quat::from_rotation_z(pos.to_angle());

        transform.translation = pos.extend(transform.translation.z);

        let mut external_entity = Entity::PLACEHOLDER;
        let mut external_entity_pos = Vec2::ZERO;
        let mut other_end = Vec2::ZERO;
        let mut other_end_direction = Vec2::ZERO;
        let mut tangent_len = 0.0;

        for (
            flow_curve,
            flow_start_interface_connection,
            flow_end_interface_connection,
            flow_start_connection,
            flow_end_connection,
        ) in &flow_query
        {
            if let Some(flow_start_interface_connection) = flow_start_interface_connection {
                if flow_start_interface_connection.target == event.target
                    && flow_end_connection.target_type == EndTargetType::Sink
                {
                    external_entity = flow_end_connection.target;
                    external_entity_pos = flow_curve.end;
                    other_end = flow_curve.start;
                    other_end_direction = flow_curve.start_direction;
                    tangent_len = flow_curve.compute_tangent_length();
                    break; // TODO : multiconnection
                }
            }
            if let Some(flow_end_interface_connection) = flow_end_interface_connection {
                if flow_end_interface_connection.target == event.target
                    && flow_start_connection.target_type == StartTargetType::Source
                {
                    external_entity = flow_start_connection.target;
                    external_entity_pos = flow_curve.start;
                    other_end = flow_curve.end;
                    other_end_direction = flow_curve.end_direction;
                    tangent_len = flow_curve.compute_tangent_length();
                    break; // TODO : multiconnection
                }
            }
        }

        if let Ok(mut transform) = transform_query.get_mut(external_entity) {
            transform.rotation = compute_external_entity_rotation(
                external_entity_pos,
                other_end,
                other_end_direction,
                tangent_len,
            );
        }
    }
}

pub fn update_initial_position_from_transform(
    mut query: Query<(&mut InitialPosition, &Transform), Changed<Transform>>,
    zoom: Res<Zoom>,
) {
    for (mut initial_position, transform) in &mut query {
        **initial_position = transform.translation.truncate() / **zoom;
    }
}

fn compute_external_entity_rotation(
    pos: Vec2,
    other_end: Vec2,
    other_end_direction: Vec2,
    tangent_len: f32,
) -> Quat {
    let dir =
        -compute_smooth_flow_terminal_direction(pos, other_end, other_end_direction, tangent_len);
    Quat::from_rotation_z(dir.to_angle())
}

pub fn compute_smooth_flow_terminal_direction(
    pos: Vec2,
    other_end: Vec2,
    other_end_direction: Vec2,
    tangent_len: f32,
) -> Vec2 {
    other_end + other_end_direction * tangent_len - pos
}

pub fn update_interface_button_from_interaction(
    interaction_query: Query<
        (
            &FlowCurve,
            &HasFlowInterfaceButton,
            &GlobalTransform,
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
        ),
        Changed<FlowCurve>,
    >,
    global_transform_query: Query<&GlobalTransform>,
    mut transform_query: Query<&mut Transform>,
    parent_query: Query<&Parent>,
) {
    for (
        flow_curve,
        has_interface_button,
        interaction_transform,
        start_connection,
        end_connection,
    ) in &interaction_query
    {
        let parent_system_entity = parent_query
            .get(has_interface_button.button_entity)
            .expect("Parent should exist")
            .get();

        let system_transform = global_transform_query
            .get(parent_system_entity)
            .expect("System should have a GlobalTransform");

        let interaction_to_system =
            system_transform.affine().inverse() * interaction_transform.affine();

        if let Some(start_connection) = start_connection {
            if start_connection.target == parent_system_entity {
                let mut transform = transform_query
                    .get_mut(has_interface_button.button_entity)
                    .expect("Button should have a Transform");

                transform.translation = interaction_to_system
                    .transform_point3(flow_curve.start.extend(0.0))
                    .truncate()
                    .extend(transform.translation.z);
            }
        }

        if let Some(end_connection) = end_connection {
            if end_connection.target == parent_system_entity {
                let mut transform = transform_query
                    .get_mut(has_interface_button.button_entity)
                    .expect("Button should have a Transform");

                transform.translation = interaction_to_system
                    .transform_point3(flow_curve.end.extend(0.0))
                    .truncate()
                    .extend(transform.translation.z);
            }
        }
    }
}
