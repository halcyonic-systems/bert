use crate::components::*;
use crate::constants::{
    BUTTON_WIDTH_HALF, EXTERNAL_ENTITY_LINE_WIDTH, EXTERNAL_ENTITY_SELECTED_LINE_WIDTH,
    EXTERNAL_ENTITY_WIDTH_HALF, EXTERNAL_ENTITY_Z,
};
use crate::events::ExternalEntityDrag;
use crate::plugins::label::{add_name_label, Alignment};
use crate::plugins::lyon_selection::HighlightBundles;
use crate::plugins::mouse_interaction::DragPosition;
use crate::plugins::mouse_interaction::PickSelection;
use crate::resources::{FixedSystemElementGeometriesByNestingLevel, StrokeTessellator};
use crate::utils::ui_transform_from_button;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_external_entity(
    commands: &mut Commands,
    subsystem_query: &Query<&Subsystem>,
    nesting_level_query: &Query<&NestingLevel>,
    focused_system: Entity,
    interface_type: InterfaceType,
    substance_type: SubstanceType,
    flow_entity: Entity,
    transform: &Transform,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: f32,
    is_selected: bool,
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
    name: &str,
    description: &str,
) -> Entity {
    let (transform, initial_position) = ui_transform_from_button(
        transform,
        EXTERNAL_ENTITY_Z,
        EXTERNAL_ENTITY_WIDTH_HALF - BUTTON_WIDTH_HALF,
        zoom,
    );

    let nesting_level = NestingLevel::current(focused_system, nesting_level_query);

    let external_entity = spawn_external_entity_only(
        commands,
        substance_type,
        is_selected,
        name,
        description,
        transform,
        initial_position,
        nesting_level,
        zoom,
        fixed_system_element_geometries,
        meshes,
        tess,
    );

    if let Ok(subsystem) = subsystem_query.get(focused_system) {
        commands
            .entity(subsystem.parent_system)
            .add_child(external_entity);
    }

    let mut entity_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            entity_commands.insert(FlowStartConnection {
                target: external_entity,
                target_type: StartTargetType::Source,
            });
        }
        InterfaceType::Export => {
            entity_commands.insert(FlowEndConnection {
                target: external_entity,
                target_type: EndTargetType::Sink,
            });
        }
    }

    external_entity
}

pub fn spawn_external_entity_only(
    commands: &mut Commands,
    substance_type: SubstanceType,
    is_selected: bool,
    name: &str,
    description: &str,
    transform: Transform,
    initial_position: InitialPosition,
    nesting_level: u16,
    zoom: f32,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
) -> Entity {
    let color = substance_type.flow_color();

    let scale = NestingLevel::compute_scale(nesting_level, zoom);

    commands
        .spawn((
            ExternalEntity {
                equivalence: "".to_string(),
                model: "".to_string(),
            },
            SpatialBundle {
                transform,
                ..default()
            },
            HighlightBundles {
                idle: Stroke::new(color, EXTERNAL_ENTITY_LINE_WIDTH * scale),
                selected: Stroke {
                    color,
                    options: StrokeOptions::default()
                        .with_line_width(EXTERNAL_ENTITY_SELECTED_LINE_WIDTH)
                        .with_line_cap(LineCap::Round),
                },
            },
            PickableBundle::default(),
            PickSelection { is_selected },
            SystemElement::ExternalEntity,
            Name::new(name.to_string()),
            ElementDescription::new(description),
            initial_position,
            fixed_system_element_geometries
                .get_or_create(nesting_level, zoom, meshes, tess)
                .external_entity,
            NestingLevel::new(nesting_level),
            On::<DragPosition>::send_event::<ExternalEntityDrag>(),
        ))
        .id()
}

// TODO : do the entire external entity like this autospawn?
pub fn auto_spawn_external_entity_label(
    mut commands: Commands,
    external_entity_query: Query<(Entity, &NestingLevel), Added<ExternalEntity>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
) {
    for (external_entity, nesting_level) in external_entity_query.iter() {
        add_name_label(
            &mut commands,
            external_entity,
            vec2(70.0, 100.0),
            vec3(1.0, 0.0, 0.0),
            Alignment::Auto,
            Alignment::Center,
            &name_query,
            &asset_server,
            *nesting_level,
        );
    }
}
