mod external_entity;
mod inflow;
mod interface;
mod outflow;
mod subsystem;

use bevy::math::vec2;
pub use external_entity::*;
pub use inflow::*;
pub use interface::*;
pub use outflow::*;
pub use subsystem::*;

use crate::components::{
    Inflow, InflowInterfaceConnection, Outflow, OutflowInterfaceConnection, System,
};
use bevy::prelude::*;
use bevy::utils::HashSet;
use num_traits::FloatConst;

macro_rules! button_transform {
    ($fn_name:ident, $flow:ty, $interface_connection:ty, $min_max:ident, $op:tt, $start_value:expr) => {
        pub fn $fn_name(
            flow_interface_query: &Query<(&$flow, &$interface_connection)>,
            transform_query: &Query<&GlobalTransform>,
            system_query: &Query<&System>,
            focused_system: Entity,
        ) -> (Vec2, f32) {
            let system_center = transform_query
                .get(focused_system)
                .expect("System should have a Transform")
                .translation();

            let mut existing_interfaces = HashSet::new();

            for (outflow, flow_interface_connection) in flow_interface_query {
                if outflow.system == focused_system {
                    existing_interfaces.insert(flow_interface_connection.target);
                }
            }

            let mut angle = $start_value;

            for interface in existing_interfaces {
                let interface_pos = transform_query
                    .get(interface)
                    .expect("Interface should have a Transform")
                    .translation();

                let a = (interface_pos - system_center).truncate().to_angle();

                angle = angle.$min_max(a);
            }

            angle $op f32::FRAC_PI_8();

            let radius = system_query
                .get(focused_system)
                .expect("Focused system should have a System")
                .radius;

            let position = Mat2::from_angle(angle) * vec2(system_center.x + radius, system_center.y);

            (position, angle)
        }
    }
}

button_transform!(next_outflow_button_transform, Outflow, OutflowInterfaceConnection, min, -=, f32::PI());
button_transform!(next_inflow_button_transform, Inflow, InflowInterfaceConnection, max, +=, -f32::PI());
