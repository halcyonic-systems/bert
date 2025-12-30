mod external_entity;
mod inflow;
mod interface;
mod outflow;
mod subsystem;

use bevy::math::vec2;
pub use outflow::*;

use crate::bevy_app::components::*;
use bevy::prelude::*;

const INTERFACE_ANGLE_INCREMENT: f32 = std::f32::consts::PI * 0.16;

macro_rules! button_transform {
    ($fn_name:ident, $flow_system_connection:ty, $interface_connection:ty, $sign:literal) => {
        pub fn $fn_name(
            flow_interface_query: &Query<(&$flow_system_connection, &$interface_connection)>,
            transform_query: &Query<&Transform>,
            system_query: &Query<&crate::bevy_app::components::System>,
            focused_system: Entity,
            initial_position_is_occupied: bool,
        ) -> (Vec2, f32) {
            let mut existing_interface_angles = vec![];

            for (flow_system_connection, flow_interface_connection) in flow_interface_query {
                if flow_system_connection.target == focused_system {
                    let interface_pos = transform_query
                        .get(flow_interface_connection.target)
                        .expect("Interface should have a Transform")
                        .translation;

                    let mut diff = interface_pos.truncate();
                    diff.x *= $sign;

                    existing_interface_angles.push(diff.to_angle());
                }
            }

            if initial_position_is_occupied {
                existing_interface_angles.push(0.0);
                existing_interface_angles.push(INTERFACE_ANGLE_INCREMENT);
                existing_interface_angles.push(-INTERFACE_ANGLE_INCREMENT);
            }

            let angle = if existing_interface_angles.is_empty() {
                0.0
            } else {
                let mut min_angle = f32::MAX;
                let mut max_angle = -f32::MAX;

                for angle in existing_interface_angles {
                    min_angle = min_angle.min(angle);
                    max_angle = max_angle.max(angle);
                }

                if -min_angle > max_angle {
                    max_angle + INTERFACE_ANGLE_INCREMENT
                } else {
                    min_angle - INTERFACE_ANGLE_INCREMENT
                }
            };

            let angle = if $sign < 0.0 {
                if angle < 0.0 {
                    -std::f32::consts::PI - angle
                } else {
                    std::f32::consts::PI - angle
                }
            } else {
                angle
            };

            let radius = system_query
                .get(focused_system)
                .expect("Focused system should have a System")
                .radius;

            let position = Mat2::from_angle(angle) * vec2(radius, 0.0);

            (position, angle)
        }
    };
}

button_transform!(
    next_outflow_button_transform,
    FlowStartConnection,
    FlowStartInterfaceConnection,
    1.0
);
button_transform!(
    next_inflow_button_transform,
    FlowEndConnection,
    FlowEndInterfaceConnection,
    -1.0
);
