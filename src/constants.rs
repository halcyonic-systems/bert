use bevy::asset::Handle;
use bevy::prelude::ColorMaterial;

pub const DEFAULT_LINE_WIDTH: f32 = 3.0;

pub const MAIN_SYSTEM_RADIUS: f32 = 300.0;

pub const SYSTEM_LINE_WIDTH: f32 = DEFAULT_LINE_WIDTH + 1.0;

pub const FLOW_END_LENGTH: f32 = 40.0;
pub const FLOW_LENGTH: f32 = 200.0;
pub const FLOW_LINE_WIDTH: f32 = 3.0;
pub const FLOW_SELECTED_LINE_WIDTH: f32 = 5.0;
pub const FLOW_SELECTED_INNER_LINE_WIDTH: f32 = FLOW_SELECTED_LINE_WIDTH - 4.0;
pub const FLOW_Z: f32 = 1.0;

pub const FLOW_CLICK_WIDTH: f32 = 20.0;
pub const FLOW_CLICK_TOLERANCE: f32 = 1.0;

pub const FLOW_ARROW_HEAD_LENGTH: f32 = 20.0;
pub const FLOW_ARROW_HEAD_WIDTH_HALF: f32 = 10.0;

pub const SUBSYSTEM_SCALING_FACTOR: f32 = 0.3;
pub const SUBSYSTEM_RADIUS_FRACTION: f32 = 0.2;
pub const SUBSYSTEM_Z: f32 = 10.0;

pub const INTERFACE_WIDTH_HALF: f32 = 12.0;
pub const INTERFACE_HEIGHT_HALF: f32 = 30.0;
pub const INTERFACE_LINE_WIDTH: f32 = DEFAULT_LINE_WIDTH;
pub const INTERFACE_SELECTED_LINE_WIDTH: f32 = INTERFACE_LINE_WIDTH + 2.0;
pub const INTERFACE_SELECTED_INNER_LINE_WIDTH: f32 = INTERFACE_SELECTED_LINE_WIDTH - 4.0;
pub const INTERFACE_Z: f32 = 100.0;

pub const BUTTON_WIDTH_HALF: f32 = 16.0;

pub const EXTERNAL_ENTITY_WIDTH_HALF: f32 = 20.0;
pub const EXTERNAL_ENTITY_HEIGHT_HALF: f32 = 60.0;
pub const EXTERNAL_ENTITY_LINE_WIDTH: f32 = DEFAULT_LINE_WIDTH;
pub const EXTERNAL_ENTITY_SELECTED_LINE_WIDTH: f32 = EXTERNAL_ENTITY_LINE_WIDTH + 2.0;
pub const EXTERNAL_ENTITY_SELECTED_INNER_LINE_WIDTH: f32 =
    EXTERNAL_ENTITY_SELECTED_LINE_WIDTH - 4.0;
pub const EXTERNAL_ENTITY_Z: f32 = 1.0;

pub const WHITE_COLOR_MATERIAL_HANDLE: Handle<ColorMaterial> =
    Handle::weak_from_u128(0xE4E775785EA288EEA9FB9EC95377D078);
