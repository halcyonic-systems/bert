use crate::bevy_app::components::*;
use crate::bevy_app::constants::{
    INTERFACE_HEIGHT_HALF, INTERFACE_LINE_WIDTH, INTERFACE_SELECTED_LINE_WIDTH, INTERFACE_Z,
};
use crate::bevy_app::events::InterfaceDrag;
use crate::bevy_app::plugins::label::{add_name_label, Alignment, CopyPositionArgs};
use crate::bevy_app::plugins::lyon_selection::HighlightBundles;
use crate::bevy_app::plugins::mouse_interaction::DragPosition;
use crate::bevy_app::plugins::mouse_interaction::PickSelection;
use crate::bevy_app::resources::{FixedSystemElementGeometriesByNestingLevel, StrokeTessellator};
use crate::bevy_app::utils::ui_transform_from_button;
use crate::plugins::label::{HorizontalAttachmentAnchor, VerticalAttachmentAnchor};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn spawn_interface(
    commands: &mut Commands,
    interface_type: InterfaceType,
    substance_type: SubstanceType,
    flow_entity: Entity,
    transform: &Transform,
    nesting_level: u16,
    focused_system: Entity,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
    zoom: f32,
    is_selected: bool,
    meshes: &mut ResMut<Assets<Mesh>>,
    tess: &mut ResMut<StrokeTessellator>,
    name: &str,
    description: &str,
) -> Entity {
    let (mut transform, initial_position) =
        ui_transform_from_button(transform, INTERFACE_Z, 0.0, zoom);

    // Normalize the rotation
    transform.rotation = Quat::from_rotation_z(transform.translation.truncate().to_angle());

    let interface_entity = spawn_interface_only(
        commands,
        substance_type,
        nesting_level,
        focused_system,
        zoom,
        is_selected,
        name,
        description,
        transform,
        initial_position,
        tess,
        meshes,
        fixed_system_element_geometries,
    );

    let mut flow_commands = commands.entity(flow_entity);

    match interface_type {
        InterfaceType::Import => {
            flow_commands.insert(FlowEndInterfaceConnection {
                target: interface_entity,
            });
        }
        InterfaceType::Export => {
            flow_commands.insert(FlowStartInterfaceConnection {
                target: interface_entity,
            });
        }
    }

    interface_entity
}

pub fn spawn_interface_only(
    commands: &mut Commands,
    substance_type: SubstanceType,
    nesting_level: u16,
    parent_system: Entity,
    zoom: f32,
    is_selected: bool,
    name: &str,
    description: &str,
    transform: Transform,
    initial_position: InitialPosition,
    tess: &mut ResMut<StrokeTessellator>,
    meshes: &mut ResMut<Assets<Mesh>>,
    fixed_system_element_geometries: &mut ResMut<FixedSystemElementGeometriesByNestingLevel>,
) -> Entity {
    let scale = NestingLevel::compute_scale(nesting_level, zoom);

    let interface_entity = commands
        .spawn((
            Interface::default(),
            transform,
            Fill::color(substance_type.interface_color()),
            PickingBehavior::default(),
            RayCastPickable::default(),
            PickSelection { is_selected },
            HighlightBundles {
                idle: Stroke::new(Color::BLACK, INTERFACE_LINE_WIDTH * scale),
                selected: Stroke::new(Color::BLACK, INTERFACE_SELECTED_LINE_WIDTH),
            },
            SystemElement::Interface,
            Name::new(name.to_string()),
            ElementDescription::new(description),
            initial_position,
            fixed_system_element_geometries
                .get_or_create(nesting_level, zoom, meshes, tess)
                .interface
                .clone(),
            NestingLevel::new(nesting_level),
        ))
        .observe(
            |on_drag: Trigger<DragPosition>, mut writer: EventWriter<InterfaceDrag>| {
                writer.send(on_drag.into());
            },
        )
        .id();

    commands.entity(parent_system).add_child(interface_entity);
    interface_entity
}

pub fn auto_spawn_interface_label(
    mut commands: Commands,
    interface_query: Query<(Entity, &NestingLevel), Added<Interface>>,
    name_query: Query<&Name>,
    asset_server: Res<AssetServer>,
) {
    for (interface, nesting_level) in interface_query.iter() {
        add_name_label(
            &mut commands,
            interface,
            vec2(50.0, 45.0),
            None,
            Some(CopyPositionArgs {
                offset: vec3(INTERFACE_HEIGHT_HALF * 1.7, -INTERFACE_HEIGHT_HALF, 0.0),
                horizontal_alignment: Alignment::AutoStartEnd,
                vertical_alignment: Alignment::Center,
                horizontal_anchor: HorizontalAttachmentAnchor::default(),
                vertical_anchor: VerticalAttachmentAnchor::default(),
            }),
            false,
            &name_query,
            &asset_server,
            None,
            *nesting_level,
        );
    }
}
