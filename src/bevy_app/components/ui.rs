use crate::bevy_app::components::{InitialPosition, InterfaceType, SubstanceType};
use crate::bevy_app::constants::FLOW_LENGTH;
use bevy::prelude::*;

// ============================================================================
// Drag-and-Drop Palette Components (Phase 1)
// ============================================================================

/// Marks an entity as a draggable palette element in the sidebar.
/// Palette elements spawn new diagram elements when dragged to canvas.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct PaletteElement {
    pub element_type: PaletteElementType,
}

/// Types of elements available in the drag-and-drop palette.
///
/// Aligned with Mobus 8-tuple formalization:
/// - Subsystem: Components (c ∈ C, internal processing)
/// - Interface: Boundary components (c ∈ I ⊂ C, protocol-mediated exchange)
/// - EnvironmentalObject: External entities (o ∈ O, unified sources/sinks)
///
/// Flows are NOT palette items - they are edges in N (internal) or G (external)
/// networks, created via connection mode.
///
/// Key insight: Direction determined by flow edges, not node types.
/// Environmental objects can be BOTH source AND sink simultaneously.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum PaletteElementType {
    /// Internal component (c ∈ C, not interface) - processes within system
    Subsystem,

    /// Boundary component (c ∈ I ⊂ C) - mediates flows across boundary
    Interface,

    /// External entity (o ∈ O) - environmental object, unified source/sink
    EnvironmentalObject,
}

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
    InterfaceSubsystem {
        is_child_of_interface: bool,
        interface_type: InterfaceType,
    },
    FlowTerminalStart,
    FlowTerminalEnd,
    Subsystem,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct HasFlowInterfaceButton {
    pub button_entity: Entity,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct HasFlowOtherEndButton;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct HasInterfaceSubsystemButton {
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

impl Default for FlowCurve {
    fn default() -> Self {
        Self {
            start: Vec2::ZERO,
            end: Vec2::X,
            start_direction: Vec2::X,
            end_direction: -Vec2::X,
        }
    }
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
        let zoomed_pos = *initial_position * zoom;

        Self {
            start: zoomed_pos + direction * FLOW_LENGTH * scale * zoom,
            start_direction: -direction,
            end: zoomed_pos,
            end_direction: direction,
        }
    }

    pub fn outflow(
        zoom: f32,
        initial_position: InitialPosition,
        direction: Vec2,
        scale: f32,
    ) -> Self {
        let zoomed_pos = *initial_position * zoom;

        Self {
            start: zoomed_pos,
            start_direction: direction,
            end: zoomed_pos + direction * FLOW_LENGTH * scale * zoom,
            end_direction: -direction,
        }
    }

    #[inline]
    pub fn compute_tangent_length(&self) -> f32 {
        Self::compute_tangent_length_from_points(self.start, self.end)
    }

    #[inline]
    pub fn compute_tangent_length_from_points(start: Vec2, end: Vec2) -> f32 {
        (end - start).length() * 0.3333
    }

    pub fn skip_start(&self) -> Self {
        Self {
            start: self.start + self.start_direction,
            ..*self
        }
    }
}

/// User-defined RELATIVE offsets for flow endpoints.
/// These offsets are ADDED to the computed base positions each frame,
/// providing stable positioning without feedback loops.
///
/// Angles are stored in radians, measured from positive X-axis (standard math convention).
/// Using angles makes the offset zoom-independent - positions scale correctly when zooming.
#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Default)]
#[reflect(Component)]
pub struct FlowEndpointOffset {
    /// Angular position (radians) of start endpoint on its subsystem boundary.
    /// None = use natural/computed position from FlowCurve
    pub start_angle: Option<f32>,
    /// Angular position (radians) of end endpoint on its subsystem boundary.
    /// None = use natural/computed position from FlowCurve
    pub end_angle: Option<f32>,
}

impl FlowEndpointOffset {
    pub fn has_offset(&self) -> bool {
        self.start_angle.is_some() || self.end_angle.is_some()
    }

    pub fn with_start_angle(angle: f32) -> Self {
        Self {
            start_angle: Some(angle),
            end_angle: None,
        }
    }

    pub fn with_end_angle(angle: f32) -> Self {
        Self {
            start_angle: None,
            end_angle: Some(angle),
        }
    }

    pub fn with_both_angles(start: f32, end: f32) -> Self {
        Self {
            start_angle: Some(start),
            end_angle: Some(end),
        }
    }
}

/// Marker component for flow endpoint handle entities (the draggable circles)
#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowEndpointHandle {
    /// The flow entity this handle belongs to
    pub flow: Entity,
    /// Which end of the flow this handle controls
    pub endpoint: FlowEndpoint,
}

#[derive(Copy, Clone, Debug, Reflect, PartialEq, Eq)]
pub enum FlowEndpoint {
    Start,
    End,
}

/// Marker component indicating that endpoint handles have been spawned for this flow.
#[derive(Copy, Clone, Debug, Component, Reflect, Default, PartialEq, Eq)]
#[reflect(Component)]
pub struct FlowEndpointHandlesSpawned;

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub enum FlowTerminalSelecting {
    Start,
    End,
}

#[derive(Copy, Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct SelectedHighlightHelperAdded {
    pub helper_entity: Entity,
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct Hidden;

#[derive(Clone, Debug, Component, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct ParentState {
    pub name: String,
    pub description: String,
}
