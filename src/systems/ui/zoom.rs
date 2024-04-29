use crate::bundles::{aabb_from_radius, get_system_geometry_from_radius};
use crate::components::InitialPosition;
use crate::resources::Zoom;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_mod_picking::backends::raycast::bevy_mod_raycast::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn apply_zoom(
    mut query: Query<(&mut Transform, &InitialPosition), Without<Camera>>,
    zoom: Res<Zoom>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (mut transform, initial_position) in &mut query {
        transform.translation = (**initial_position * **zoom).extend(transform.translation.z);
    }
}

pub fn apply_zoom_to_system_radii(
    mut query: Query<(
        &mut SimplifiedMesh,
        &mut Path,
        &mut Aabb,
        &crate::components::System,
    )>,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (mut simplified_mesh, mut path, mut aabb, system) in &mut query {
        let zoomed_radius = system.radius * **zoom;

        let (mesh, p) = get_system_geometry_from_radius(zoomed_radius);

        simplified_mesh.mesh = meshes.add(mesh).into();
        *path = p;

        *aabb = aabb_from_radius(zoomed_radius);
    }
}
