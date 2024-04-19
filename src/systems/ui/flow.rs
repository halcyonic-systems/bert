use crate::components::FlowCurve;
use crate::constants::{
    FLOW_ARROW_HEAD_LENGTH, FLOW_ARROW_HEAD_WIDTH_HALF, FLOW_CLICK_TOLERANCE, FLOW_CLICK_WIDTH,
};
use crate::resources::{StrokeTessellator, Zoom};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssetUsages;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
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
        ),
        Changed<FlowCurve>,
    >,
    mut path_query: Query<&mut Path, Without<FlowCurve>>,
    zoom: Res<Zoom>,
    mut stroke_tess: ResMut<StrokeTessellator>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (flow_curve, path, simplified_mesh, aabb, children) in &mut query {
        update_flow_curve(
            &mut path_query,
            flow_curve,
            path,
            simplified_mesh,
            aabb,
            children,
            **zoom,
            &mut stroke_tess,
            &mut meshes,
        );
    }
}

pub fn update_flow_curve(
    path_query: &mut Query<&mut Path, Without<FlowCurve>>,
    flow_curve: &FlowCurve,
    mut path: Mut<Path>,
    mut simplified_mesh: Mut<SimplifiedMesh>,
    mut aabb: Mut<Aabb>,
    children: &Children,
    zoom: f32,
    stroke_tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
) {
    let (curve_path, head_path) = create_paths_from_flow_curve(flow_curve, zoom);

    simplified_mesh.mesh = tessellate_simplified_mesh(&curve_path, meshes, stroke_tess);
    *aabb = create_aabb_from_flow_curve(flow_curve, zoom);

    *path = curve_path;

    if let Some(child) = children.iter().next() {
        if let Ok(mut path) = path_query.get_mut(*child) {
            *path = head_path;
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

pub fn create_paths_from_flow_curve(flow_curve: &FlowCurve, zoom: f32) -> (Path, Path) {
    let mut curve_path_builder = PathBuilder::new();

    let zoomed_start = flow_curve.start * zoom;
    let zoomed_end = flow_curve.end * zoom;

    curve_path_builder.move_to(zoomed_start);

    let end_direction = flow_curve.end_direction.normalize();
    let end = zoomed_end + end_direction * (FLOW_ARROW_HEAD_LENGTH - 2.0);

    curve_path_builder.cubic_bezier_to(
        zoomed_start + flow_curve.start_direction * zoom,
        end + flow_curve.end_direction * zoom,
        end,
    );

    let mut head_path_builder = PathBuilder::new();

    let head_width_direction = vec2(end_direction.y, -end_direction.x);

    head_path_builder.move_to(zoomed_end);
    head_path_builder.line_to(
        zoomed_end
            + end_direction * FLOW_ARROW_HEAD_LENGTH
            + head_width_direction * FLOW_ARROW_HEAD_WIDTH_HALF,
    );
    head_path_builder.line_to(
        zoomed_end + end_direction * FLOW_ARROW_HEAD_LENGTH
            - head_width_direction * FLOW_ARROW_HEAD_WIDTH_HALF,
    );
    head_path_builder.close();

    (curve_path_builder.build(), head_path_builder.build())
}

pub fn create_aabb_from_flow_curve(flow_curve: &FlowCurve, zoom: f32) -> Aabb {
    let mut aabb = Aabb::enclosing(&[
        (flow_curve.start * zoom).extend(0.0),
        (flow_curve.start + flow_curve.start_direction * zoom).extend(0.0),
        (flow_curve.end + flow_curve.end_direction * zoom).extend(0.0),
        (flow_curve.end * zoom).extend(0.0),
    ])
    .expect("Iterator is not empty so there has to be an Aabb");

    aabb.half_extents.x += FLOW_CLICK_WIDTH;
    aabb.half_extents.y += FLOW_CLICK_WIDTH;

    aabb
}
