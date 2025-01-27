use crate::components::{FlowCurve, NestingLevel};
use crate::constants::{FLOW_ARROW_HEAD_LENGTH, FLOW_CLICK_TOLERANCE, FLOW_CLICK_WIDTH};
use crate::resources::{StrokeTessellator, Zoom};
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
            &FlowCurve,
            &mut Path,
            &mut SimplifiedMesh,
            &mut Aabb,
            &Children,
            &NestingLevel,
        ),
        Changed<FlowCurve>,
    >,
    mut transform_query: Query<&mut Transform, With<Path>>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut meshes: ResMut<Assets<Mesh>>,
    zoom: Res<Zoom>,
) {
    for (flow_curve, path, simplified_mesh, aabb, children, nesting_level) in &mut query {
        update_flow_curve(
            &mut transform_query,
            flow_curve,
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
