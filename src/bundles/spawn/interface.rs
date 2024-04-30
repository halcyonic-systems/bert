use crate::components::*;
use crate::constants::{INTERFACE_LINE_WIDTH, INTERFACE_SELECTED_LINE_WIDTH, INTERFACE_Z};
use crate::events::InterfaceDrag;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::resources::{FixedSystemElementGeometries, FocusedSystem};
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_interface(
    commands: &mut Commands,
    interface_type: InterfaceType,
    substance_type: SubstanceType,
    flow_entity: Entity,
    transform: &Transform,
    initial_position: &InitialPosition,
    focused_system: &Res<FocusedSystem>,
    fixed_system_element_geometries: &Res<FixedSystemElementGeometries>,
    zoom: f32,
    is_selected: bool,
) -> Entity {
    let (mut transform, initial_position) =
        ui_transform_from_button(transform, initial_position, INTERFACE_Z, 0.0, zoom);

    // Normalize the rotation
    transform.rotation = Quat::from_rotation_z(transform.translation.truncate().to_angle());

    let interface_entity = commands
        .spawn((
            Interface::default(),
            SpatialBundle {
                transform,
                ..default()
            },
            Fill::color(substance_type.interface_color()),
            PickableBundle {
                selection: PickSelection { is_selected },
                ..default()
            },
            HighlightBundles {
                idle: Stroke::new(Color::BLACK, INTERFACE_LINE_WIDTH),
                selected: Stroke::new(Color::BLACK, INTERFACE_SELECTED_LINE_WIDTH),
            },
            SystemElement::Interface,
            Name::new("Interface"),
            ElementDescription::default(),
            initial_position,
            fixed_system_element_geometries.interface.clone(),
            On::<Pointer<Drag>>::send_event::<InterfaceDrag>(),
        ))
        .id();

    commands
        .entity(***focused_system)
        .add_child(interface_entity);

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

    interface_entity
}
