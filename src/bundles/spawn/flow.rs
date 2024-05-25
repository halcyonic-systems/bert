use crate::bundles::{spawn_external_entity, spawn_interface};
use crate::components::*;
use crate::constants::*;
use crate::data_model::Transform2d;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::plugins::mouse_interaction::PickSelection;
use crate::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator,
};
use crate::systems::{
    create_aabb_from_flow_curve, create_path_from_flow_curve, tessellate_simplified_mesh,
};
use crate::utils::ui_transform_from_button;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rust_decimal::Decimal;

macro_rules! spawn_flow {
    (
        $fn_name:ident,
        $curve_method:tt,
        $flow_conn_ty:tt,
        $target_ty:tt
    ) => {
        pub fn $fn_name(
            commands: &mut Commands,
            subsystem_query: &Query<&Subsystem>,
            nesting_query: &Query<&NestingLevel>,
            system_entity: Entity,
            transform: &Transform,
            stroke_tess: &mut ResMut<StrokeTessellator>,
            meshes: &mut ResMut<Assets<Mesh>>,
            zoom: f32,
            is_selected: bool,
            substance_type: SubstanceType,
            usability: InteractionUsability,
            amount: Decimal,
            unit: &str,
            name: &str,
            description: &str,
        ) -> Entity {
            let (transform, initial_position) = ui_transform_from_button(transform, 0.0, 0.0, zoom);

            let direction = transform.right().truncate();

            let nesting_level = NestingLevel::current(system_entity, nesting_query);
            let scale = NestingLevel::compute_scale(nesting_level, zoom);

            let flow_curve = FlowCurve::$curve_method(zoom, initial_position, direction, scale);

            spawn_interaction(
                commands,
                subsystem_query,
                stroke_tess,
                meshes,
                flow_curve,
                system_entity,
                name,
                description,
                Flow {
                    interaction_type: InteractionType::default(),
                    substance_type,
                    substance_sub_type: "".to_string(),
                    amount,
                    unit: unit.to_string(),
                    usability,
                    parameters: vec![],
                },
                $flow_conn_ty {
                    target: system_entity,
                    target_type: $target_ty::System,
                },
                is_selected,
                nesting_level,
                scale,
            )
        }
    };
}

spawn_flow!(spawn_outflow, outflow, FlowStartConnection, StartTargetType);
spawn_flow!(spawn_inflow, inflow, FlowEndConnection, EndTargetType);

fn spawn_interaction<C: Component>(
    commands: &mut Commands,
    subsystem_query: &Query<&Subsystem>,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
    flow_curve: FlowCurve,
    system_entity: Entity,
    name: &str,
    description: &str,
    flow: Flow,
    flow_connection: C,
    is_selected: bool,
    nesting_level: u16,
    scale: f32,
) -> Entity {
    let interaction_entity = spawn_interaction_only(
        commands,
        flow,
        flow_curve,
        name,
        description,
        is_selected,
        nesting_level,
        scale,
        stroke_tess,
        meshes,
    );

    commands.entity(interaction_entity).insert(flow_connection);

    if let Ok(subsystem) = subsystem_query.get(system_entity) {
        commands
            .entity(subsystem.parent_system)
            .add_child(interaction_entity);
    }

    interaction_entity
}

pub fn spawn_interaction_only(
    commands: &mut Commands,
    flow: Flow,
    flow_curve: FlowCurve,
    name: &str,
    description: &str,
    is_selected: bool,
    nesting_level: u16,
    scale: f32,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
) -> Entity {
    let curve_path = create_path_from_flow_curve(&flow_curve, scale);

    let mut head_path_builder = PathBuilder::new();

    head_path_builder.move_to(Vec2::ZERO);
    head_path_builder.line_to(vec2(FLOW_ARROW_HEAD_LENGTH, FLOW_ARROW_HEAD_WIDTH_HALF));
    head_path_builder.line_to(vec2(FLOW_ARROW_HEAD_LENGTH, -FLOW_ARROW_HEAD_WIDTH_HALF));
    head_path_builder.close();
    let head_path = head_path_builder.build();

    let aabb = create_aabb_from_flow_curve(&flow_curve);

    let color = flow.substance_type.flow_color();

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
            PickableBundle::default(),
            PickSelection { is_selected },
            HighlightBundles {
                idle: Stroke::new(color, FLOW_LINE_WIDTH * scale),
                selected: Stroke::new(color, FLOW_SELECTED_LINE_WIDTH),
            },
            SystemElement::Interaction,
            Name::new(name.to_string()),
            ElementDescription::new(description),
            NestingLevel::new(nesting_level),
        ))
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: head_path,
                    spatial: SpatialBundle {
                        transform: Transform::from_translation(flow_curve.end.extend(2.0))
                            .with_scale(vec3(scale, scale, 1.0))
                            .with_rotation(flow_curve.head_rotation()),
                        ..default()
                    },
                    ..default()
                },
                Fill::color(color),
                ApplyZoomToScale,
                NestingLevel::new(nesting_level),
            ));
        })
        .id();
    flow_entity
}

macro_rules! spawn_complete_flow {
    ($fn_name:ident, $spawn_name:ident, $interface_ty:expr) => {
        pub fn $fn_name(
            mut commands: &mut Commands,
            focused_system: FocusedSystem,
            subsystem_query: &Query<&Subsystem>,
            nesting_query: &Query<&NestingLevel>,
            mut meshes: &mut ResMut<Assets<Mesh>>,
            mut stroke_tess: &mut ResMut<StrokeTessellator>,
            fixed_system_element_geometries: &mut ResMut<
                FixedSystemElementGeometriesByNestingLevel,
            >,
            zoom: f32,
            interface_angle: f32,
            system_radius: f32,
            substance_type: SubstanceType,
            usability: InteractionUsability,
            amount: Decimal,
            unit: &str,
            interface_name: &str,
            interface_description: &str,
            flow_name: &str,
            flow_description: &str,
            external_entity_name: &str,
            external_entity_description: &str,
            external_entity_transform: Option<&Transform2d>,
        ) -> (Entity, Entity, Entity) {
            let mut translation = vec3(system_radius * zoom, 0.0, 0.0);

            let mut transform = Transform::from_rotation(Quat::from_rotation_z(interface_angle));
            translation = transform.transform_point(translation);
            transform.translation = translation;

            let nesting_level = NestingLevel::current(*focused_system, nesting_query);

            let product_flow_entity = $spawn_name(
                &mut commands,
                subsystem_query,
                nesting_query,
                *focused_system,
                &transform,
                &mut stroke_tess,
                &mut meshes,
                zoom,
                false,
                substance_type,
                usability,
                amount,
                unit,
                flow_name,
                flow_description,
            );

            let interface_entity = spawn_interface(
                &mut commands,
                $interface_ty,
                substance_type,
                product_flow_entity,
                &transform,
                nesting_level,
                *focused_system,
                fixed_system_element_geometries,
                zoom,
                false,
                meshes,
                stroke_tess,
                interface_name,
                interface_description,
            );

            let transform = if let Some(t2d) = external_entity_transform {
                Transform::from_translation(
                    (t2d.translation * zoom).extend(transform.translation.z),
                )
                .with_rotation(Quat::from_rotation_z(t2d.rotation))
            } else {
                let right = transform.right();
                transform.translation += right * FLOW_LENGTH;
                transform
            };

            let external_entity = spawn_external_entity(
                &mut commands,
                subsystem_query,
                nesting_query,
                *focused_system,
                $interface_ty,
                substance_type,
                product_flow_entity,
                &transform,
                fixed_system_element_geometries,
                zoom,
                false,
                meshes,
                stroke_tess,
                external_entity_name,
                external_entity_description,
            );

            (interface_entity, interface_entity, external_entity)
        }
    };
}

spawn_complete_flow!(spawn_complete_outflow, spawn_outflow, InterfaceType::Export);
spawn_complete_flow!(spawn_complete_inflow, spawn_inflow, InterfaceType::Import);
