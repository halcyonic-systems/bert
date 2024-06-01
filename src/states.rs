//! Custom defined states
//! Primarily used to determine system ordering and run conditions.

use bevy::prelude::*;
/// The `AppState` enum represents the various states the application can be in.
/// 
/// ## States
/// - `Normal`: The default state of the application.
/// - `FlowTerminalSelection`: This state is activated when the user clicks a source or sink button in a nested flow.
/// 
/// ## Behavior in `FlowTerminalSelection` State
/// - When a nested source/sink button is clicked, the terminal selection starts where the flow end is attached to the mouse cursor.
/// - In this mode, switching the focused system is disabled.
/// - Clicking on items will only set them as targets for the flow, without selecting them.
/// 
/// Different systems should operate based on the current state of the application.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Normal,
    FlowTerminalSelection,
}
