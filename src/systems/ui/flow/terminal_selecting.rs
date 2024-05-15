use crate::bundles::spawn_external_entity;
use crate::components::*;
use crate::plugins::mouse_interaction::MouseWorldPosition;
use crate::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Zoom,
};
use crate::states::AppState;
use crate::systems::compute_smooth_flow_terminal_direction;
use crate::utils::transform_from_point2d_and_direction;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
            Option<&crate::components::System>,
            // TODO : Option<&Interface>,
            &GlobalTransform,
            Option<&NestingLevel>,
        ),
        Changed<PickingInteraction>,
    >,
    mut flow_query: Query<(
        Entity,
        &Flow,
        &mut FlowCurve,
        &FlowTerminalSelecting,
        &GlobalTransform,
        &NestingLevel,
    )>,
    subsystem_query: Query<&Subsystem>,
    nesting_level_query: Query<&NestingLevel>,
    mut next_state: ResMut<NextState<AppState>>,
    focused_system: Res<FocusedSystem>,
    mut fixed_system_element_geometries: ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tess: ResMut<StrokeTessellator>,
) {
    let (flow_entity, flow, mut flow_curve, selecting, flow_transform, flow_nesting_level) =
        flow_query.single_mut();

    for (target_entity, interaction, system, target_transform, target_nesting_level) in
        &interaction_query
    {
        if matches!(*interaction, PickingInteraction::Pressed) {
            if let Some(system) = system {
                let target_nesting_level = target_nesting_level.map(|n| **n).unwrap_or(0);

                if **flow_nesting_level == target_nesting_level {
                    let mut flow_commands = commands.entity(flow_entity);
                    flow_commands.remove::<FlowTerminalSelecting>();

                    let flow_transform_inverse = flow_transform.affine().inverse();
                    let target_pos = flow_transform_inverse
                        .transform_point3(target_transform.translation())
                        .truncate();

                    match selecting {
                        FlowTerminalSelecting::Start => {
                            let mut start_direction = compute_smooth_flow_terminal_direction(
                                target_pos,
                                flow_curve.end,
                                flow_curve.end_direction,
                                FlowCurve::compute_tangent_length_from_points(
                                    target_pos,
                                    flow_curve.end,
                                ),
                            )
                            .normalize();

                            start_direction += (flow_curve.end - target_pos).normalize();
                            start_direction = start_direction.normalize();

                            flow_curve.start = target_pos + start_direction * system.radius;
                            flow_curve.start_direction = start_direction;

                            // TODO : move all the geometric stuff above to update drag system?
                            flow_commands.insert(FlowStartConnection {
                                target: target_entity,
                                target_type: StartTargetType::System,
                            });
                        }
                        FlowTerminalSelecting::End => {
                            let mut end_direction = compute_smooth_flow_terminal_direction(
                                target_pos,
                                flow_curve.start,
                                flow_curve.start_direction,
                                FlowCurve::compute_tangent_length_from_points(
                                    target_pos,
                                    flow_curve.start,
                                ),
                            )
                            .normalize();

                            end_direction += (flow_curve.start - target_pos).normalize();
                            end_direction = end_direction.normalize();

                            flow_curve.end = target_pos + end_direction * system.radius;
                            flow_curve.end_direction = end_direction;

                            // TODO : move all the geometric stuff above to update drag system?
                            flow_commands.insert(FlowEndConnection {
                                target: target_entity,
                                target_type: EndTargetType::System,
                            });
                        }
                    }

                    next_state.set(AppState::Normal);
                } else if **flow_nesting_level - 1 == target_nesting_level {
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
                            );
                        }
                    }

                    next_state.set(AppState::Normal);
                }
            }
        }
    }
}
