use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::systems::create_paths_from_flow_curve;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_selected_interface(
    mut commands: Commands,
    interface_query: Query<
        (Entity, &PickSelection),
        (
            With<Interface>,
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    fixed_system_element_geometries: Res<FixedSystemElementGeometries>,
) {
    for (selected_entity, selection) in &interface_query {
        if selection.is_selected {
            let helper_entity = commands
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    },
                    fixed_system_element_geometries.interface.clone(),
                    Stroke::new(Color::WHITE, INTERFACE_SELECTED_INNER_LINE_WIDTH),
                ))
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
        (Entity, &FlowCurve, &PickSelection),
        (
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
) {
    for (selected_entity, flow_curve, selection) in &curve_query {
        if selection.is_selected {
            let (curve_path, _) = create_paths_from_flow_curve(&flow_curve);

            let helper_entity = commands
                .spawn((
                    ShapeBundle {
                        path: curve_path,
                        spatial: SpatialBundle {
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    Stroke::new(Color::WHITE, FLOW_SELECTED_INNER_LINE_WIDTH),
                ))
                .id();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn spawn_selected_external_entity(
    mut commands: Commands,
    external_entity_query: Query<
        (Entity, &PickSelection),
        (
            With<ExternalEntity>,
            Changed<PickSelection>,
            Without<SelectedHighlightHelperAdded>,
        ),
    >,
    fixed_system_element_geometries: Res<FixedSystemElementGeometries>,
) {
    for (selected_entity, selection) in &external_entity_query {
        if selection.is_selected {
            let helper_entity = commands
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    },
                    fixed_system_element_geometries.external_entity.clone(),
                    Stroke::new(Color::WHITE, EXTERNAL_ENTITY_SELECTED_INNER_LINE_WIDTH),
                ))
                .id();

            commands
                .entity(selected_entity)
                .add_child(helper_entity)
                .insert(SelectedHighlightHelperAdded { helper_entity });
        }
    }
}

pub fn update_selected_flow_curve(
    flow_curve_query: Query<(&FlowCurve, &SelectedHighlightHelperAdded), Changed<FlowCurve>>,
    mut selected_query: Query<&mut Path>,
) {
    for (flow_curve, helper) in &flow_curve_query {
        let mut path = selected_query
            .get_mut(helper.helper_entity)
            .expect("Helper entity should exist");
        let (curve_path, _) = create_paths_from_flow_curve(&flow_curve);

        *path = curve_path;
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
            commands.entity(helper.helper_entity).despawn_recursive();
            commands
                .entity(deselected_entity)
                .remove::<SelectedHighlightHelperAdded>();
        }
    }
}
