use crate::bundles::spawn_create_button;
use crate::components::*;
use crate::constants::BUTTON_WIDTH_HALF;
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;

macro_rules! external_entity_create_button {
    (
        $fn_name:ident,
        $flow_conn_ty:ty,
        $interface_connection:ty,
        $terminal_connection:ty,
        $button_type:expr,
        $target_button_type:expr,
        $side:tt,
        $side_dir:tt
    ) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &FlowCurve, &Flow, &$flow_conn_ty),
                (
                    With<$interface_connection>,
                    Without<$terminal_connection>,
                    Without<HasFlowOtherEndButton>,
                ),
            >,
            subsystem_query: Query<&Subsystem>,
            focused_system: Res<FocusedSystem>,
            zoom: Res<Zoom>,
            asset_server: Res<AssetServer>,
        ) {
            for (entity, flow_curve, flow, flow_system_connection) in &query {
                if flow_system_connection.target != **focused_system {
                    continue;
                }

                let direction = -flow_curve.$side_dir;

                let (button_type, parent) = if let Ok(subsystem) = subsystem_query.get(**focused_system) {
                    ($target_button_type, Some(subsystem.parent_system))
                } else {
                    ($button_type, None)
                };

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: button_type,
                        connection_source: entity,
                        system: **focused_system,
                        substance_type: Some(flow.substance_type),
                    },
                    (flow_curve.$side + direction * BUTTON_WIDTH_HALF) / **zoom,
                    direction.to_angle(),
                    **zoom,
                    parent,
                    &asset_server,
                );
            }
        }
    };
}

external_entity_create_button!(
    add_source_create_button,
    FlowEndConnection,
    FlowEndInterfaceConnection,
    FlowStartConnection,
    CreateButtonType::Source,
    CreateButtonType::FlowTerminalStart,
    start,
    start_direction
);
external_entity_create_button!(
    add_sink_create_button,
    FlowStartConnection,
    FlowStartInterfaceConnection,
    FlowEndConnection,
    CreateButtonType::Sink,
    CreateButtonType::FlowTerminalEnd,
    end,
    end_direction
);
