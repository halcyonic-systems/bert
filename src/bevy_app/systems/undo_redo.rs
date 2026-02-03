//! Undo/Redo system using Command pattern for reversible user actions.
//!
//! Tracks user actions (place element, create flow, edit properties) in a history stack
//! and provides Ctrl+Z (undo) and Ctrl+Shift+Z (redo) keyboard shortcuts.
//!
//! ## Architecture
//! - `UndoCommand` trait: Defines `execute()` and `undo()` for reversible actions
//! - `CommandHistory` resource: Manages undo/redo stacks
//! - Concrete commands: `PlaceElementCommand`, `CreateFlowCommand`, `EditPropertyCommand`
//!
//! ## Usage
//! Systems that create reversible actions should:
//! 1. Create a command struct implementing `UndoCommand`
//! 2. Push to `CommandHistory` after executing
//! 3. Store necessary data to reverse the action
//!
//! ## Example
//! ```rust
//! // After placing an element:
//! let command = PlaceElementCommand {
//!     entity,
//!     element_type: PaletteElementType::Subsystem,
//!     position: Vec2::new(100.0, 100.0),
//! };
//! command_history.push(Box::new(command));
//! ```

use crate::bevy_app::components::PaletteElementType;
use bevy::ecs::message::MessageCursor;
use bevy::prelude::*;

/// Message requesting an undo operation.
#[derive(Message)]
pub struct UndoEvent;

/// Message requesting a redo operation.
#[derive(Message)]
pub struct RedoEvent;

/// Local resource to track undo message reader state between frames.
#[derive(Resource, Default)]
pub struct UndoEventReader(MessageCursor<UndoEvent>);

/// Local resource to track redo message reader state between frames.
#[derive(Resource, Default)]
pub struct RedoEventReader(MessageCursor<RedoEvent>);

/// Trait for reversible commands that can be undone and redone.
///
/// Each command stores the necessary data to both execute and reverse
/// its action. Commands are stored in CommandHistory for undo/redo.
pub trait UndoCommand: Send + Sync {
    /// Reverses the action performed by this command.
    ///
    /// Called when user presses Ctrl+Z. Should restore system state
    /// to before the command was executed.
    fn undo(&mut self, world: &mut World);

    /// Re-executes the action after it was undone.
    ///
    /// Called when user presses Ctrl+Shift+Z. Should restore system state
    /// to after the command was originally executed.
    fn redo(&mut self, world: &mut World);

    /// Returns a human-readable description of this command for debugging.
    fn description(&self) -> String;
}

/// Resource managing undo/redo command history.
///
/// Maintains two stacks:
/// - `undo_stack`: Commands that can be undone (most recent on top)
/// - `redo_stack`: Commands that can be redone (cleared on new action)
#[derive(Resource, Default)]
pub struct CommandHistory {
    /// Stack of commands that can be undone (newest first)
    undo_stack: Vec<Box<dyn UndoCommand>>,
    /// Stack of commands that can be redone (newest first)
    redo_stack: Vec<Box<dyn UndoCommand>>,
    /// Maximum number of commands to keep in history
    max_history: usize,
}

impl CommandHistory {
    /// Creates a new command history with specified maximum size.
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::with_capacity(max_history),
            redo_stack: Vec::with_capacity(max_history),
            max_history,
        }
    }

    /// Pushes a new command onto the undo stack after it's been executed.
    ///
    /// Clears the redo stack (can't redo after performing new action).
    /// Drops oldest command if history limit exceeded.
    pub fn push(&mut self, command: Box<dyn UndoCommand>) {
        // Clear redo stack on new action
        self.redo_stack.clear();

        // Add to undo stack
        self.undo_stack.push(command);

        // Enforce max history size
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }

        debug!(
            "📝 Command pushed. Undo stack: {}, Redo stack: {}",
            self.undo_stack.len(),
            self.redo_stack.len()
        );
    }

    /// Undoes the most recent command, moving it to redo stack.
    ///
    /// Returns true if a command was undone, false if undo stack empty.
    pub fn undo(&mut self, world: &mut World) -> bool {
        if let Some(mut command) = self.undo_stack.pop() {
            info!("⏪ Undoing: {}", command.description());
            command.undo(world);
            self.redo_stack.push(command);
            true
        } else {
            warn!("⚠️ Nothing to undo");
            false
        }
    }

    /// Redoes the most recently undone command, moving it back to undo stack.
    ///
    /// Returns true if a command was redone, false if redo stack empty.
    pub fn redo(&mut self, world: &mut World) -> bool {
        if let Some(mut command) = self.redo_stack.pop() {
            info!("⏩ Redoing: {}", command.description());
            command.redo(world);
            self.undo_stack.push(command);
            true
        } else {
            warn!("⚠️ Nothing to redo");
            false
        }
    }

    /// Returns true if there are commands that can be undone.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if there are commands that can be redone.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clears all command history (both undo and redo stacks).
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        info!("🗑️ Command history cleared");
    }
}

/// Command for placing an element (Subsystem, Interface, or EnvironmentalObject).
///
/// Stores entity ID and necessary data to remove/respawn the element.
pub struct PlaceElementCommand {
    /// Entity that was spawned
    pub entity: Entity,
    /// Type of element placed
    pub element_type: PaletteElementType,
    /// Position where element was placed
    pub position: Vec2,
    /// Parent system entity (for subsystems/interfaces)
    pub parent_system: Option<Entity>,
}

impl UndoCommand for PlaceElementCommand {
    fn undo(&mut self, world: &mut World) {
        // Remove the entity that was placed
        if let Ok(entity) = world.get_entity_mut(self.entity) {
            entity.despawn();
            info!("Removed {:?} at {:?}", self.element_type, self.position);
        } else {
            warn!("Entity {:?} already despawned or not found", self.entity);
        }
    }

    fn redo(&mut self, _world: &mut World) {
        // TODO: Implement redo by respawning entities
        // Challenge: spawn functions expect system context (Query, ResMut) but we have
        // exclusive world context (QueryState, Mut). Options:
        // 1. Refactor spawn functions to accept both types
        // 2. Manually recreate entities with all components here
        // 3. Store enough data to rebuild without calling spawn functions
        warn!(
            "Redo not yet fully implemented (would respawn {:?} at {:?})",
            self.element_type, self.position
        );
    }

    fn description(&self) -> String {
        format!("Place {:?} at {:?}", self.element_type, self.position)
    }
}

/// System that handles undo/redo keyboard shortcuts.
///
/// - Ctrl+Z: Undo last action
/// - Ctrl+Shift+Z: Redo last undone action
///
/// Sends UndoEvent or RedoEvent which are handled by execute_undo/execute_redo
/// systems that have exclusive world access.
pub fn handle_undo_redo_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    command_history: Res<CommandHistory>,
    mut undo_events: MessageWriter<UndoEvent>,
    mut redo_events: MessageWriter<RedoEvent>,
) {
    let ctrl = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);

    if ctrl && shift && keyboard.just_pressed(KeyCode::KeyZ) {
        // Ctrl+Shift+Z: Redo
        if command_history.can_redo() {
            info!("⏩ Sending redo event (Ctrl+Shift+Z)");
            redo_events.write(RedoEvent);
        } else {
            warn!("⚠️ Nothing to redo");
        }
    } else if ctrl && keyboard.just_pressed(KeyCode::KeyZ) {
        // Ctrl+Z: Undo
        if command_history.can_undo() {
            info!("⏪ Sending undo event (Ctrl+Z)");
            undo_events.write(UndoEvent);
        } else {
            warn!("⚠️ Nothing to undo");
        }
    }
}

/// System that executes undo operations from UndoEvent.
///
/// Runs with exclusive world access to perform entity despawning.
/// Only processes ONE undo per frame to match user expectation.
/// Uses a persistent ManualEventReader to avoid re-processing events.
pub fn execute_undo(world: &mut World) {
    world.resource_scope(|world, mut event_reader: Mut<UndoEventReader>| {
        let events = world.resource::<Messages<UndoEvent>>();

        // Read only the NEXT event (not all events)
        if let Some(_event) = event_reader.0.read(&events).next() {
            // Process exactly ONE undo
            world.resource_scope(|world, mut command_history: Mut<CommandHistory>| {
                command_history.undo(world);
            });
        }
    });
}

/// System that executes redo operations from RedoEvent.
///
/// Runs with exclusive world access to perform entity respawning.
/// Only processes ONE redo per frame to match user expectation.
/// Uses a persistent ManualEventReader to avoid re-processing events.
pub fn execute_redo(world: &mut World) {
    world.resource_scope(|world, mut event_reader: Mut<RedoEventReader>| {
        let events = world.resource::<Messages<RedoEvent>>();

        // Read only the NEXT event (not all events)
        if let Some(_event) = event_reader.0.read(&events).next() {
            // Process exactly ONE redo
            world.resource_scope(|world, mut command_history: Mut<CommandHistory>| {
                command_history.redo(world);
            });
        }
    });
}
