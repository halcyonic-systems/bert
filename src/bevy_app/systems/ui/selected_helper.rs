use crate::bevy_app::components::*;
use crate::bevy_app::constants::*;
use crate::bevy_app::plugins::mouse_interaction::{PickParent, PickSelection};
use crate::bevy_app::resources::*;
use crate::bevy_app::systems::create_flow_curve_shape;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_selected_system(
    mut commands: Commands,
    system_query: Query<
        (
            Entity,
            &PickSelection,
            &crate::bevy_app::components::System,
            &Name,
        ),
        (
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    zoom: Res<Zoom>,
) {
    for (selected_entity, selection, system, name) in &system_query {
        if selection.is_selected {
            let helper_shape = ShapeBuilder::with(&shapes::Circle {
                radius: system.radius * **zoom,
                ..default()
            })
            .stroke((Color::WHITE, SYSTEM_SELECTED_INNER_LINE_WIDTH))
            .build();

            let helper_entity = commands
                .spawn((PickParent, helper_shape, Transform::from_xyz(0.0, 0.0, 2.0)))
                .id();

            // Clone name to trigger change detection for leptos_bevy_canvas reactivity
            let name_clone = name.clone();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity })
                .insert(name_clone); // Re-insert Name to trigger Changed<Name>
        }
    }
}

pub fn spawn_selected_interface(
    mut commands: Commands,
    interface_query: Query<
        (Entity, &PickSelection, &NestingLevel),
        (
            With<Interface>,
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    fixed_system_element_geometries: Res<FixedSystemElementGeometriesByNestingLevel>,
) {
    for (selected_entity, selection, nesting_level) in &interface_query {
        if selection.is_selected {
            let mut geom = fixed_system_element_geometries
                .get(&**nesting_level)
                .expect("Geometries added in spawn_interface")
                .interface
                .clone();
            // Set stroke for selection highlight
            geom.shape.stroke = Some(Stroke::new(
                Color::WHITE,
                INTERFACE_SELECTED_INNER_LINE_WIDTH,
            ));

            let helper_entity = commands
                .spawn((Transform::from_xyz(0.0, 0.0, 1.0), PickParent, geom))
                .id();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn spawn_selected_flow(
    mut commands: Commands,
    curve_query: Query<
        (
            Entity,
            &FlowCurve,
            Option<&FlowEndpointOffset>,
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
            &PickSelection,
            &NestingLevel,
        ),
        (
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    subsystem_query: Query<(&GlobalTransform, &crate::bevy_app::components::System)>,
    parent_query: Query<&ChildOf>,
    global_transform_query: Query<&GlobalTransform>,
    zoom: Res<Zoom>,
) {
    for (entity, flow_curve, offset, start_conn, end_conn, selection, nesting_level) in
        &curve_query
    {
        if selection.is_selected {
            let mut adjusted_curve =
                crate::bevy_app::systems::ui::flow::compute_adjusted_curve(
                    flow_curve,
                    offset,
                    start_conn,
                    end_conn,
                    &subsystem_query,
                    **zoom,
                );

            if let Some(offset) = offset {
                if let Ok(parent) = parent_query.get(entity) {
                    if let Ok(parent_gt) = global_transform_query.get(parent.parent()) {
                        let parent_inv = parent_gt.affine().inverse();
                        if offset.start_angle.is_some() {
                            adjusted_curve.start = parent_inv
                                .transform_point3(adjusted_curve.start.extend(0.0))
                                .truncate();
                        }
                        if offset.end_angle.is_some() {
                            adjusted_curve.end = parent_inv
                                .transform_point3(adjusted_curve.end.extend(0.0))
                                .truncate();
                        }
                    }
                }
            }

            let mut curve_shape = create_flow_curve_shape(
                &adjusted_curve,
                NestingLevel::compute_scale(**nesting_level, **zoom),
            );
            curve_shape.stroke = Some(Stroke::new(Color::WHITE, FLOW_SELECTED_INNER_LINE_WIDTH));

            let helper_entity = commands
                .spawn((curve_shape, Transform::from_xyz(0.0, 0.0, 1.0)))
                .id();

            commands
                .entity(entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn spawn_selected_external_entity(
    mut commands: Commands,
    external_entity_query: Query<
        (Entity, &PickSelection, &NestingLevel),
        (
            With<ExternalEntity>,
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    fixed_system_element_geometries: Res<FixedSystemElementGeometriesByNestingLevel>,
) {
    for (selected_entity, selection, nesting_level) in &external_entity_query {
        if selection.is_selected {
            let mut geom = fixed_system_element_geometries
                .get(&**nesting_level)
                .expect("Geometries have to be created already by spawn_external_entity")
                .external_entity
                .clone();
            geom.shape.stroke = Some(Stroke::new(
                Color::WHITE,
                EXTERNAL_ENTITY_SELECTED_INNER_LINE_WIDTH,
            ));

            let helper_entity = commands
                .spawn((Transform::from_xyz(0.0, 0.0, 1.0), geom, PickParent))
                .id();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn update_selected_flow_curve(
    flow_curve_query: Query<
        (
            Entity,
            &FlowCurve,
            Option<&FlowEndpointOffset>,
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
            &SelectedHighlightHelperAdded,
            &NestingLevel,
        ),
        Or<(Changed<FlowCurve>, Changed<FlowEndpointOffset>)>,
    >,
    subsystem_query: Query<(&GlobalTransform, &crate::bevy_app::components::System)>,
    parent_query: Query<&ChildOf>,
    global_transform_query: Query<&GlobalTransform>,
    mut selected_query: Query<&mut Shape>,
    zoom: Res<Zoom>,
) {
    for (entity, flow_curve, offset, start_conn, end_conn, helper, nesting_level) in
        &flow_curve_query
    {
        let mut adjusted_curve = crate::bevy_app::systems::ui::flow::compute_adjusted_curve(
            flow_curve,
            offset,
            start_conn,
            end_conn,
            &subsystem_query,
            **zoom,
        );

        if let Some(offset) = offset {
            if let Ok(parent) = parent_query.get(entity) {
                if let Ok(parent_gt) = global_transform_query.get(parent.parent()) {
                    let parent_inv = parent_gt.affine().inverse();
                    if offset.start_angle.is_some() {
                        adjusted_curve.start = parent_inv
                            .transform_point3(adjusted_curve.start.extend(0.0))
                            .truncate();
                    }
                    if offset.end_angle.is_some() {
                        adjusted_curve.end = parent_inv
                            .transform_point3(adjusted_curve.end.extend(0.0))
                            .truncate();
                    }
                }
            }
        }

        let mut shape = selected_query
            .get_mut(helper.helper_entity)
            .expect("Helper entity should exist");
        let curve_shape = create_flow_curve_shape(
            &adjusted_curve,
            NestingLevel::compute_scale(**nesting_level, **zoom),
        );

        shape.path = curve_shape.path;
    }
}

pub fn despawn_selected_helper(
    mut commands: Commands,
    selected_query: Query<
        (Entity, &SelectedHighlightHelperAdded, &PickSelection),
        Changed<PickSelection>,
    >,
) {
    for (deselected_entity, helper, selection) in &selected_query {
        if !selection.is_selected {
            commands
                .entity(deselected_entity)
                .remove::<SelectedHighlightHelperAdded>()
                .remove_children(&[helper.helper_entity]);
            commands.entity(helper.helper_entity).despawn();
        }
    }
}
