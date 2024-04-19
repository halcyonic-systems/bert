use crate::components::*;
use crate::constants::*;
use crate::resources::StrokeTessellator;
use crate::systems::{
    create_aabb_from_flow_curve, create_paths_from_flow_curve, tessellate_simplified_mesh,
};
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_outflow(
    commands: &mut Commands,
    system_entity: Entity,
    transform: &GlobalTransform,
    initial_position: &InitialPosition,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
    zoom: f32,
) {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 6.0, 0.0, zoom);

    let direction = transform.right().truncate();

    let flow_curve = FlowCurve {
        start: *initial_position,
        start_direction: direction * FLOW_END_LENGTH,
        end: *initial_position + direction * FLOW_LENGTH,
        end_direction: direction * -FLOW_END_LENGTH,
    };

    let (curve_path, head_path) = create_paths_from_flow_curve(&flow_curve, zoom);
    let aabb = create_aabb_from_flow_curve(&flow_curve, zoom);

    commands
        .spawn((
            Outflow {
                system: system_entity,
                substance_type: Default::default(),
                usability: Default::default(),
            },
            flow_curve,
            SimplifiedMesh {
                mesh: tessellate_simplified_mesh(&curve_path, meshes, stroke_tess),
            },
            aabb,
            ShapeBundle {
                path: curve_path,
                ..default()
            },
            Stroke::new(Color::BLACK, FLOW_LINE_WIDTH),
            Fill::color(Color::WHITE),
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::Outflow,
            Name::new("Outflow"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: head_path,
                    ..default()
                },
                Fill::color(Color::BLACK),
            ));
        });
}

pub fn spawn_inflow(
    commands: &mut Commands,
    system_entity: Entity,
    transform: &GlobalTransform,
    initial_position: &InitialPosition,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    zoom: f32,
) {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 6.0, 64.0, zoom);

    commands.spawn((
        Inflow {
            system: system_entity,
            substance_type: Default::default(),
            usability: Default::default(),
        },
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(32.0, 32.0)).into(),
            transform,
            material: materials.add(ColorMaterial::from(Color::RED)),
            ..default()
        },
        PickableBundle {
            selection: PickSelection { is_selected: true },
            ..default()
        },
        SystemElement::Inflow,
        Name::new("Inflow"),
        initial_position,
    ));
}
