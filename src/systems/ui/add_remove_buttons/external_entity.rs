use crate::bundles::spawn_create_button;
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;
use crate::constants::BUTTON_WIDTH_HALF;

macro_rules! external_entity_create_button {
    ($fn_name:ident, $flow:ty, $interface_connection:ty, $terminal_connection:ty, $button_type:expr) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &FlowCurve, &$flow),
                (
                    With<$interface_connection>,
                    Without<$terminal_connection>,
                    Without<FlowOtherEndButton>,
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

                let direction = -flow_curve.end_direction.normalize();

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: $button_type,
                        connection_source: entity,
                        system: **focused_system,
                    },
                    flow_curve.end + direction * BUTTON_WIDTH_HALF,
                    direction.to_angle(),
                    **zoom,
                    &asset_server,
                );
            }
        }
    };
}

external_entity_create_button!(
    add_source_create_button,
    Inflow,
    InflowInterfaceConnection,
    InflowSourceConnection,
    CreateButtonType::Source
);
external_entity_create_button!(
    add_sink_create_button,
    Outflow,
    OutflowInterfaceConnection,
    OutflowSinkConnection,
    CreateButtonType::Sink
);
