use crate::components::*;
use crate::constants::{EXTERNAL_ENTITY_WIDTH_HALF, FLOW_END_LENGTH};
use crate::events::*;
use crate::resources::Zoom;
use crate::utils::compute_end_and_direction_from_system_child;
use bevy::prelude::*;
use std::ops::DerefMut;

pub fn drag_external_entity(
    mut events: EventReader<ExternalEntityDrag>,
    mut transform_query: Query<&mut Transform>,
    flow_query: Query<(&FlowStartConnection, &FlowEndConnection), With<FlowCurve>>,
    subsystem_query: Query<&Subsystem>,
    system_query: Query<&crate::components::System>,
    zoom: Res<Zoom>,
) {
    for event in events.read() {
        if event.has_bubbled() {
            continue;
        }

        let mut transform = transform_query
            .get_mut(event.target)
            .expect("External entity should have a Transform");
        transform.translation.x = event.position.x;
        transform.translation.y = event.position.y;

        let mut system = Entity::PLACEHOLDER;

        for (start_connection, end_connection) in &flow_query {
            if start_connection.target == event.target {
                debug_assert!(end_connection.target_type == EndTargetType::System);
                system = end_connection.target;
                break;
            } else if end_connection.target == event.target {
                debug_assert!(start_connection.target_type == StartTargetType::System);
                system = start_connection.target;
                break;
            }
        }

        if let Ok(subsystem) = subsystem_query.get(system) {
            let parent_system = system_query
                .get(subsystem.parent_system)
                .expect("Parent system has to exist");
            transform.translation = transform
                .translation
                .clamp_length_max(parent_system.radius * **zoom);
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
            } else if flow_end_connection.target == target {
                let right = transform.right().truncate();
                flow_curve.end =
                    transform.translation.truncate() - right * EXTERNAL_ENTITY_WIDTH_HALF * scale;
            } else {
                continue;
            }
        }
    }
}

pub fn update_external_entity_from_flow(
    mut external_entity_query: Query<(Entity, &mut Transform, &NestingLevel), With<ExternalEntity>>,
    mut flow_query: Query<
        (&mut FlowCurve, &FlowStartConnection, &FlowEndConnection),
        Changed<FlowCurve>,
    >,
    zoom: Res<Zoom>,
) {
    for (mut flow_curve, flow_start_connection, flow_end_connection) in &mut flow_query {
        for (target, mut transform, nesting_level) in &mut external_entity_query {
            let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

            let right = if flow_start_connection.target == target {
                let right = (transform.translation.truncate() - flow_curve.end - flow_curve.end_direction).normalize();
                flow_curve.start_direction = -right * FLOW_END_LENGTH * scale;

                right
            } else if flow_end_connection.target == target {
                let right = (transform.translation.truncate() - flow_curve.start - flow_curve.start_direction).normalize();
                flow_curve.end_direction = -right * FLOW_END_LENGTH * scale;

                right
            } else {
                continue;
            };

            transform.rotation = Quat::from_rotation_z(right.to_angle());
        }
    }
}

pub fn update_flow_from_interface(
    interface_query: Query<(Entity, &NestingLevel), (With<Interface>, Changed<Transform>)>,
    transform_query: Query<&Transform>,
    parent_query: Query<&Parent>,
    mut flow_query: Query<(
        &mut FlowCurve,
        Option<&Parent>,
        Option<&FlowStartInterfaceConnection>,
        Option<&FlowEndInterfaceConnection>,
    )>,
    zoom: Res<Zoom>,
) {
    for (target, nesting_level) in &interface_query {
        let scale = NestingLevel::compute_scale(**nesting_level, **zoom);

        for (mut flow_curve, flow_parent, flow_start_connection, flow_end_connection) in
            &mut flow_query
        {
            let flow_parent = flow_parent.map(|p| p.get());

            if let Some(flow_end_connection) = flow_end_connection {
                if flow_end_connection.target == target {
                    let (end, dir) = compute_end_and_direction_from_system_child(
                        target,
                        &transform_query,
                        &parent_query,
                        flow_parent,
                        **zoom,
                        scale,
                    );
                    flow_curve.end = end;
                    flow_curve.end_direction = dir;
                }
            }
            if let Some(flow_start_connection) = flow_start_connection {
                if flow_start_connection.target == target {
                    let (start, dir) = compute_end_and_direction_from_system_child(
                        target,
                        &transform_query,
                        &parent_query,
                        flow_parent,
                        **zoom,
                        scale,
                    );
                    flow_curve.start = start;
                    flow_curve.start_direction = dir;
                }
            }
        }
    }
}

pub fn drag_interface(
    mut events: EventReader<InterfaceDrag>,
    mut transform_query: Query<&mut Transform, Without<crate::components::System>>,
    parent_query: Query<&Parent>,
    system_query: Query<&crate::components::System>,
    zoom: Res<Zoom>,
) {
    for event in events.read() {
        if event.has_bubbled() {
            continue;
        }

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

pub fn update_flow_from_system(
    system_query: Query<&Children, (With<crate::components::System>, Changed<Transform>)>,
    mut interface_query: Query<
        &mut Transform,
        (With<Interface>, Without<crate::components::System>),
    >,
) {
    for children in &system_query {
        for child in children.iter() {
            if let Ok(mut transform) = interface_query.get_mut(*child) {
                // touch to trigger flow updates
                let _ = transform.deref_mut();
            }
        }
    }
}

pub fn update_flow_from_interface_subsystem(
    interface_query: Query<&InterfaceSubsystemConnection, Changed<Transform>>,
    mut system_query: Query<
        &mut Transform,
        (
            With<crate::components::System>,
            Without<InterfaceSubsystemConnection>,
        ),
    >,
) {
    for interface_subsystem_connection in &interface_query {
        if let Ok(mut transform) = system_query.get_mut(interface_subsystem_connection.target) {
            // touch to trigger flow updates
            let _ = transform.deref_mut();
        }
    }
}
