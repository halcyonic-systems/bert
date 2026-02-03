use crate::bevy_app::bundles::{spawn_external_entity, spawn_interface};
use crate::bevy_app::components::*;
use crate::bevy_app::constants::*;
use crate::bevy_app::data_model::Transform2d;
use crate::bevy_app::events::FlowEndpointHandleDrag;
use crate::bevy_app::plugins::label::{add_name_label, BackgroundArgs};
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::bevy_app::plugins::mouse_interaction::{DragPosition, PickSelection};
use crate::bevy_app::resources::{
    FixedSystemElementGeometriesByNestingLevel, FocusedSystem, StrokeTessellator, Theme,
};
use crate::bevy_app::systems::{
    create_aabb_from_flow_curve, create_flow_curve_shape, tessellate_simplified_mesh,
};
use crate::bevy_app::utils::ui_transform_from_button;
use bevy::asset::RenderAssetUsages;
use bevy::camera::primitives::Aabb;
use bevy::math::{vec2, vec3, Vec3A};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::picking::mesh_picking::ray_cast::SimplifiedMesh;
use bevy::prelude::*;
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
            system_query: &Query<(
                &Transform,
                &crate::bevy_app::components::System,
                &Name,
                &ElementDescription,
            )>,
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

            let scale = if let Ok(subsystem) = subsystem_query.get(system_entity) {
                let (_, system, _, _) = system_query
                    .get(subsystem.parent_system)
                    .expect("Parent system has to exist");

                SUBSYSTEM_SCALING_FACTOR * system.radius / MAIN_SYSTEM_RADIUS
            } else {
                1.0
            };

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
                    smart_parameters: vec![],
                },
                $flow_conn_ty {
                    target: system_entity,
                    target_type: $target_ty::System,
                },
                is_selected,
                nesting_level,
                NestingLevel::compute_scale(nesting_level, zoom),
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
    let curve_shape = create_flow_curve_shape(&flow_curve, scale);

    let head_path = ShapePath::new()
        .move_to(Vec2::ZERO)
        .line_to(vec2(FLOW_ARROW_HEAD_LENGTH, FLOW_ARROW_HEAD_WIDTH_HALF))
        .line_to(vec2(FLOW_ARROW_HEAD_LENGTH, -FLOW_ARROW_HEAD_WIDTH_HALF))
        .close();

    let aabb = create_aabb_from_flow_curve(&flow_curve);

    let color = flow.substance_type.flow_color_default();

    let flow_entity = commands
        .spawn((
            flow,
            flow_curve,
            SimplifiedMesh(tessellate_simplified_mesh(
                &curve_shape,
                meshes,
                stroke_tess,
            )),
            aabb,
            curve_shape,
            Transform::from_xyz(0.0, 0.0, FLOW_Z),
            Pickable::default(),
            PickSelection { is_selected },
            HighlightBundles {
                idle_stroke: Some(Stroke::new(color, FLOW_LINE_WIDTH * scale)),
                selected_stroke: Some(Stroke::new(color, FLOW_SELECTED_LINE_WIDTH)),
                idle_fill: None,
                selected_fill: None,
            },
            SystemElement::Interaction,
            Name::new(name.to_string()),
            ElementDescription::new(description),
            NestingLevel::new(nesting_level),
        ))
        .with_children(|parent| {
            let head_shape = ShapeBuilder::with(&head_path).fill(color).build();

            parent.spawn((
                head_shape,
                Transform::from_translation(flow_curve.end.extend(2.0))
                    .with_scale(vec3(scale, scale, 1.0))
                    .with_rotation(flow_curve.head_rotation()),
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
            system_query: &Query<(
                &Transform,
                &crate::bevy_app::components::System,
                &Name,
                &ElementDescription,
            )>,
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
                system_query,
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
                false,
            );

            (interface_entity, interface_entity, external_entity)
        }
    };
}

spawn_complete_flow!(spawn_complete_outflow, spawn_outflow, InterfaceType::Export);
spawn_complete_flow!(spawn_complete_inflow, spawn_inflow, InterfaceType::Import);

pub fn auto_spawn_flow_label(
    mut commands: Commands,
    flow_query: Query<(Entity, &NestingLevel), Added<Flow>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
    theme: Res<Theme>,
) {
    for (flow_entity, nesting_level) in flow_query.iter() {
        let color = match *theme {
            Theme::Normal => {
                if **nesting_level == 0 {
                    CLEAR_COLOR
                } else {
                    Color::WHITE
                }
            }
            Theme::White => Color::srgb(0.95, 0.95, 0.95),
        };

        add_name_label(
            &mut commands,
            flow_entity,
            vec2(50.0, 45.0),
            Some(BackgroundArgs { color, ..default() }),
            None,
            false,
            &name_query,
            &asset_server,
            None,
            *nesting_level,
        );
    }
}

/// Auto-spawn draggable handle entities at flow endpoints.
/// Only spawns for internal flows (subsystem-to-subsystem connections).
/// Checks for existing handles rather than using a marker to avoid stale state issues.
pub fn auto_spawn_flow_endpoint_handles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    flow_query: Query<
        (
            Entity,
            &FlowCurve,
            &NestingLevel,
            Option<&FlowEndpointOffset>,
            &FlowStartConnection,
            &FlowEndConnection,
        ),
        (
            With<Flow>,
            Without<FlowStartInterfaceConnection>,
            Without<FlowEndInterfaceConnection>,
        ),
    >,
    existing_handles: Query<&FlowEndpointHandle>,
    parent_query: Query<&ChildOf>,
    global_transform_query: Query<&GlobalTransform>,
    subsystem_query: Query<(&GlobalTransform, &crate::bevy_app::components::System)>,
) {
    let flows_with_handles: std::collections::HashSet<Entity> =
        existing_handles.iter().map(|h| h.flow).collect();

    for (flow_entity, flow_curve, nesting_level, existing_offset, start_conn, end_conn) in
        flow_query.iter()
    {
        if flows_with_handles.contains(&flow_entity) {
            continue;
        }

        let is_e_network_feedback = start_conn.target_type == StartTargetType::Sink
            && end_conn.target_type == EndTargetType::Source;
        let is_e_network_feedforward = start_conn.target_type == StartTargetType::Source
            && end_conn.target_type == EndTargetType::Sink;
        if is_e_network_feedback || is_e_network_feedforward {
            continue;
        }

        let is_internal_flow = start_conn.target_type == StartTargetType::System
            && end_conn.target_type == EndTargetType::System;
        if !is_internal_flow {
            continue;
        }
        info!(
            "Spawning endpoint handles for flow {:?} at start={:?}, end={:?}",
            flow_entity, flow_curve.start, flow_curve.end
        );

        if existing_offset.is_none() {
            let mut initial_offset = FlowEndpointOffset::default();

            let (flow_world_start, flow_world_end) =
                if let Ok(parent) = parent_query.get(flow_entity) {
                    if let Ok(parent_gt) = global_transform_query.get(parent.parent()) {
                        (
                            parent_gt
                                .transform_point(flow_curve.start.extend(0.0))
                                .truncate(),
                            parent_gt
                                .transform_point(flow_curve.end.extend(0.0))
                                .truncate(),
                        )
                    } else {
                        (flow_curve.start, flow_curve.end)
                    }
                } else {
                    (flow_curve.start, flow_curve.end)
                };

            if let Ok((subsys_gt, _)) = subsystem_query.get(start_conn.target) {
                let center = subsys_gt.translation().truncate();
                let to_start = flow_world_start - center;
                if to_start.length() > 0.001 {
                    initial_offset.start_angle = Some(to_start.to_angle());
                }
            }

            if let Ok((subsys_gt, _)) = subsystem_query.get(end_conn.target) {
                let center = subsys_gt.translation().truncate();
                let to_end = flow_world_end - center;
                if to_end.length() > 0.001 {
                    initial_offset.end_angle = Some(to_end.to_angle());
                }
            }

            commands.entity(flow_entity).insert(initial_offset);
        }

        let scale = NestingLevel::compute_scale(**nesting_level, 1.0);
        let handle_radius = FLOW_ENDPOINT_HANDLE_RADIUS * scale;

        let circle_shape = shapes::Circle {
            radius: handle_radius,
            center: Vec2::ZERO,
        };
        let circle_lyon_shape = ShapeBuilder::with(&circle_shape)
            .fill(Color::srgb(1.0, 0.0, 0.0))
            .build();

        let simplified_mesh = SimplifiedMesh(build_circle_picking_mesh(&mut meshes, handle_radius));

        let handle_aabb = Aabb {
            center: Vec3A::ZERO,
            half_extents: Vec3A::new(handle_radius, handle_radius, 0.0),
        };

        commands
            .spawn((
                FlowEndpointHandle {
                    flow: flow_entity,
                    endpoint: FlowEndpoint::Start,
                },
                circle_lyon_shape.clone(),
                Transform::from_translation(flow_curve.start.extend(FLOW_ENDPOINT_HANDLE_Z)),
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
                PickSelection::default(),
                simplified_mesh.clone(),
                handle_aabb,
                ApplyZoomToScale,
                NestingLevel::new(**nesting_level),
            ))
            .observe(
                |on: On<DragPosition>, mut writer: MessageWriter<FlowEndpointHandleDrag>| {
                    info!(
                        "START handle received DragPosition: entity={:?}, pos={:?}",
                        on.event().target,
                        on.world_position
                    );
                    writer.write(FlowEndpointHandleDrag::from_on(&on));
                },
            );

        commands
            .spawn((
                FlowEndpointHandle {
                    flow: flow_entity,
                    endpoint: FlowEndpoint::End,
                },
                circle_lyon_shape,
                Transform::from_translation(flow_curve.end.extend(FLOW_ENDPOINT_HANDLE_Z)),
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                },
                PickSelection::default(),
                simplified_mesh,
                handle_aabb,
                ApplyZoomToScale,
                NestingLevel::new(**nesting_level),
            ))
            .observe(
                |on: On<DragPosition>, mut writer: MessageWriter<FlowEndpointHandleDrag>| {
                    info!(
                        "END handle received DragPosition: entity={:?}, pos={:?}",
                        on.event().target,
                        on.world_position
                    );
                    writer.write(FlowEndpointHandleDrag::from_on(&on));
                },
            );
    }
}

/// Build a tessellated circle mesh for raycast picking.
fn build_circle_picking_mesh(meshes: &mut ResMut<Assets<Mesh>>, radius: f32) -> Handle<Mesh> {
    const SEGMENTS: usize = 32;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(SEGMENTS + 2);
    let mut indices: Vec<u32> = Vec::with_capacity(SEGMENTS * 3);

    positions.push([0.0, 0.0, 0.0]);

    for i in 0..=SEGMENTS {
        let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
        positions.push([radius * angle.cos(), radius * angle.sin(), 0.0]);
    }

    for i in 0..SEGMENTS {
        indices.push(0);
        indices.push((i + 1) as u32);
        indices.push((i + 2) as u32);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_indices(Indices::U32(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    meshes.add(mesh)
}

/// Update handle positions to follow flow curve endpoints.
pub fn update_flow_endpoint_handle_positions(
    flow_query: Query<(
        &FlowCurve,
        Option<&FlowEndpointOffset>,
        Option<&FlowStartConnection>,
        Option<&FlowEndConnection>,
    )>,
    subsystem_query: Query<(&GlobalTransform, &crate::bevy_app::components::System)>,
    mut handle_query: Query<(&FlowEndpointHandle, &mut Transform)>,
    zoom: Res<crate::bevy_app::resources::Zoom>,
) {
    for (handle, mut transform) in handle_query.iter_mut() {
        let Ok((flow_curve, offset, start_conn, end_conn)) = flow_query.get(handle.flow) else {
            continue;
        };

        let target_pos = match handle.endpoint {
            FlowEndpoint::Start => {
                if let Some(Some(angle)) = offset.map(|o| o.start_angle) {
                    if let Some(conn) = start_conn {
                        if let Ok((subsys_transform, system)) = subsystem_query.get(conn.target) {
                            let center = subsys_transform.translation().truncate();
                            let radius = system.radius * **zoom;
                            center + Vec2::from_angle(angle) * radius
                        } else {
                            flow_curve.start
                        }
                    } else {
                        flow_curve.start
                    }
                } else {
                    flow_curve.start
                }
            }
            FlowEndpoint::End => {
                if let Some(Some(angle)) = offset.map(|o| o.end_angle) {
                    if let Some(conn) = end_conn {
                        if let Ok((subsys_transform, system)) = subsystem_query.get(conn.target) {
                            let center = subsys_transform.translation().truncate();
                            let radius = system.radius * **zoom;
                            center + Vec2::from_angle(angle) * radius
                        } else {
                            flow_curve.end
                        }
                    } else {
                        flow_curve.end
                    }
                } else {
                    flow_curve.end
                }
            }
        };

        transform.translation.x = target_pos.x;
        transform.translation.y = target_pos.y;
    }
}
