use crate::bundles::{spawn_external_entity, spawn_interface};
use crate::components::*;
use crate::constants::*;
use crate::plugins::lyon_selection::{HighlightBundles, SelectedSpawnListener, SpawnOnSelected};
use crate::resources::{FixedSystemElementGeometries, FocusedSystem, StrokeTessellator, Zoom};
use crate::systems::{
    create_aabb_from_flow_curve, create_paths_from_flow_curve, tessellate_simplified_mesh,
};
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_outflow(
    commands: &mut Commands,
    system_entity: Entity,
    transform: &Transform,
    initial_position: &InitialPosition,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    is_selected: bool,
    substance_type: SubstanceType,
    usability: OutflowUsability,
) -> Entity {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 6.0, 0.0, zoom);

    let direction = transform.right().truncate();

    let flow_curve = FlowCurve {
        start: *initial_position + direction * INTERFACE_WIDTH_HALF,
        start_direction: direction * FLOW_END_LENGTH,
        end: *initial_position + direction * FLOW_LENGTH,
        end_direction: direction * -FLOW_END_LENGTH,
    };

    spawn_flow(
        commands,
        stroke_tess,
        meshes,
        zoom,
        flow_curve,
        SystemElement::Outflow,
        "Outflow",
        Outflow {
            system: system_entity,
            substance_type,
            usability,
        },
        is_selected,
    )
}

fn spawn_selected_flow(
    mut commands: Commands,
    mut listener: SelectedSpawnListener,
    curve_query: Query<(&FlowCurve, &Transform)>,
    zoom: Res<Zoom>,
) {
    let (flow_curve, transform) = curve_query
        .get(listener.selected())
        .expect("Selected entity should have a FlowCurve");

    let (curve_path, _) = create_paths_from_flow_curve(&flow_curve, **zoom);

    listener.add_spawned(
        commands
            .spawn((
                ShapeBundle {
                    path: curve_path,
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, transform.translation.z + 1.0),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(Color::WHITE, FLOW_SELECTED_INNER_LINE_WIDTH),
            ))
            .id(),
    );
}

pub fn spawn_inflow(
    commands: &mut Commands,
    system_entity: Entity,
    transform: &Transform,
    initial_position: &InitialPosition,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    is_selected: bool,
    substance_type: SubstanceType,
    usability: InflowUsability,
) -> Entity {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 6.0, 0.0, zoom);

    let direction = transform.right().truncate();

    let flow_curve = FlowCurve {
        start: *initial_position + direction * FLOW_LENGTH,
        start_direction: direction * -FLOW_END_LENGTH,
        end: *initial_position + direction * INTERFACE_WIDTH_HALF,
        end_direction: direction * FLOW_END_LENGTH,
    };

    spawn_flow(
        commands,
        stroke_tess,
        meshes,
        zoom,
        flow_curve,
        SystemElement::Inflow,
        "Inflow",
        Inflow {
            system: system_entity,
            substance_type,
            usability,
        },
        is_selected,
    )
}

fn spawn_flow<F: Bundle>(
    commands: &mut Commands,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
    flow_curve: FlowCurve,
    system_element: SystemElement,
    name: &'static str,
    flow: F,
    is_selected: bool,
) -> Entity {
    let (curve_path, head_path) = create_paths_from_flow_curve(&flow_curve, zoom);
    let aabb = create_aabb_from_flow_curve(&flow_curve, zoom);

    commands
        .spawn((
            flow,
            flow_curve,
            SimplifiedMesh {
                mesh: tessellate_simplified_mesh(&curve_path, meshes, stroke_tess),
            },
            aabb,
            ShapeBundle {
                path: curve_path,
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected },
                ..default()
            },
            SpawnOnSelected::new(spawn_selected_flow),
            HighlightBundles {
                idle: Stroke::new(Color::BLACK, FLOW_LINE_WIDTH),
                selected: Stroke::new(Color::BLACK, FLOW_SELECTED_LINE_WIDTH),
            },
            system_element,
            Name::new(name),
            ElementDescription::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: head_path,
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 2.0),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(Color::BLACK),
            ));
        })
        .id()
}

macro_rules! spawn_complete_flow {
    ($fn_name:ident, $spawn_name:ident, $interface_ty:expr, $usability_ty:ty) => {
        pub fn $fn_name(
            mut commands: &mut Commands,
            focused_system: &Res<FocusedSystem>,
            mut meshes: &mut ResMut<Assets<Mesh>>,
            mut stroke_tess: &mut ResMut<StrokeTessellator>,
            fixed_system_element_geometries: &Res<FixedSystemElementGeometries>,
            zoom: f32,
            outflow_start_position: Vec2,
            substance_type: SubstanceType,
            usability: $usability_ty,
        ) -> Entity {
            let mut transform = Transform::from_translation(outflow_start_position.extend(0.0))
                .with_rotation(Quat::from_rotation_z(outflow_start_position.to_angle()));
            let mut initial_position = InitialPosition::new(outflow_start_position);

            let product_flow = $spawn_name(
                &mut commands,
                ***focused_system,
                &transform,
                &initial_position,
                &mut stroke_tess,
                &mut meshes,
                zoom,
                false,
                substance_type,
                usability,
            );

            let product_flow_interface = spawn_interface(
                &mut commands,
                $interface_ty,
                product_flow,
                &transform,
                &initial_position,
                &focused_system,
                &fixed_system_element_geometries,
                zoom,
                false,
            );

            let right = transform.right();
            transform.translation += right * FLOW_LENGTH;
            *initial_position += right.truncate() * FLOW_LENGTH;

            spawn_external_entity(
                &mut commands,
                $interface_ty,
                product_flow,
                &transform,
                &initial_position,
                &fixed_system_element_geometries,
                zoom,
                false,
            );

            product_flow_interface
        }
    };
}

spawn_complete_flow!(
    spawn_complete_outflow,
    spawn_outflow,
    InterfaceType::Export,
    OutflowUsability
);
spawn_complete_flow!(
    spawn_complete_inflow,
    spawn_inflow,
    InterfaceType::Import,
    InflowUsability
);
