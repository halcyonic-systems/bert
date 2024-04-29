use crate::components::*;
use crate::constants::{
    BUTTON_WIDTH_HALF, EXTERNAL_ENTITY_LINE_WIDTH, EXTERNAL_ENTITY_SELECTED_LINE_WIDTH,
    EXTERNAL_ENTITY_WIDTH_HALF, EXTERNAL_ENTITY_Z,
};
use crate::events::ExternalEntityDrag;
use crate::plugins::lyon_selection::HighlightBundles;
use crate::resources::{FixedSystemElementGeometries, FocusedSystem};
use crate::utils::ui_transform_from_button;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_external_entity(
    commands: &mut Commands,
    subsystem_query: &Query<&Subsystem>,
    focused_system: &Res<FocusedSystem>,
    interface_type: InterfaceType,
    flow_entity: Entity,
    transform: &Transform,
    initial_position: &InitialPosition,
    fixed_system_element_geometries: &Res<FixedSystemElementGeometries>,
    zoom: f32,
    is_selected: bool,
) -> Entity {
    let (transform, initial_position) = ui_transform_from_button(
        transform,
        initial_position,
        EXTERNAL_ENTITY_Z,
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
            HighlightBundles {
                idle: Stroke::new(Color::BLACK, EXTERNAL_ENTITY_LINE_WIDTH),
                selected: Stroke {
                    color: Color::BLACK,
                    options: StrokeOptions::default()
                        .with_line_width(EXTERNAL_ENTITY_SELECTED_LINE_WIDTH)
                        .with_line_cap(LineCap::Round),
                },
            },
            PickableBundle {
                selection: PickSelection { is_selected },
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

    if let Ok(subsystem) = subsystem_query.get(***focused_system) {
        commands
            .entity(subsystem.parent_system)
            .add_child(external_entity);
    }

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

    external_entity
}
