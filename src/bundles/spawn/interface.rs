use crate::components::*;
use crate::constants::INTERFACE_LINE_WIDTH;
use crate::resources::FixedSystemElementGeometries;
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_interface(
    commands: &mut Commands,
    interface_type: InterfaceType,
    flow_entity: Entity,
    transform: &GlobalTransform,
    initial_position: &InitialPosition,
    fixed_system_element_geometries: &Res<FixedSystemElementGeometries>,
    zoom: f32,
) {
    let (transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 5.0, 0.0, zoom);

    let interface_entity = commands
        .spawn((
            Interface::default(),
            SpatialBundle {
                transform,
                ..default()
            },
            Stroke::new(Color::BLACK, INTERFACE_LINE_WIDTH),
            Fill::color(Color::WHITE),
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::Interface,
            Name::new("Interface"),
            initial_position,
            fixed_system_element_geometries.interface.clone(),
        ))
        .id();

    let mut entity_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            entity_commands.insert(InflowInterfaceConnection {
                target: interface_entity,
            });
        }
        InterfaceType::Export => {
            entity_commands.insert(OutflowInterfaceConnection {
                target: interface_entity,
            });
        }
    }
}
