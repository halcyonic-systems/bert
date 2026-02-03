use crate::bevy_app::bundles::spawn_external_entity;
use crate::bevy_app::components::*;
use crate::bevy_app::plugins::mouse_interaction::{MouseWorldPosition, PickTarget};
use crate::bevy_app::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Zoom,
};
use crate::bevy_app::states::AppState;
use crate::bevy_app::systems::compute_smooth_flow_terminal_direction;
use crate::bevy_app::utils::transform_from_point2d_and_direction;
use bevy::picking::hover::PickingInteraction;
use bevy::prelude::*;
use std::ops::DerefMut;

pub fn update_selecting_flow_from_mouse(
    mouse_position: Res<MouseWorldPosition>,
    mut flow_query: Query<(&mut FlowCurve, &FlowTerminalSelecting, &GlobalTransform)>,
) {
    for (mut flow_curve, selecting, transform) in &mut flow_query {
        let mouse_pos = (**mouse_position).extend(0.0);
        let mouse_pos = transform
            .affine()
            .inverse()
            .transform_point3(mouse_pos)
            .truncate();

        match selecting {
            FlowTerminalSelecting::Start => {
                flow_curve.start = mouse_pos;
                flow_curve.start_direction = compute_smooth_flow_terminal_direction(
                    mouse_pos,
                    flow_curve.end,
                    flow_curve.end_direction,
                    flow_curve.compute_tangent_length(),
                )
                .normalize();
            }
            FlowTerminalSelecting::End => {
                flow_curve.end = mouse_pos;
                flow_curve.end_direction = compute_smooth_flow_terminal_direction(
                    mouse_pos,
                    flow_curve.start,
                    flow_curve.start_direction,
                    flow_curve.compute_tangent_length(),
                )
                .normalize();
            }
        }
    }
}

pub fn select_flow_terminal(
    mut commands: Commands,
    interaction_query: Query<
        (
            Entity,
            &PickingInteraction,
            Option<&SystemElement>,
            // TODO : Option<&Interface>,
            Option<&PickTarget>,
        ),
        Changed<PickingInteraction>,
    >,
    flow_query: Query<(
        Entity,
        &Flow,
        &FlowCurve,
        &FlowTerminalSelecting,
        &NestingLevel,
    )>,
    subsystem_query: Query<&Subsystem>,
    nesting_level_query: Query<&NestingLevel>,
    element_query: Query<&SystemElement>,
    mut transform_query: Query<&mut Transform>,
    mut next_state: ResMut<NextState<AppState>>,
    focused_system: Res<FocusedSystem>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tess: ResMut<StrokeTessellator>,
) {
    let (flow_entity, flow, flow_curve, selecting, flow_nesting_level) =
        flow_query.single().unwrap();
    let flow_nesting_level = **flow_nesting_level;

    let mut target_entity = Entity::PLACEHOLDER;

    for (entity, interaction, element, pick_target) in &interaction_query {
        if matches!(*interaction, PickingInteraction::Pressed) {
            if let Some(pick_target) = pick_target {
                target_entity = pick_target.target;
                break;
            } else if element.is_some() {
                target_entity = entity;
                break;
            }
        }
    }

    let target_nesting_level = nesting_level_query
        .get(target_entity)
        .map(|n| **n)
        .unwrap_or(0);

    if let Ok(element) = element_query.get(target_entity) {
        if flow_nesting_level == target_nesting_level && matches!(element, SystemElement::System) {
            let mut flow_commands = commands.entity(flow_entity);
            flow_commands.remove::<FlowTerminalSelecting>();

            match selecting {
                FlowTerminalSelecting::Start => {
                    flow_commands.insert(FlowStartConnection {
                        target: target_entity,
                        target_type: StartTargetType::System,
                    });
                }
                FlowTerminalSelecting::End => {
                    flow_commands.insert(FlowEndConnection {
                        target: target_entity,
                        target_type: EndTargetType::System,
                    });
                }
            }

            // touch to trigger curve update
            let _ = transform_query
                .get_mut(target_entity)
                .expect("Target should have a Transform")
                .deref_mut();

            next_state.set(AppState::Normal);
        } else if flow_nesting_level - 1 == target_nesting_level
            && matches!(element, SystemElement::System)
            || flow_nesting_level == target_nesting_level
                && !matches!(element, SystemElement::System)
        {
            commands
                .entity(flow_entity)
                .remove::<FlowTerminalSelecting>();

            match selecting {
                FlowTerminalSelecting::Start => {
                    spawn_external_entity(
                        &mut commands,
                        &subsystem_query,
                        &nesting_level_query,
                        **focused_system,
                        InterfaceType::Import,
                        flow.substance_type,
                        flow_entity,
                        &transform_from_point2d_and_direction(
                            flow_curve.start,
                            -flow_curve.start_direction,
                        ),
                        &mut fixed_system_element_geometries,
                        **zoom,
                        true,
                        &mut meshes,
                        &mut tess,
                        "Source",
                        "",
                        false,
                    );
                }
                FlowTerminalSelecting::End => {
                    spawn_external_entity(
                        &mut commands,
                        &subsystem_query,
                        &nesting_level_query,
                        **focused_system,
                        InterfaceType::Export,
                        flow.substance_type,
                        flow_entity,
                        &transform_from_point2d_and_direction(
                            flow_curve.end,
                            -flow_curve.end_direction,
                        ),
                        &mut fixed_system_element_geometries,
                        **zoom,
                        true,
                        &mut meshes,
                        &mut tess,
                        "Sink",
                        "",
                        false,
                    );
                }
            }

            next_state.set(AppState::Normal);
        }
    }
}
