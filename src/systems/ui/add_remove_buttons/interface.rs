use crate::bundles::spawn_create_button;
use crate::components::*;
use crate::resources::{FocusedSystem, Zoom};
use crate::utils::combined_transform_of_entity_until_ancestor;
use bevy::prelude::*;

macro_rules! interface_create_button {
    ($fn_name:ident, $flow_conn_ty:ty, $interface_connection:ty, $button_type:expr, $side:tt, $side_dir:tt) => {
        pub fn $fn_name(
            mut commands: Commands,
            query: Query<
                (Entity, &FlowCurve, &Flow, &$flow_conn_ty, Option<&Parent>),
                (
                    Without<$interface_connection>,
                    Without<HasFlowInterfaceButton>,
                ),
            >,
            parent_query: Query<&Parent>,
            transform_query: Query<&Transform>,
            focused_system: Res<FocusedSystem>,
            zoom: Res<Zoom>,
            asset_server: Res<AssetServer>,
        ) {
            for (entity, flow_curve, flow, flow_system_connection, flow_parent) in &query {
                if flow_system_connection.target != **focused_system {
                    continue;
                }

                let (direction, position) = if let Some(parent) = flow_parent {
                    let transform = combined_transform_of_entity_until_ancestor(
                        **focused_system,
                        Some(parent.get()),
                        &transform_query,
                        &parent_query,
                    );

                    let inverse_transform = transform.compute_affine().inverse();

                    (
                        inverse_transform
                            .transform_vector3(flow_curve.$side_dir.extend(0.0))
                            .truncate(),
                        inverse_transform
                            .transform_point3(flow_curve.$side.extend(0.0))
                            .truncate()
                            / **zoom,
                    )
                } else {
                    (flow_curve.$side_dir, flow_curve.$side)
                };

                spawn_create_button(
                    &mut commands,
                    CreateButton {
                        ty: $button_type,
                        connection_source: entity,
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
