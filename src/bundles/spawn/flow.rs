use crate::bundles::{spawn_external_entity, spawn_interface};
use crate::components::*;
use crate::constants::*;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::resources::{FixedSystemElementGeometries, FocusedSystem, StrokeTessellator};
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
    subsystem_query: &Query<&Subsystem>,
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
        subsystem_query,
        stroke_tess,
        meshes,
        flow_curve,
        SystemElement::Outflow,
        system_entity,
        "Outflow",
        Outflow {
            system: system_entity,
            substance_type,
            usability,
        },
        is_selected,
    )
}

pub fn spawn_inflow(
    commands: &mut Commands,
    subsystem_query: &Query<&Subsystem>,
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
        subsystem_query,
        stroke_tess,
        meshes,
        flow_curve,
        SystemElement::Inflow,
        system_entity,
        "Inflow",
        Inflow {
            system: system_entity,
            substance_type,
            usability,
        },
        is_selected,
    )
}

fn spawn_flow<F: Bundle + HasSubstanceType>(
    commands: &mut Commands,
    subsystem_query: &Query<&Subsystem>,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
    flow_curve: FlowCurve,
    system_element: SystemElement,
    system_entity: Entity,
    name: &'static str,
    flow: F,
    is_selected: bool,
) -> Entity {
    let (curve_path, head_path) = create_paths_from_flow_curve(&flow_curve);
    let aabb = create_aabb_from_flow_curve(&flow_curve);

    let color = flow.substance_type().flow_color();

    let flow_entity = commands
        .spawn((
            flow,
            flow_curve,
            SimplifiedMesh {
                mesh: tessellate_simplified_mesh(&curve_path, meshes, stroke_tess),
            },
            aabb,
            ShapeBundle {
                path: curve_path,
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, FLOW_Z),
                    ..default()
                },
                ..default()
            },
            PickableBundle {
                selection: PickSelection { is_selected },
                ..default()
            },
            HighlightBundles {
                idle: Stroke::new(color, FLOW_LINE_WIDTH),
                selected: Stroke::new(color, FLOW_SELECTED_LINE_WIDTH),
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
                Fill::color(color),
            ));
        })
        .id();

    if let Ok(subsystem) = subsystem_query.get(system_entity) {
        commands
            .entity(subsystem.parent_system)
            .add_child(flow_entity);
    }

    flow_entity
}

macro_rules! spawn_complete_flow {
    ($fn_name:ident, $spawn_name:ident, $interface_ty:expr, $usability_ty:ty) => {
        pub fn $fn_name(
            mut commands: &mut Commands,
            focused_system: &Res<FocusedSystem>,
            subsystem_query: &Query<&Subsystem>,
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
                subsystem_query,
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
                subsystem_query,
                focused_system,
                $interface_ty,
                substance_type,
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
