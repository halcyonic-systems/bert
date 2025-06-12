//! # Label System Plugin
//!
//! This module provides comprehensive text labeling capabilities for BERT's visual
//! system modeling interface, including dynamic text positioning, background management,
//! and automatic contrast adjustment for optimal readability.
//!
//! ## Architecture Overview
//!
//! The label system implements a flexible text overlay architecture with:
//!
//! - **Dynamic Positioning**: Labels automatically follow parent entities
//! - **Smart Alignment**: Automatic text alignment based on position context
//! - **Background Management**: Optional backgrounds with padding and contrast
//! - **Interactive Integration**: Labels can proxy clicks to their parent entities
//!
//! ## Label Composition
//!
//! Each label consists of multiple coordinated entities:
//!
//! ```
//! Parent Entity (System/Interface/etc.)
//!     ↓
//! Label Container (Sprite + PickTarget)
//!     ↓
//! Text Entity (Text2D + Styling)
//! ```
//!
//! ## Key Components
//!
//! ### Text Components
//! - [`NameLabel`]: Links entities to their name-based labels
//! - [`MarkerLabel`]: Links entities to custom marker labels
//! - [`AutoContrastTextColor`]: Automatic color adjustment for readability
//!
//! ### Positioning Components
//! - [`CopyPosition`]: Single position synchronization configuration
//! - [`CopyPositions`]: Multiple position synchronization for complex layouts
//! - [`Alignment`]: Text alignment options (Center, Auto, AutoStartEnd)
//!
//! ### Visual Components
//! - [`Background`]: Background sizing and padding configuration
//! - [`LabelContainer`]: Marker for label container entities
//!
//! ## Positioning System
//!
//! Labels use a sophisticated positioning system with:
//!
//! ### Anchor Points
//! - **Horizontal**: Center, East/West (Local/World)
//! - **Vertical**: Center, North/South (Local/World)
//!
//! ### Alignment Modes
//! - **Center**: Fixed center alignment
//! - **Auto**: Automatic alignment based on position context
//! - **AutoStartEnd**: Automatic start/end alignment without center
//!
//! ## Usage Patterns
//!
//! ### Basic Name Label
//! ```rust
//! use bert::label::{add_name_label, BackgroundArgs};
//!
//! add_name_label(
//!     &mut commands,
//!     entity,
//!     Vec2::new(100.0, 30.0), // text bounds
//!     Some(BackgroundArgs::default()), // background
//!     None, // positioning
//!     false, // multiple positions
//!     &name_query,
//!     &asset_server,
//!     None, // text color
//!     (), // additional bundle
//! );
//! ```
//!
//! ### Marker with Positioning
//! ```rust
//! use bert::label::{add_marker_with_text, CopyPositionArgs};
//!
//! add_marker_with_text(
//!     &mut commands,
//!     entity,
//!     &mut copy_positions,
//!     &entity_aabb,
//!     Vec2::new(20.0, 20.0), // sprite size
//!     Some(CopyPositionArgs {
//!         offset: Vec3::new(0.0, 25.0, 0.0),
//!         horizontal_alignment: Alignment::Center,
//!         vertical_alignment: Alignment::Auto,
//!         horizontal_anchor: HorizontalAttachmentAnchor::Center,
//!         vertical_anchor: VerticalAttachmentAnchor::NorthLocal,
//!     }),
//!     "Marker Text",
//!     "icons/marker.png",
//!     &asset_server,
//!     None,
//!     (),
//! );
//! ```
//!
//! ## System Integration
//!
//! The plugin integrates with several BERT systems:
//! - **Mouse Interaction**: Labels proxy clicks to parent entities via [`PickTarget`]
//! - **Visual Rendering**: Manages z-ordering and layering with [`LABEL_Z`]
//! - **Entity Hierarchy**: Maintains parent-child relationships for positioning
//! - **Text Rendering**: Leverages Bevy's text system with custom styling
//!
//! ## Performance Features
//!
//! - **Change Detection**: Updates only when parent entities or text properties change
//! - **Efficient Queries**: Targeted component queries minimize processing overhead
//! - **Batch Operations**: Multiple labels can share positioning configurations
//! - **Smart Alignment**: Automatic alignment reduces manual positioning needs
//!
//! ## See Also
//!
//! - [`copy_position`]: Position synchronization sub-module
//! - [`text`]: Text rendering and styling sub-module
//! - [`crate::bevy_app::plugins::mouse_interaction`]: Mouse interaction integration

use crate::bevy_app::plugins::mouse_interaction::PickTarget;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy::text::{update_text2d_layout, TextBounds};

mod copy_position;
mod text;

use crate::bevy_app::constants::LABEL_Z;
pub use copy_position::*;
pub use text::*;

/// Main plugin providing comprehensive text labeling capabilities for BERT.
///
/// `LabelPlugin` orchestrates text rendering, positioning, and styling systems
/// to provide dynamic, context-aware labels that automatically follow and
/// enhance system entities with readable text overlays.
///
/// # System Architecture
///
/// The plugin manages several coordinated systems:
///
/// 1. **Position Synchronization**: Keeps labels aligned with parent entities
/// 2. **Text Updates**: Synchronizes label text with entity names and properties
/// 3. **Visual Styling**: Applies automatic contrast and background management
/// 4. **Layout Management**: Handles text bounds and background sizing
///
/// # System Scheduling
///
/// All label systems run in PostUpdate to ensure:
/// - Parent entity transforms are finalized
/// - Text layout calculations are complete
/// - Visual updates occur after all logic processing
/// - Proper z-ordering and layering is maintained
///
/// # Registration
///
/// The plugin registers all label-related component types for:
/// - Bevy Inspector compatibility
/// - Serialization support
/// - Debug visualization
/// - Runtime type information
///
/// # Usage
///
/// Add to your Bevy app to enable label functionality:
/// ```rust
/// use bert::label::LabelPlugin;
/// 
/// app.add_plugins(LabelPlugin);
/// ```
///
/// # See Also
///
/// - [`add_name_label`]: Function for creating name-based labels
/// - [`add_marker_with_text`]: Function for creating marker labels
/// - [`CopyPosition`]: Component for position synchronization
pub struct LabelPlugin;

impl Plugin for LabelPlugin {
    /// Configures the Bevy app with label systems and component registration.
    ///
    /// This method sets up the complete label management pipeline including:
    /// - Position synchronization systems with change detection
    /// - Text content and styling update systems
    /// - Background management and automatic sizing
    /// - Component type registration for tooling support
    ///
    /// # System Scheduling
    ///
    /// All systems run in **PostUpdate** to ensure proper execution order:
    /// - Parent transforms are finalized before position copying
    /// - Text layout is calculated before background sizing
    /// - All entity updates are complete before visual processing
    ///
    /// # System Dependencies
    ///
    /// The systems are organized with explicit dependencies:
    /// - `update_background_size_from_label` runs after `update_text2d_layout`
    ///   to ensure text dimensions are available for background sizing
    /// - Position systems run independently for parallel execution
    /// - Text update systems use change detection for efficiency
    ///
    /// # Component Registration
    ///
    /// Registers all label-related components for:
    /// - **Inspector**: Runtime debugging and visualization
    /// - **Serialization**: Save/load support for label configurations
    /// - **Reflection**: Dynamic component access and manipulation
    /// - **Type Safety**: Compile-time guarantees for component operations
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                copy_position,
                copy_positions,
                copy_positions_changed,
                compute_text_alignment,
                compute_text_alignments,
                copy_name_to_label,
                apply_text_color_contrast,
                update_background_size_from_label.after(update_text2d_layout),
            ),
        )
        .register_type::<NameLabel>()
        .register_type::<CopyPosition>()
        .register_type::<CopyPositions>()
        .register_type::<AutoContrastTextColor>()
        .register_type::<LabelContainer>()
        .register_type::<Background>();
    }
}

/// Configuration arguments for label positioning relative to parent entities.
///
/// `CopyPositionArgs` provides a comprehensive set of parameters for controlling
/// how labels are positioned relative to their parent entities, including offset,
/// alignment, and anchor point specifications.
///
/// # Positioning Components
///
/// - **Offset**: 3D offset from the calculated anchor point
/// - **Alignment**: How text is aligned within its container
/// - **Anchors**: Where the label attaches to the parent entity
///
/// # Usage
///
/// ```rust
/// use bert::label::{CopyPositionArgs, Alignment, HorizontalAttachmentAnchor, VerticalAttachmentAnchor};
///
/// let positioning = CopyPositionArgs {
///     offset: Vec3::new(10.0, 20.0, 0.0),
///     horizontal_alignment: Alignment::Center,
///     vertical_alignment: Alignment::Auto,
///     horizontal_anchor: HorizontalAttachmentAnchor::EastLocal,
///     vertical_anchor: VerticalAttachmentAnchor::NorthLocal,
/// };
/// ```
///
/// # Coordinate Systems
///
/// - **Local**: Relative to the parent entity's local coordinate system
/// - **World**: Relative to the world coordinate system
/// - **Auto**: Automatically determined based on context
///
/// # See Also
///
/// - [`CopyPosition`]: The component that uses these arguments
/// - [`Alignment`]: Text alignment options
/// - [`HorizontalAttachmentAnchor`]: Horizontal anchor point options
/// - [`VerticalAttachmentAnchor`]: Vertical anchor point options
pub struct CopyPositionArgs {
    /// 3D offset from the calculated anchor point.
    ///
    /// The offset is applied after anchor point calculation and can be
    /// in local or world coordinates depending on the `local_offset` setting.
    pub offset: Vec3,
    
    /// Horizontal text alignment within the label container.
    ///
    /// Controls how text is aligned horizontally when the text is smaller
    /// than its container bounds.
    pub horizontal_alignment: Alignment,
    
    /// Vertical text alignment within the label container.
    ///
    /// Controls how text is aligned vertically when the text is smaller
    /// than its container bounds.
    pub vertical_alignment: Alignment,
    
    /// Horizontal anchor point on the parent entity.
    ///
    /// Determines which point on the parent entity the label should
    /// attach to horizontally.
    pub horizontal_anchor: HorizontalAttachmentAnchor,
    
    /// Vertical anchor point on the parent entity.
    ///
    /// Determines which point on the parent entity the label should
    /// attach to vertically.
    pub vertical_anchor: VerticalAttachmentAnchor,
}

/// Configuration arguments for label background styling and sizing.
///
/// `BackgroundArgs` provides parameters for creating visual backgrounds
/// behind label text to improve readability and visual separation from
/// the underlying system elements.
///
/// # Visual Properties
///
/// - **Color**: Background color with alpha support for transparency
/// - **Padding**: Horizontal and vertical padding around the text
///
/// # Default Styling
///
/// The default background provides a clean, readable appearance:
/// - White background color
/// - 7 pixels horizontal padding
/// - 3 pixels vertical padding
///
/// # Usage
///
/// ```rust
/// use bert::label::BackgroundArgs;
///
/// // Use default styling
/// let background = BackgroundArgs::default();
///
/// // Custom styling
/// let custom_background = BackgroundArgs {
///     color: Color::rgba(0.9, 0.9, 0.9, 0.8), // Semi-transparent gray
///     padding_horizontal: 10.0,
///     padding_vertical: 5.0,
/// };
/// ```
///
/// # Color Considerations
///
/// - Use sufficient contrast with text color for readability
/// - Consider alpha values for visual layering
/// - Background color interacts with automatic text contrast systems
///
/// # Sizing Behavior
///
/// Background size is automatically calculated based on:
/// - Text layout dimensions
/// - Specified padding values
/// - Text bounds and line wrapping
///
/// # See Also
///
/// - [`Background`]: The component that uses these arguments
/// - [`AutoContrastTextColor`]: Automatic text color adjustment
/// - [`update_background_size_from_label`]: System that handles sizing
pub struct BackgroundArgs {
    /// Background color with alpha support.
    ///
    /// The color is used for the sprite background behind the label text.
    /// Alpha values less than 1.0 create semi-transparent backgrounds.
    pub color: Color,
    
    /// Horizontal padding around the text in pixels.
    ///
    /// This padding is applied to both left and right sides of the text,
    /// so the total horizontal expansion is `2 * padding_horizontal`.
    pub padding_horizontal: f32,
    
    /// Vertical padding around the text in pixels.
    ///
    /// This padding is applied to both top and bottom of the text,
    /// so the total vertical expansion is `2 * padding_vertical`.
    pub padding_vertical: f32,
}

impl Default for BackgroundArgs {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            padding_horizontal: 7.0,
            padding_vertical: 3.0,
        }
    }
}

pub fn add_name_label<B: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    text_bounds_size: Vec2,
    background: Option<BackgroundArgs>,
    copy_position: Option<CopyPositionArgs>,
    multiple_copy_positions: bool,
    name_query: &Query<&Name>,
    asset_server: &Res<AssetServer>,
    text_color: Option<AutoContrastTextColor>,
    additional_bundle: B,
) {
    let mut text_commands = commands.spawn((
        Text2d::new(
            &name_query
                .get(entity)
                .expect("Entity should have a name")
                .to_string(),
        ),
        TextFont {
            font: asset_server
                .load("fonts/Fira_Sans/FiraSans-Bold.ttf")
                .clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor::BLACK,
        TextLayout {
            justify: JustifyText::Left,
            linebreak: LineBreak::WordBoundary,
        },
        TextBounds::from(text_bounds_size),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Name::new("Label Text"),
    ));

    if let Some(text_color) = text_color {
        text_commands.insert(text_color);
    }

    let background_color = if let Some(background) = background {
        text_commands.insert(Background {
            padding_horizontal: background.padding_horizontal,
            padding_vertical: background.padding_vertical,
        });

        background.color
    } else {
        Color::NONE
    };

    let text_entity = text_commands.id();

    let sprite_entity = commands
        .spawn((
            Sprite {
                color: background_color,
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, LABEL_Z),
            Name::new("Label Sprite"),
            PickTarget { target: entity },
            additional_bundle,
            LabelContainer,
        ))
        .add_children(&[text_entity])
        .id();

    let mut entity_commands = commands.entity(entity);

    entity_commands.insert((NameLabel { label: text_entity },));

    if let Some(CopyPositionArgs {
        offset,
        horizontal_alignment,
        vertical_alignment,
        vertical_anchor,
        horizontal_anchor,
    }) = copy_position
    {
        let copy_position = CopyPosition {
            target: sprite_entity,
            aabb: None,
            offset,
            local_offset: true,
            horizontal_alignment,
            vertical_alignment,
            horizontal_anchor,
            vertical_anchor,
        };

        if multiple_copy_positions {
            entity_commands.insert(CopyPositions(vec![copy_position]));
        } else {
            entity_commands.insert(copy_position);
        }
    }
}

pub fn add_marker_with_text<B: Bundle>(
    commands: &mut Commands,
    entity: Entity,
    copy_positions: &mut CopyPositions,
    aabb: &Aabb,
    sprite_size: Vec2,
    copy_position: Option<CopyPositionArgs>,
    text: &str,
    asset_path: &str,
    asset_server: &Res<AssetServer>,
    text_color: Option<AutoContrastTextColor>,
    additional_bundle: B,
) {
    let mut text_commands = commands.spawn((
        Text2d::new(text),
        TextFont {
            font: asset_server
                .load("fonts/Fira_Sans/FiraSans-Bold.ttf")
                .clone(),
            font_size: 16.0,
            ..default()
        },
        TextColor::BLACK,
        TextLayout {
            justify: JustifyText::Center,
            linebreak: LineBreak::WordBoundary,
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Name::new("Label Text"),
    ));

    if let Some(text_color) = text_color {
        text_commands.insert(text_color);
    }

    let text_entity = text_commands.id();

    let sprite_entity = commands
        .spawn((
            Sprite {
                image: asset_server.load(asset_path),
                custom_size: Some(sprite_size),
                ..default()
            },
            Name::new("Named Indicator"),
            PickTarget { target: entity },
            additional_bundle,
            LabelContainer,
        ))
        .add_children(&[text_entity])
        .id();

    let mut entity_commands = commands.entity(entity);

    entity_commands.insert((MarkerLabel { label: text_entity },));

    if let Some(CopyPositionArgs {
        offset,
        horizontal_alignment,
        vertical_alignment,
        vertical_anchor,
        horizontal_anchor,
    }) = copy_position
    {
        copy_positions.0.push(CopyPosition {
            target: sprite_entity,
            aabb: Some(*aabb),
            offset,
            local_offset: true,
            horizontal_alignment,
            vertical_alignment,
            horizontal_anchor,
            vertical_anchor,
        });
    }
}
