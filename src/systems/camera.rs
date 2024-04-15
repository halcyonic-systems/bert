use bevy::prelude::*;

pub fn zoom_control_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection>,
) {
    let mut projection = camera_query.single_mut();

    if input.pressed(KeyCode::Minus) {
        info!("minus");
        projection.scale += 0.2;
    }

    if input.pressed(KeyCode::Equal) {
        projection.scale -= 0.2;
    }

    projection.scale = projection.scale.clamp(0.2, 5.);
}
