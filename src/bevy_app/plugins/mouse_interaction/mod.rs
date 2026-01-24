//! # Mouse Interaction Plugin
//!
//! This module provides comprehensive mouse interaction capabilities for BERT's
//! visual system modeling interface, including entity selection, dragging, and
//! hierarchical interaction management.
//!
//! ## Architecture
//!
//! The mouse interaction system integrates with Bevy's picking system to provide:
//!
//! - **Entity Selection**: Click-based selection with multi-select support
//! - **Drag and Drop**: Position manipulation of system elements
//! - **Hierarchical Picking**: Parent-child relationship aware interactions
//! - **Selection Management**: Centralized selection state and operations
//!
//! ## Interaction Flow
//!
//! ```
//! Mouse Input → Screen-to-World → Entity Picking → Selection/Drag → State Update
//!     ↓              ↓               ↓              ↓            ↓
//! Raw Events → World Position → Entity Detection → Actions → Component Updates
//! ```
//!
//! ## Key Components
//!
//! - [`MouseInteractionPlugin`]: Main plugin providing all interaction systems
//! - [`PickSelection`]: Component marking entities as selectable and tracking selection state
//! - [`DragPosition`]: Event for communicating drag operations to entity systems
//! - [`Selection`]: Global selection state resource
//! - [`Dragging`]: Drag operation state tracking
//!
//! ## Usage Patterns
//!
//! ### Basic Entity Selection
//! ```rust
//! use bert::mouse_interaction::{PickSelection, Selection};
//!
//! // Make an entity selectable
//! commands.entity(entity).insert(PickSelection::default());
//!
//! // Check if entity is selected
//! if pick_selection.is_selected {
//!     // Handle selected entity
//! }
//! ```
//!
//! ### Hierarchical Interaction
//! ```rust
//! use bert::mouse_interaction::{PickParent, PickTarget};
//!
//! // Child entity that selects its parent when clicked
//! commands.entity(child).insert(PickParent);
//!
//! // Proxy entity that selects a different target
//! commands.entity(proxy).insert(PickTarget { target: actual_entity });
//! ```
//!
//! ## Selection Behavior
//!
//! - **Single Click**: Select entity (deselect others)
//! - **Shift+Click**: Toggle entity selection (multi-select)
//! - **Escape**: Deselect all entities
//! - **Drag**: Move selected entities
//!
//! ## Integration Points
//!
//! The plugin integrates with:
//! - Bevy's picking system for entity detection
//! - Transform hierarchy for coordinate conversion
//! - Event system for drag position communication
//! - Visibility system for automatic deselection of hidden entities

#[cfg(feature = "debug_selection")]
pub mod debug;

use crate::bevy_app::components::{BoundaryRegion, EnvironmentRegion, SpatialDetailPanelMode};
use crate::bevy_app::systems::connection_mode::ConnectionMode;
use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::window::PrimaryWindow;
use bevy_picking::events::{Click, Pointer};
use bevy_picking::focus::PickingInteraction;

/// Handles spatial interaction panel switching based on which region was clicked.
///
/// This system detects clicks on BoundaryRegion and EnvironmentRegion entities and updates
/// the SpatialDetailPanelMode resource accordingly, enabling contextual property panels.
///
/// - System interior clicks → SpatialDetailPanelMode::System
/// - Boundary ring clicks → SpatialDetailPanelMode::Boundary
/// - Environment area clicks → SpatialDetailPanelMode::Environment
///
/// This implements the core spatial interaction UX where users click WHERE they want to edit.
///
/// Suppressed during connection mode to avoid panel switching during flow creation.
fn handle_spatial_panel_switching(
    mut click_events: EventReader<Pointer<Click>>,
    mut panel_mode: ResMut<SpatialDetailPanelMode>,
    connection_mode: Res<ConnectionMode>,
    boundary_query: Query<&BoundaryRegion>,
    environment_query: Query<&EnvironmentRegion>,
    system_query: Query<Entity, With<crate::bevy_app::components::System>>,
) {
    // Suppress panel switching during connection mode or just after exit (Phase 3C UX improvement)
    // - Active: User is connecting elements, not exploring spatial regions
    // - Just exited: Prevents final connection click from switching panels on same frame
    if connection_mode.active || connection_mode.just_exited {
        return;
    }

    for event in click_events.read() {
        // Check what type of entity was clicked and update panel mode accordingly
        if boundary_query.get(event.target).is_ok() {
            *panel_mode = SpatialDetailPanelMode::Boundary;
            info!("🎯 Switched to BOUNDARY panel mode");
        } else if environment_query.get(event.target).is_ok() {
            *panel_mode = SpatialDetailPanelMode::Environment;
            info!("🌍 Switched to ENVIRONMENT panel mode");
        } else if system_query.get(event.target).is_ok() {
            *panel_mode = SpatialDetailPanelMode::System;
            info!("⚙️ Switched to SYSTEM panel mode");
        }
    }
}

/// Minimum distance squared (in pixels) the mouse must move to initiate a drag operation.
///
/// This threshold prevents accidental drag operations from small mouse movements
/// during click operations. The squared distance avoids expensive square root
/// calculations during real-time interaction detection.
const DRAG_THRESHOLD_SQUARED: f32 = 4.0;

/// Main plugin providing comprehensive mouse interaction capabilities for BERT.
///
/// `MouseInteractionPlugin` integrates mouse input handling, entity selection,
/// drag operations, and selection management into a cohesive interaction system
/// that supports the visual manipulation of system models.
///
/// # System Architecture
///
/// The plugin orchestrates several interconnected systems:
///
/// 1. **Input Processing**: Converts screen coordinates to world coordinates
/// 2. **Interaction Detection**: Identifies entities under mouse cursor
/// 3. **Selection Management**: Handles single and multi-entity selection
/// 4. **Drag Operations**: Manages entity position manipulation
/// 5. **State Synchronization**: Maintains consistent selection state
///
/// # Resource Management
///
/// The plugin manages several global resources:
/// - Mouse position tracking in world coordinates
/// - Current selection set with multi-select support
/// - Drag operation state and thresholds
/// - Selection enablement for context-sensitive behavior
///
/// # Event Integration
///
/// Uses Bevy's event system for:
/// - Drag position updates for entity movement
/// - Debug selection information (when debug features enabled)
/// - Integration with other BERT systems
///
/// # Usage
///
/// Add to your Bevy app to enable mouse interactions:
/// ```rust
/// use bert::mouse_interaction::MouseInteractionPlugin;
///
/// app.add_plugins(MouseInteractionPlugin);
/// ```
///
/// # See Also
///
/// - [`PickSelection`]: Component for making entities selectable
/// - [`DragPosition`]: Event for handling drag operations
/// - [`Selection`]: Resource containing current selection state
pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    /// Configures the Bevy app with mouse interaction systems and resources.
    ///
    /// This method sets up the complete mouse interaction pipeline including:
    /// - Resource initialization for state tracking
    /// - Event registration for drag operations
    /// - System scheduling with proper ordering and conditions
    /// - Debug features when enabled
    ///
    /// # System Scheduling
    ///
    /// - **PreUpdate**: Mouse position tracking (runs before main logic)
    /// - **Update**: All interaction handling with proper dependencies
    ///   - Mouse down/up handlers run conditionally on input events
    ///   - Drag handler runs after mouse down with input conditions
    ///   - Selection management runs independently
    ///
    /// # Conditional Systems
    ///
    /// Systems use input conditions to run only when relevant:
    /// - Mouse down: Triggers on left click press
    /// - Mouse up: Triggers on left click release  
    /// - Mouse drag: Runs while left button held after mouse down
    /// - Deselect all: Triggers on Escape key press
    ///
    /// # Debug Features
    ///
    /// When `debug_selection` feature is enabled, adds additional systems
    /// for selection debugging and visualization.
    fn build(&self, app: &mut App) {
        app.init_resource::<Dragging>()
            .init_resource::<Selection>()
            .init_resource::<MouseWorldPosition>()
            .init_resource::<SelectionEnabled>()
            .init_resource::<SpatialDetailPanelMode>()
            .add_event::<DragPosition>()
            .register_type::<PickSelection>()
            .register_type::<SpatialDetailPanelMode>()
            .add_systems(PreUpdate, mouse_screen_to_world_position)
            .add_systems(
                Update,
                (
                    (
                        handle_mouse_down.run_if(input_just_pressed(MouseButton::Left)),
                        handle_mouse_up.run_if(input_just_released(MouseButton::Left)),
                        handle_mouse_drag
                            .run_if(input_pressed(MouseButton::Left))
                            .after(handle_mouse_down),
                    )
                        .in_set(MouseInteractionSet),
                    handle_spatial_panel_switching,
                    deselect_when_invisible,
                    deselect_all.run_if(input_just_pressed(KeyCode::Escape)),
                ),
            );

        #[cfg(feature = "debug_selection")]
        {
            app.init_resource::<debug::SelectedEntities>()
                .register_type::<debug::SelectedEntities>()
                .add_systems(
                    Update,
                    (debug::debug_selection, debug::debug_spatial_clicks),
                );
        }
    }
}

/// System set for organizing mouse interaction systems with proper execution order.
///
/// `MouseInteractionSet` groups all mouse interaction systems to ensure they
/// run together and can be easily scheduled relative to other system sets
/// in the BERT application.
///
/// # Usage
///
/// Systems in this set handle the complete mouse interaction pipeline and
/// should run after input processing but before rendering updates.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MouseInteractionSet;

/// Resource controlling whether entity selection is currently enabled.
///
/// `SelectionEnabled` provides a global toggle for selection functionality,
/// allowing other systems to temporarily disable selection during specific
/// operations (e.g., during flow terminal selection mode).
///
/// # Default State
///
/// Selection is enabled by default to support normal interaction patterns.
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::SelectionEnabled;
///
/// // Disable selection temporarily
/// fn disable_selection(mut selection_enabled: ResMut<SelectionEnabled>) {
///     **selection_enabled = false;
/// }
///
/// // Check if selection is enabled
/// fn check_selection(selection_enabled: Res<SelectionEnabled>) {
///     if **selection_enabled {
///         // Handle selection logic
///     }
/// }
/// ```
#[derive(Resource, Clone, PartialEq, Eq, Reflect, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub struct SelectionEnabled(bool);

impl Default for SelectionEnabled {
    fn default() -> Self {
        Self(true)
    }
}

/// Event fired when an entity is being dragged to a new position.
///
/// `DragPosition` communicates drag operations from the mouse interaction
/// system to entity-specific systems that handle position updates. The event
/// provides both local and world coordinates to support different positioning
/// schemes.
///
/// # Event Data
///
/// - **target**: The entity being dragged
/// - **local_position**: Position relative to the entity's parent
/// - **world_position**: Absolute position in world coordinates
///
/// # Usage
///
/// Listen for this event to implement custom drag behavior:
/// ```rust
/// use bert::mouse_interaction::DragPosition;
///
/// fn handle_drag(
///     mut drag_events: EventReader<DragPosition>,
///     mut transforms: Query<&mut Transform>,
/// ) {
///     for event in drag_events.read() {
///         if let Ok(mut transform) = transforms.get_mut(event.target) {
///             transform.translation = event.world_position.extend(transform.translation.z);
///         }
///     }
/// }
/// ```
///
/// # Coordinate Systems
///
/// The event provides coordinates in two systems:
/// - **Local**: Relative to parent transform (for hierarchical positioning)
/// - **World**: Absolute world coordinates (for direct positioning)
#[derive(Clone, Event)]
#[allow(dead_code)]
pub struct DragPosition {
    /// The entity being dragged.
    pub target: Entity,

    /// Position relative to the entity's parent transform.
    ///
    /// Use this for entities that should maintain their position relative
    /// to a parent system or container. The coordinates are transformed
    /// through the parent's inverse transform.
    pub local_position: Vec2,

    /// Absolute position in world coordinates.
    ///
    /// Use this for entities that should be positioned directly in world
    /// space without regard to parent transforms.
    pub world_position: Vec2,
}

/// Resource tracking the current state of drag operations.
///
/// `Dragging` maintains internal state for drag detection and processing,
/// including which entity is being hovered and whether a drag operation
/// is currently active.
///
/// # Internal State
///
/// - **hovered_entity**: Entity currently under the mouse cursor
/// - **started**: Whether a drag operation is currently active
/// - **start_pos**: Initial mouse position when drag detection began
///
/// # Drag Detection
///
/// Drag operations are detected when the mouse moves more than
/// [`DRAG_THRESHOLD_SQUARED`] pixels from the initial click position.
#[derive(Resource, Clone, PartialEq, Reflect, Debug, Default)]
pub struct Dragging {
    /// Entity currently under the mouse cursor (if any).
    hovered_entity: Option<Entity>,

    /// Whether a drag operation is currently active.
    started: bool,

    /// Initial mouse position when interaction began.
    start_pos: Vec2,
}

/// Resource containing the set of currently selected entities.
///
/// `Selection` maintains a global set of selected entities that can be
/// manipulated through mouse interactions. The selection supports both
/// single and multi-select operations.
///
/// # Selection Operations
///
/// - **Single Select**: Click an entity (clears other selections)
/// - **Multi Select**: Shift+Click to toggle entity selection
/// - **Clear All**: Escape key or click empty space
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::Selection;
///
/// fn process_selected_entities(
///     selection: Res<Selection>,
///     query: Query<&SomeComponent>,
/// ) {
///     for entity in selection.iter() {
///         if let Ok(component) = query.get(*entity) {
///             // Process selected entity
///         }
///     }
/// }
/// ```
#[derive(Resource, Clone, Deref, DerefMut, PartialEq, Eq, Reflect, Debug, Default)]
pub struct Selection(HashSet<Entity>);

/// Resource tracking the current mouse position in world coordinates.
///
/// `MouseWorldPosition` provides the mouse cursor position converted from
/// screen coordinates to world coordinates using the camera transform.
/// This enables accurate entity interaction in the 2D world space.
///
/// # Coordinate Conversion
///
/// The position is updated each frame by converting:
/// 1. Window cursor position (screen pixels)
/// 2. Through camera viewport transformation  
/// 3. To world coordinates (using camera transform)
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::MouseWorldPosition;
///
/// fn check_mouse_position(mouse_pos: Res<MouseWorldPosition>) {
///     let world_pos: Vec2 = **mouse_pos;
///     println!("Mouse at world position: {:?}", world_pos);
/// }
/// ```
#[derive(Resource, Clone, Deref, DerefMut, PartialEq, Reflect, Debug, Default)]
pub struct MouseWorldPosition(Vec2);

/// Component marking an entity as selectable and tracking its selection state.
///
/// `PickSelection` enables entities to participate in the mouse selection system.
/// Entities with this component can be clicked to select them, participate in
/// multi-select operations, and have their selection state queried by other systems.
///
/// # Selection State
///
/// The component tracks whether the entity is currently selected, allowing
/// systems to query selection status and update visual representation accordingly.
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::PickSelection;
///
/// // Make an entity selectable
/// commands.entity(entity).insert(PickSelection::default());
///
/// // Check selection state
/// fn update_selected_visuals(
///     query: Query<(&PickSelection, &mut Visibility)>,
/// ) {
///     for (selection, mut visibility) in &mut query {
///         if selection.is_selected {
///             // Update visual appearance for selected state
///         }
///     }
/// }
/// ```
///
/// # Integration
///
/// Works with the mouse interaction systems to:
/// - Detect mouse clicks on the entity
/// - Update selection state based on interaction mode
/// - Support multi-select with Shift+Click
/// - Clear selection on Escape or empty space clicks
///
/// # See Also
///
/// - [`Selection`]: Global resource containing all selected entities
/// - [`NoDeselect`]: Component to prevent deselection of specific entities
#[derive(Component, Default, Copy, Clone, Debug, Reflect, PartialEq, Eq)]
#[reflect(Component)]
pub struct PickSelection {
    /// Whether this entity is currently selected.
    ///
    /// Updated automatically by the mouse interaction systems based on
    /// user input and selection mode (single vs. multi-select).
    pub is_selected: bool,
}

/// Marker component preventing an entity from being deselected through normal means.
///
/// `NoDeselect` protects entities from being deselected when the user clicks
/// empty space or presses Escape. This is useful for entities that should
/// maintain their selection state during specific operations.
///
/// # Use Cases
///
/// - UI elements that should remain selected during interaction
/// - Entities involved in ongoing operations (e.g., flow creation)
/// - System elements that need persistent selection for contextual operations
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::{PickSelection, NoDeselect};
///
/// // Create an entity that cannot be deselected
/// commands.entity(entity)
///     .insert(PickSelection { is_selected: true })
///     .insert(NoDeselect);
/// ```
///
/// # Behavior
///
/// Entities with `NoDeselect`:
/// - Can still be manually deselected by clicking them again (if multi-select)
/// - Are protected from global deselection operations
/// - Maintain selection state during mode changes
#[derive(Component, Copy, Clone, PartialEq, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct NoDeselect;

/// Marker component indicating that clicks on this entity should select its parent.
///
/// `PickParent` enables hierarchical selection where child entities can be used
/// to select their parent entities. This is useful for complex entities where
/// the visual representation consists of multiple parts, but the logical
/// selection target is the parent entity.
///
/// # Hierarchical Selection
///
/// When an entity with `PickParent` is clicked:
/// 1. The mouse interaction system identifies the clicked entity
/// 2. Looks up the entity's parent using Bevy's hierarchy
/// 3. Applies selection operations to the parent entity instead
///
/// # Use Cases
///
/// - Visual components of complex system entities
/// - UI elements that should select their containing system
/// - Decoration or annotation entities that belong to larger entities
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::{PickSelection, PickParent};
///
/// // Create a parent-child relationship with hierarchical selection
/// let parent = commands.spawn(PickSelection::default()).id();
/// let child = commands.spawn(PickParent)
///     .set_parent(parent)
///     .id();
///
/// // Clicking the child will select the parent
/// ```
///
/// # Requirements
///
/// Entities with `PickParent` must have a parent in Bevy's hierarchy system.
/// The mouse interaction system expects to find a valid parent entity.
///
/// # See Also
///
/// - [`PickTarget`]: For selecting a specific target entity (not necessarily parent)
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct PickParent;

/// Component redirecting selection to a specific target entity.
///
/// `PickTarget` enables proxy selection where clicking one entity results in
/// selecting a different, explicitly specified target entity. This provides
/// more flexibility than `PickParent` by allowing arbitrary selection targets.
///
/// # Proxy Selection
///
/// When an entity with `PickTarget` is clicked:
/// 1. The mouse interaction system identifies the clicked entity
/// 2. Uses the specified target entity for selection operations
/// 3. Applies selection state to the target entity
///
/// # Use Cases
///
/// - UI buttons that select specific system entities
/// - Visual proxies for entities that are not directly clickable
/// - Interface elements that represent remote or hidden entities
/// - Delegation patterns where multiple clickable areas select the same target
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::{PickSelection, PickTarget};
///
/// // Create a target entity
/// let target = commands.spawn(PickSelection::default()).id();
///
/// // Create a proxy that selects the target when clicked
/// let proxy = commands.spawn(PickTarget { target }).id();
///
/// // Clicking the proxy will select the target entity
/// ```
///
/// # Target Validation
///
/// The target entity should exist and typically should have a `PickSelection`
/// component to participate in the selection system.
///
/// # See Also
///
/// - [`PickParent`]: For selecting the parent entity in hierarchy
/// - [`PickSelection`]: Component required on target entities for selection
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect, Debug)]
#[reflect(Component)]
pub struct PickTarget {
    /// The entity that should be selected when this entity is clicked.
    ///
    /// This entity will receive the selection operation instead of the
    /// entity that was actually clicked by the user.
    pub target: Entity,
}

fn handle_mouse_down(
    interaction_query: Query<(
        Entity,
        &PickingInteraction,
        &GlobalTransform,
        Option<&PickParent>,
        Option<&PickTarget>,
    )>,
    parent_query: Query<&Parent>,
    mouse_position: Res<MouseWorldPosition>,
    mut dragging: ResMut<Dragging>,
) {
    dragging.hovered_entity = None;
    dragging.started = false;
    dragging.start_pos = **mouse_position;

    // Collect all interacting entities with their z-values
    // Higher z = closer to camera = should be selected first
    let mut candidates: Vec<(Entity, f32, Option<&PickParent>, Option<&PickTarget>)> = Vec::new();

    for (entity, interaction, global_transform, pick_parent, pick_target) in &interaction_query {
        if !matches!(interaction, PickingInteraction::None) {
            let z = global_transform.translation().z;
            candidates.push((entity, z, pick_parent, pick_target));
        }
    }

    // Sort by z descending (highest z first = closest to camera)
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Select the topmost entity
    if let Some((entity, _, pick_parent, pick_target)) = candidates.first() {
        if pick_parent.is_some() {
            dragging.hovered_entity = Some(
                parent_query
                    .get(*entity)
                    .expect("Parent should exist for components that have PickParent")
                    .get(),
            );
        } else if let Some(target) = pick_target {
            dragging.hovered_entity = Some(target.target);
        } else {
            dragging.hovered_entity = Some(*entity);
        }
    }
}

fn handle_mouse_up(
    interaction_query: Query<(Entity, Option<&NoDeselect>)>,
    mut pick_selection_query: Query<&mut PickSelection>,
    mut dragging: ResMut<Dragging>,
    mut selection: ResMut<Selection>,
    selection_enabled: Res<SelectionEnabled>,
    connection_mode: Res<ConnectionMode>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if dragging.started {
        dragging.started = false;
        dragging.hovered_entity = None;
        return;
    }

    let multi_select = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    // Suppress selection when connection mode is active or just exited (Phase 3C UX improvement)
    // - Active: Users are connecting elements, not inspecting properties
    // - Just exited: Prevents final connection click from opening panel on same frame
    // Connection mode handles clicks for source/destination selection, not entity selection
    if **selection_enabled && !connection_mode.active && !connection_mode.just_exited {
        selection.clear();

        let mut deselect = true;

        if let Some(hovered_entity) = dragging.hovered_entity {
            if let Ok((entity, no_deselect)) = interaction_query.get(hovered_entity) {
                if no_deselect.is_some() {
                    deselect = false;
                } else {
                    selection.insert(entity);
                }
            } else {
                deselect = false;
            }
        }

        if deselect && !multi_select {
            do_deselect_all(&mut pick_selection_query);
        }

        if !selection.is_empty() {
            for entity in &selection.0 {
                if let Ok(mut pick_selection) = pick_selection_query.get_mut(*entity) {
                    if multi_select {
                        pick_selection.is_selected = !pick_selection.is_selected;
                    } else {
                        pick_selection.is_selected = true;
                    }
                }
            }
        }
    }

    dragging.hovered_entity = None;
}

fn handle_mouse_drag(
    mouse_position: Res<MouseWorldPosition>,
    mut commands: Commands,
    mut dragging: ResMut<Dragging>,
    transform_query: Query<&GlobalTransform>,
    parent_query: Query<&Parent>,
) {
    let mouse_position = **mouse_position;

    if !dragging.started
        && mouse_position.distance_squared(dragging.start_pos) > DRAG_THRESHOLD_SQUARED
    {
        dragging.started = true;
    }

    if dragging.started {
        if let Some(entity) = dragging.hovered_entity {
            let position = if let Ok(parent) = parent_query.get(entity) {
                let parent_transform = transform_query
                    .get(parent.get())
                    .expect("Parent should have a Transform");
                parent_transform
                    .affine()
                    .inverse()
                    .transform_point3(mouse_position.extend(0.0))
                    .truncate()
            } else {
                mouse_position
            };

            commands.trigger_targets(
                DragPosition {
                    target: entity,
                    local_position: position,
                    world_position: mouse_position,
                },
                entity,
            );
        }
    }
}

fn mouse_screen_to_world_position(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut mouse_world_position: ResMut<MouseWorldPosition>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(world_position) = window.cursor_position().and_then(|window_position| {
        camera
            .viewport_to_world_2d(camera_transform, window_position)
            .ok()
    }) {
        **mouse_world_position = world_position;
    }
}

/// System function for deselecting all currently selected entities.
///
/// `deselect_all` clears the selection state of all entities with `PickSelection`
/// components, typically triggered by the Escape key or other global deselection
/// events.
///
/// # Usage
///
/// This function is automatically called by the mouse interaction systems, but
/// can also be used manually:
/// ```rust
/// use bert::mouse_interaction::deselect_all;
///
/// app.add_systems(Update, deselect_all.run_if(some_condition));
/// ```
///
/// # Parameters
///
/// - `query`: Mutable query for all entities with `PickSelection` components
///
/// # Returns
///
/// This function returns `()` (unit type) and operates through side effects.
///
/// # Errors
///
/// This function does not return errors.
///
/// # Panics
///
/// Does not panic under normal operation.
///
/// # See Also
///
/// - [`do_deselect_all`]: The underlying implementation function
/// - [`SelectionEnabled`]: Resource that can disable this functionality
pub fn deselect_all(mut query: Query<&mut PickSelection>) {
    do_deselect_all(&mut query);
}

/// Internal helper function implementing the deselection logic.
///
/// `do_deselect_all` is the core implementation for clearing selection state
/// across all entities. It's separated from the system function to allow
/// reuse in other contexts without the query parameter constraints.
///
/// # Parameters
///
/// - `pick_selection_query`: Mutable query for entities with `PickSelection`
///
/// # Returns
///
/// This function returns `()` (unit type) and operates through side effects.
///
/// # Errors
///
/// This function does not return errors.
///
/// # Panics
///
/// Does not panic under normal operation.
///
/// # Implementation
///
/// Iterates through all entities with `PickSelection` components and sets
/// their `is_selected` field to `false`. This operation respects the
/// `NoDeselect` component by only operating on the provided query.
pub fn do_deselect_all(pick_selection_query: &mut Query<&mut PickSelection>) {
    for mut pick_selection in pick_selection_query {
        pick_selection.is_selected = false;
    }
}

/// System function to disable mouse selection functionality.
///
/// `disable_selection` sets the global `SelectionEnabled` resource to `false`,
/// preventing any selection operations from occurring. This is useful during
/// modes where selection should be temporarily disabled.
///
/// # Use Cases
///
/// - Flow terminal selection mode
/// - Modal dialog interactions
/// - Drawing or annotation modes
/// - Any context where selection would interfere with other operations
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::disable_selection;
///
/// // Disable selection during a specific mode
/// app.add_systems(OnEnter(DrawingMode), disable_selection);
/// ```
///
/// # Parameters
///
/// - `selection_enabled`: Mutable resource for controlling selection state
///
/// # Returns
///
/// This function returns `()` (unit type) and operates through side effects.
///
/// # Errors
///
/// This function does not return errors.
///
/// # Panics
///
/// Does not panic under normal operation.
///
/// # See Also
///
/// - [`enable_selection`]: Function to re-enable selection
/// - [`SelectionEnabled`]: The resource being modified
pub fn disable_selection(mut selection_enabled: ResMut<SelectionEnabled>) {
    **selection_enabled = false;
}

/// System function to enable mouse selection functionality and reset drag state.
///
/// `enable_selection` sets the global `SelectionEnabled` resource to `true`
/// and clears any ongoing drag operations. This ensures a clean state when
/// re-enabling selection after it was disabled.
///
/// # State Reset
///
/// In addition to enabling selection, this function:
/// - Clears any hovered entity state
/// - Resets drag operation flags
/// - Ensures clean interaction state
///
/// # Usage
///
/// ```rust
/// use bert::mouse_interaction::enable_selection;
///
/// // Re-enable selection when exiting a special mode
/// app.add_systems(OnExit(DrawingMode), enable_selection);
/// ```
///
/// # Parameters
///
/// - `selection_enabled`: Mutable resource for controlling selection state
/// - `dragging`: Mutable resource for drag operation state (reset to default)
///
/// # Returns
///
/// This function returns `()` (unit type) and operates through side effects.
///
/// # Errors
///
/// This function does not return errors.
///
/// # Panics
///
/// Does not panic under normal operation.
///
/// # See Also
///
/// - [`disable_selection`]: Function to disable selection
/// - [`SelectionEnabled`]: The resource being modified
/// - [`Dragging`]: The resource being reset
pub fn enable_selection(
    mut selection_enabled: ResMut<SelectionEnabled>,
    mut dragging: ResMut<Dragging>,
) {
    **selection_enabled = true;

    dragging.hovered_entity = None;
    dragging.started = false;
}

/// System to automatically deselect entities when they become invisible.
///
/// `deselect_when_invisible` monitors visibility changes and automatically
/// deselects entities that become invisible. This prevents having selected
/// entities that the user cannot see, which would be confusing.
///
/// # Visibility Integration
///
/// Uses Bevy's `InheritedVisibility` component to detect when entities
/// become invisible through:
/// - Direct visibility changes
/// - Parent visibility changes (inheritance)
/// - Layer or rendering visibility changes
///
/// # Change Detection
///
/// The system uses Bevy's change detection (`Changed<InheritedVisibility>`)
/// to efficiently process only entities whose visibility has changed,
/// minimizing performance impact.
///
/// # Usage
///
/// This system runs automatically as part of the mouse interaction plugin.
/// No manual configuration is required.
///
/// # Parameters
///
/// - `selection_query`: Query for entities with both `PickSelection` and
///   `InheritedVisibility`, filtered to only changed visibility
///
/// # Returns
///
/// This function returns `()` (unit type) and operates through side effects.
///
/// # Errors
///
/// This function does not return errors.
///
/// # Panics
///
/// Does not panic under normal operation.
///
/// # Behavior
///
/// When an entity's visibility changes to invisible (`!visibility.get()`),
/// its selection state is cleared. Entities becoming visible are not
/// automatically selected.
pub fn deselect_when_invisible(
    mut selection_query: Query<
        (&mut PickSelection, &InheritedVisibility),
        Changed<InheritedVisibility>,
    >,
) {
    for (mut pick_selection, visibility) in &mut selection_query {
        if !visibility.get() {
            pick_selection.is_selected = false;
        }
    }
}
