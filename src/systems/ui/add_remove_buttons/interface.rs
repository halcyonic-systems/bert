use crate::bundles::spawn_create_button;
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use bevy::prelude::*;

macro_rules! interface_create_button {
    ($fn_name:ident, $flow_conn_ty:ty, $interface_connection:ty, $button_type:expr, $side:tt, $side_dir:tt) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &FlowCurve, &Flow, &$flow_conn_ty),
                (
                    Without<$interface_connection>,
                    Without<HasFlowInterfaceButton>,
                ),
            >,
            transform_query: Query<&GlobalTransform>,
            focused_system: Res<FocusedSystem>,
            zoom: Res<Zoom>,
            asset_server: Res<AssetServer>,
        ) {
            for (flow_entity, flow_curve, flow, flow_system_connection) in &query {
                if flow_system_connection.target != **focused_system {
                    continue;
                }

                let flow_transform = transform_query
                    .get(flow_entity)
                    .expect("Flow should have global transform");

                let system_transform = transform_query
                    .get(**focused_system)
                    .expect("Focused system should have global transform");

                let combined = system_transform.affine().inverse() * flow_transform.affine();

                let direction = combined
                    .transform_vector3(flow_curve.$side_dir.extend(0.0))
                    .truncate();
                let position = combined
                    .transform_point3(flow_curve.$side.extend(0.0))
                    .truncate()
                    / **zoom;

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: $button_type,
                        connection_source: flow_entity,
                        system: **focused_system,
                        substance_type: Some(flow.substance_type),
                    },
                    position,
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
    FlowStartConnection,
    FlowStartInterfaceConnection,
    CreateButtonType::ExportInterface,
    start,
    start_direction
);
interface_create_button!(
    add_inflow_interface_create_button,
    FlowEndConnection,
    FlowEndInterfaceConnection,
    CreateButtonType::ImportInterface,
    end,
    end_direction
);
