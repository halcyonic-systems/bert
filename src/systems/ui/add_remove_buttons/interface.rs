use crate::bundles::spawn_create_button;
use crate::components::*;
use crate::constants::INTERFACE_WIDTH_HALF;
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;

macro_rules! interface_create_button {
    ($fn_name:ident, $flow:ty, $interface_connection:ty, $terminal_connection:ty, $button_type:expr, $side:tt, $side_dir:tt) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &FlowCurve, &$flow),
                (
                    Without<$interface_connection>,
                    Without<$terminal_connection>,
                    Without<FlowInterfaceButton>,
                ),
            >,
            focused_system: Res<FocusedSystem>,
            zoom: Res<Zoom>,
            asset_server: Res<AssetServer>,
        ) {
            for (entity, flow_curve, flow) in &query {
                if flow.system != **focused_system {
                    continue;
                }

                let direction = flow_curve.start_direction;

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: $button_type,
                        connection_source: entity,
                        system: **focused_system,
                        substance_type: None,
                    },
                    flow_curve.$side - flow_curve.$side_dir.normalize() * INTERFACE_WIDTH_HALF,
                    direction.to_angle(),
                    **zoom,
                    Some(**focused_system),
                    &asset_server,
                );
            }
        }
    };
}

interface_create_button!(
    add_outflow_interface_create_button,
    Outflow,
    OutflowInterfaceConnection,
    OutflowSinkConnection,
    CreateButtonType::ExportInterface,
    start,
    start_direction
);
interface_create_button!(
    add_inflow_interface_create_button,
    Inflow,
    InflowInterfaceConnection,
    InflowSourceConnection,
    CreateButtonType::ImportInterface,
    end,
    end_direction
);
