use crate::bundles::spawn_create_button;
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;

macro_rules! external_entity_create_button {
    ($fn_name:ident, $flow:ty, $interface_connection:ty, $terminal_connection:ty, $button_type:expr) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &Transform, &InitialPosition, &$flow),
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
            for (entity, transform, initial_position, flow) in &query {
                if flow.system != **focused_system {
                    continue;
                }

                let direction = transform.right().truncate();

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: $button_type,
                        connection_source: entity,
                        system: **focused_system,
                    },
                    **initial_position + direction * 64.0,
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
