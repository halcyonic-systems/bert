use crate::components::{InitialPosition, SubstanceType};
use crate::constants::{FLOW_END_LENGTH, FLOW_LENGTH};
use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct CreateButton {
    pub ty: CreateButtonType,
    pub connection_source: Entity,
    pub system: Entity,
    pub substance_type: Option<SubstanceType>,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum CreateButtonType {
    ImportInterface,
    ExportInterface,
    Inflow,
    Outflow,
    Source,
    Sink,
    InterfaceSubsystem { is_child_of_interface: bool },
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowInterfaceButton;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowOtherEndButton;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct InterfaceSubsystemButton {
    pub button_entity: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct FlowCurve {
    pub start: Vec2,
    pub end: Vec2,
    pub start_direction: Vec2,
    pub end_direction: Vec2,
}

impl FlowCurve {
    pub fn head_rotation(&self) -> Quat {
        Quat::from_rotation_z(self.end_direction.to_angle())
    }

    pub fn inflow(
        zoom: f32,
        initial_position: InitialPosition,
        direction: Vec2,
        scale: f32,
    ) -> Self {
        Self {
            start: (*initial_position + direction * FLOW_LENGTH * scale) * zoom,
            start_direction: direction * -FLOW_END_LENGTH * zoom,
            end: *initial_position * zoom,
            end_direction: direction * FLOW_END_LENGTH * zoom,
        }
    }

    pub fn outflow(
        zoom: f32,
        initial_position: InitialPosition,
        direction: Vec2,
        scale: f32,
    ) -> Self {
        Self {
            start: *initial_position * zoom,
            start_direction: direction * FLOW_END_LENGTH * zoom,
            end: (*initial_position + direction * FLOW_LENGTH * scale) * zoom,
            end_direction: direction * -FLOW_END_LENGTH * zoom,
        }
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct SelectedHighlightHelperAdded {
    pub helper_entity: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq)]
#[reflect(Component)]
pub struct Pin {
    pub target: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct Pinnable {
    pub has_pins: bool,
}
