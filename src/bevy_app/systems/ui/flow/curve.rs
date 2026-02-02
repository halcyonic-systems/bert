use crate::bevy_app::components::{
    FlowCurve, FlowEndConnection, FlowEndpointOffset, FlowStartConnection, NestingLevel,
};
use crate::bevy_app::constants::{FLOW_ARROW_HEAD_LENGTH, FLOW_CLICK_TOLERANCE, FLOW_CLICK_WIDTH};
use crate::bevy_app::resources::{StrokeTessellator, Zoom};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssetUsages;
use bevy_picking::mesh_picking::ray_cast::SimplifiedMesh;
use bevy_prototype_lyon::prelude::tess::{
    BuffersBuilder, StrokeVertex, StrokeVertexConstructor, VertexBuffers,
};
use bevy_prototype_lyon::prelude::*;

pub fn draw_flow_curve(
    mut query: Query<
        (
            Entity,
            &FlowCurve,
            Option<&FlowEndpointOffset>,
            Option<&FlowStartConnection>,
            Option<&FlowEndConnection>,
            &mut Path,
            &mut SimplifiedMesh,
            &mut Aabb,
            &Children,
            &NestingLevel,
        ),
        Or<(Changed<FlowCurve>, Changed<FlowEndpointOffset>)>,
    >,
    subsystem_query: Query<(&GlobalTransform, &crate::bevy_app::components::System)>,
    parent_query: Query<&Parent>,
    global_transform_query: Query<&GlobalTransform>,
    mut transform_query: Query<&mut Transform, With<Path>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut meshes: ResMut<Assets<Mesh>>,
    zoom: Res<Zoom>,
) {
    for (
        entity,
        flow_curve,
        offset,
        start_conn,
        end_conn,
        path,
        simplified_mesh,
        aabb,
        children,
        nesting_level,
    ) in &mut query
    {
        // Compute adjusted start/end positions based on angle offsets
        let mut adjusted_curve = compute_adjusted_curve(
            flow_curve,
            offset,
            start_conn,
            end_conn,
            &subsystem_query,
            **zoom,
        );

        // compute_adjusted_curve produces world-space positions when angles are set.
        // The flow path renders in parent-local space, so convert back.
        // Same pattern as connection_mode.rs:585-591.
        if let Some(offset) = offset {
            if let Ok(parent) = parent_query.get(entity) {
                if let Ok(parent_gt) = global_transform_query.get(parent.get()) {
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

        update_flow_curve(
            &mut transform_query,
            &adjusted_curve,
            path,
            simplified_mesh,
            aabb,
            children,
            **nesting_level,
            **zoom,
            &mut stroke_tess,
            &mut meshes,
        );
    }
}

/// Compute flow curve with angle-based offsets applied.
/// If an angle is set, position = subsystem_center + Vec2::from_angle(angle) * radius
fn compute_adjusted_curve(
    flow_curve: &FlowCurve,
    offset: Option<&FlowEndpointOffset>,
    start_conn: Option<&FlowStartConnection>,
    end_conn: Option<&FlowEndConnection>,
    subsystem_query: &Query<(&GlobalTransform, &crate::bevy_app::components::System)>,
    zoom: f32,
) -> FlowCurve {
    let Some(offset) = offset else {
        return flow_curve.clone();
    };

    let mut adjusted = flow_curve.clone();

    // Compute adjusted start position from angle
    if let Some(start_angle) = offset.start_angle {
        if let Some(start_conn) = start_conn {
            if let Ok((transform, system)) = subsystem_query.get(start_conn.target) {
                let center = transform.translation().truncate();
                let radius = system.radius * zoom;
                adjusted.start = center + Vec2::from_angle(start_angle) * radius;
                // Update direction to point outward from center
                adjusted.start_direction = Vec2::from_angle(start_angle);
            }
        }
    }

    // Compute adjusted end position from angle
    if let Some(end_angle) = offset.end_angle {
        if let Some(end_conn) = end_conn {
            if let Ok((transform, system)) = subsystem_query.get(end_conn.target) {
                let center = transform.translation().truncate();
                let radius = system.radius * zoom;
                adjusted.end = center + Vec2::from_angle(end_angle) * radius;
                // Update direction to point outward from center (into subsystem)
                adjusted.end_direction = Vec2::from_angle(end_angle);
            }
        }
    }

    adjusted
}

pub fn update_flow_curve(
    transform_query: &mut Query<&mut Transform, With<Path>>,
    flow_curve: &FlowCurve,
    mut path: Mut<Path>,
    mut simplified_mesh: Mut<SimplifiedMesh>,
    mut aabb: Mut<Aabb>,
    children: &Children,
    nesting_level: u16,
    zoom: f32,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let scale = NestingLevel::compute_scale(nesting_level, zoom);

    let curve_path = create_path_from_flow_curve(flow_curve, scale);

    let simplified_curve = flow_curve.skip_start();
    let simplified_curve_path = create_path_from_flow_curve(&simplified_curve, scale);

    simplified_mesh.0 = tessellate_simplified_mesh(&simplified_curve_path, meshes, stroke_tess);
    *aabb = create_aabb_from_flow_curve(&simplified_curve);

    *path = curve_path;

    if let Some(child) = children.iter().next() {
        if let Ok(mut transform) = transform_query.get_mut(*child) {
            transform.rotation = flow_curve.head_rotation();
            transform.translation = flow_curve.end.extend(transform.translation.z);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vertex {
    pub position: [f32; 2],
}

struct VertexConstructor;

impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            position: vertex.position().to_array(),
        }
    }
}

pub fn tessellate_simplified_mesh(
    curve_path: &Path,
    meshes: &mut ResMut<Assets<Mesh>>,
    stroke_tess: &mut ResMut<StrokeTessellator>,
) -> Handle<Mesh> {
    let mut buffers = VertexBuffers::new();

    if let Err(err) = stroke_tess.tessellate_path(
        &curve_path.0,
        &StrokeOptions::default()
            .with_line_width(FLOW_CLICK_WIDTH)
            .with_tolerance(FLOW_CLICK_TOLERANCE),
        &mut BuffersBuilder::new(&mut buffers, VertexConstructor),
    ) {
        error!("Failed to tessellate flow curve: {}", err);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_indices(Indices::U32(buffers.indices));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        buffers
            .vertices
            .iter()
            .map(|v| [v.position[0], v.position[1], 0.0])
            .collect::<Vec<_>>(),
    );

    meshes.add(mesh)
}

pub fn create_path_from_flow_curve(flow_curve: &FlowCurve, scale: f32) -> Path {
    let mut curve_path_builder = PathBuilder::new();

    let start = flow_curve.start;
    let end = flow_curve.end;

    curve_path_builder.move_to(start);

    let end_direction = flow_curve.end_direction;
    let end = end + end_direction * (FLOW_ARROW_HEAD_LENGTH - 2.0) * scale;

    let tangent_len = flow_curve.compute_tangent_length();

    curve_path_builder.cubic_bezier_to(
        start + flow_curve.start_direction * tangent_len,
        end + flow_curve.end_direction * tangent_len,
        end,
    );

    curve_path_builder.build()
}

pub fn create_aabb_from_flow_curve(flow_curve: &FlowCurve) -> Aabb {
    let tangent_length = flow_curve.compute_tangent_length();

    let mut aabb = Aabb::enclosing([
        (flow_curve.start).extend(0.0),
        (flow_curve.start + flow_curve.start_direction * tangent_length).extend(0.0),
        (flow_curve.end + flow_curve.end_direction * tangent_length).extend(0.0),
        (flow_curve.end).extend(0.0),
    ])
    .expect("Iterator is not empty so there has to be an Aabb");

    aabb.half_extents.x += FLOW_CLICK_WIDTH;
    aabb.half_extents.y += FLOW_CLICK_WIDTH;

    aabb
}
