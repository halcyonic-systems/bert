use crate::components::*;
use crate::constants::{
    INTERFACE_LINE_WIDTH, INTERFACE_SELECTED_INNER_LINE_WIDTH, INTERFACE_SELECTED_LINE_WIDTH,
};
use crate::events::InterfaceDrag;
use crate::plugins::lyon_selection::{HighlightBundles, SelectedSpawnListener, SpawnOnSelected};
use crate::resources::{FixedSystemElementGeometries, FocusedSystem};
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
    system_query: &Query<(&Transform, &crate::components::System)>,
    focused_system: &Res<FocusedSystem>,
    fixed_system_element_geometries: &Res<FixedSystemElementGeometries>,
    zoom: f32,
) {
    let (mut transform, initial_position) =
        ui_transform_from_button(transform, initial_position, 5.0, 0.0, zoom);

    let (system_transform, _) = system_query
        .get(***focused_system)
        .expect("focused system not found");

    // Normalize the rotation
    transform.rotation = Quat::from_rotation_z(
        (transform.translation.truncate() - system_transform.translation.truncate()).to_angle(),
    );

    let interface_entity = commands
        .spawn((
            Interface::default(),
            SpatialBundle {
                transform,
                ..default()
            },
            Fill::color(Color::WHITE),
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            HighlightBundles {
                idle: Stroke::new(Color::BLACK, INTERFACE_LINE_WIDTH),
                selected: Stroke::new(Color::BLACK, INTERFACE_SELECTED_LINE_WIDTH),
            },
            SpawnOnSelected::new(spawn_selected_interface),
            SystemElement::Interface,
            Name::new("Interface"),
            initial_position,
            fixed_system_element_geometries.interface.clone(),
            On::<Pointer<Drag>>::send_event::<InterfaceDrag>(),
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

fn spawn_selected_interface(
    mut commands: Commands,
    mut listener: SelectedSpawnListener,
    transform_query: Query<&Transform>,
    fixed_system_element_geometries: Res<FixedSystemElementGeometries>,
) {
    let mut transform = transform_query
        .get(listener.selected())
        .expect("Selected entity should have a transform")
        .clone();

    transform.translation.z += 1.0;

    listener.add_spawned(
        commands
            .spawn((
                SpatialBundle {
                    transform,
                    ..default()
                },
                fixed_system_element_geometries.interface.clone(),
                Stroke::new(Color::WHITE, INTERFACE_SELECTED_INNER_LINE_WIDTH),
            ))
            .id(),
    );
}
