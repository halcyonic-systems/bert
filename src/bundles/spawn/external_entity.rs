use crate::components::*;
use crate::constants::{BUTTON_WIDTH_HALF, EXTERNAL_ENTITY_LINE_WIDTH, EXTERNAL_ENTITY_WIDTH_HALF};
use crate::resources::FixedSystemElementGeometries;
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::events::ExternalEntityDrag;

pub fn spawn_external_entity(
    commands: &mut Commands,
    interface_type: InterfaceType,
    flow_entity: Entity,
    transform: &Transform,
    initial_position: &InitialPosition,
    fixed_system_element_geometries: &Res<FixedSystemElementGeometries>,
    zoom: f32,
) {
    let (transform, initial_position) = ui_transform_from_button(
        transform,
        initial_position,
        1.0,
        EXTERNAL_ENTITY_WIDTH_HALF - BUTTON_WIDTH_HALF,
        zoom,
    );

    let external_entity = commands
        .spawn((
            ExternalEntity::default(),
            SpatialBundle {
                transform,
                ..default()
            },
            Stroke::new(Color::BLACK, EXTERNAL_ENTITY_LINE_WIDTH),
            PickableBundle {
                selection: PickSelection { is_selected: true },
                ..default()
            },
            SystemElement::ExternalEntity,
            Name::new("External Entity"),
            ElementDescription::default(),
            initial_position,
            fixed_system_element_geometries.external_entity.clone(),
            On::<Pointer<Drag>>::send_event::<ExternalEntityDrag>(),
        ))
        .id();

    let mut entity_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            entity_commands.insert(InflowSourceConnection {
                target: external_entity,
            });
        }
        InterfaceType::Export => {
            entity_commands.insert(OutflowSinkConnection {
                target: external_entity,
            });
        }
    }
}
