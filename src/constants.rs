//! The application relies on several constant values which are aggregated into this file
use bevy::asset::Handle;
use bevy::prelude::{Color, ColorMaterial};

/// Minimum scale of a system element before the visibility is switched to hidden.
pub const SCALE_VISIBILITY_THRESHOLD: f32 = 0.2;
/// Minimum scale of a label before the visibility is switched to hidden.
pub const LABEL_SCALE_VISIBILITY_THRESHOLD: f32 = 0.5;
// Default z-index of a label.
pub const LABEL_Z: f32 = 150.0;

/// Default width of a line from which all other lines are defined.
pub const DEFAULT_LINE_WIDTH: f32 = 3.0;

/// Radius of the initial system circle created upon app startup. 
pub const MAIN_SYSTEM_RADIUS: f32 = 300.0;
/// Default width of the curve outlining the circumference of a system circle.  
pub const SYSTEM_LINE_WIDTH: f32 = DEFAULT_LINE_WIDTH + 1.0;
/// Default width of the curve outlining the circumference of a selected system circle.
pub const SYSTEM_SELECTED_LINE_WIDTH: f32 = SYSTEM_LINE_WIDTH + 2.0;
/// Default width of the curve overlaid on top of the circumference of a selected system circle.
pub const SYSTEM_SELECTED_INNER_LINE_WIDTH: f32 = SYSTEM_SELECTED_LINE_WIDTH - 4.0;

/// Default length of the line segment of a flow.
pub const FLOW_LENGTH: f32 = 200.0;
/// Default width of the line segment of a flow.
pub const FLOW_LINE_WIDTH: f32 = 3.0;
/// Default width of the line segment of a selected flow.
pub const FLOW_SELECTED_LINE_WIDTH: f32 = 5.0;
/// Default width of the selection line segment overlaid on top of a selected flow's line segment.
pub const FLOW_SELECTED_INNER_LINE_WIDTH: f32 = FLOW_SELECTED_LINE_WIDTH - 4.0;
/// Default z-index of a flow.
pub const FLOW_Z: f32 = 1.0;

pub const FLOW_CLICK_WIDTH: f32 = 20.0;
pub const FLOW_CLICK_TOLERANCE: f32 = 1.0;

/// Default length of the triangle that defines the arrow head of a flow.
pub const FLOW_ARROW_HEAD_LENGTH: f32 = 20.0;
/// Default half width of the triangle that defines the arrow head of a flow.
pub const FLOW_ARROW_HEAD_WIDTH_HALF: f32 = 10.0;

/// Default number that defines the scale factor used in the (re)sizing a subsystem.
pub const SUBSYSTEM_SCALING_FACTOR: f32 = 0.4;
/// Default number that defines the lower bound of the scale factor used in the (re)sizing a subsystem.
pub const SUBSYSTEM_MIN_SCALING_FACTOR: f32 = 0.2;
/// Default z-index of a subsystem.
pub const SUBSYSTEM_Z: f32 = 10.0;
/// Size of subsystems are scaled based on the number of interfaces it has until it reaches this limit.
pub const SUBSYSTEM_FULL_SIZE_INTERFACE_COUNT: f32 = 8.0;

/// Default half width of the rectangle that defines an interface.
pub const INTERFACE_WIDTH_HALF: f32 = 12.0;
/// Default half height of the rectangle that defines an interface.
pub const INTERFACE_HEIGHT_HALF: f32 = 30.0;
/// Default line width of the perimeter of the rectangle that defines an interface.
pub const INTERFACE_LINE_WIDTH: f32 = DEFAULT_LINE_WIDTH;
/// Default line width of the perimeter of the rectangle of a selected interface.
pub const INTERFACE_SELECTED_LINE_WIDTH: f32 = INTERFACE_LINE_WIDTH + 2.0;
/// Default line width of the selection lines overlaid on the perimeter of the rectangle of a selected interface.
pub const INTERFACE_SELECTED_INNER_LINE_WIDTH: f32 = INTERFACE_SELECTED_LINE_WIDTH - 4.0;
/// Default z-index of an interface.
pub const INTERFACE_Z: f32 = 100.0;

/// Default half width of a create-button.
pub const BUTTON_WIDTH_HALF: f32 = 16.0;

/// Default half width of an external entity.
pub const EXTERNAL_ENTITY_WIDTH_HALF: f32 = 20.0;
/// Default half height of an external entity.
pub const EXTERNAL_ENTITY_HEIGHT_HALF: f32 = 60.0;
/// Default line width of the lines that define an external entity.
pub const EXTERNAL_ENTITY_LINE_WIDTH: f32 = DEFAULT_LINE_WIDTH;
/// Default line width of the lines that define a selected external entity.
pub const EXTERNAL_ENTITY_SELECTED_LINE_WIDTH: f32 = EXTERNAL_ENTITY_LINE_WIDTH + 2.0;
/// Default line width of the selection lines overlaid on a selected external entity.
pub const EXTERNAL_ENTITY_SELECTED_INNER_LINE_WIDTH: f32 =
    EXTERNAL_ENTITY_SELECTED_LINE_WIDTH - 4.0;
/// Default z-index of an external entity.
pub const EXTERNAL_ENTITY_Z: f32 = 1.0;

/// The initial handle for the default color material on some meshes.
pub const WHITE_COLOR_MATERIAL_HANDLE: Handle<ColorMaterial> =
    Handle::weak_from_u128(0xE4E775785EA288EEA9FB9EC95377D078);

/// The default background color of the canvas. It's also used as the background color for flow labels in the environment.
pub const CLEAR_COLOR: Color = Color::ANTIQUE_WHITE;
