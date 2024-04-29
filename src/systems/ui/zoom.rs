use crate::bundles::get_system_geometry_from_radius;
use crate::components::InitialPosition;
use crate::resources::Zoom;
use bevy::prelude::*;
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
    mut query: Query<(&mut SimplifiedMesh, &mut Path, &crate::components::System)>,
    zoom: Res<Zoom>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if !zoom.is_changed() {
        return;
    }

    for (mut simplified_mesh, mut path, system) in &mut query {
        let radius = system.radius * **zoom;

        let (mesh, p) = get_system_geometry_from_radius(radius);

        simplified_mesh.mesh = meshes.add(mesh).into();
        *path = p;
    }
}
