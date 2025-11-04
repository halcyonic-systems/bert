mod save_notifications;
mod system_element_geometry;

pub use save_notifications::*;
pub use system_element_geometry::*;

use bevy::prelude::*;

#[derive(Debug, Resource, Deref, DerefMut, Copy, Clone, Reflect)]
#[reflect(Resource)]
pub struct FocusedSystem(Entity);

impl FocusedSystem {
    pub fn new(entity: Entity) -> Self {
        Self(entity)
    }
}

impl Default for FocusedSystem {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

#[derive(Debug, Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct Zoom(f32);

impl Zoom {
    pub fn mul(&mut self, fac: f32) {
        debug_assert!(fac > 0.0);
        self.0 *= fac;
    }
}

impl Default for Zoom {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Phase 3B: Animation target for smooth zoom transitions on focus change.
///
/// When user focuses a nested subsystem, this resource stores the target zoom level
/// and camera position for smooth animation over ~300ms.
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct ZoomTarget {
    /// Target zoom level to animate toward
    pub target_zoom: f32,
    /// Target camera pan position (world coordinates)
    pub target_pan: Vec2,
    /// Whether animation is currently active
    pub animating: bool,
    /// Animation progress (0.0 = start, 1.0 = complete)
    pub progress: f32,
}

impl Default for ZoomTarget {
    fn default() -> Self {
        Self {
            target_zoom: 1.0,
            target_pan: Vec2::ZERO,
            animating: false,
            progress: 0.0,
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct StrokeTessellator(bevy_prototype_lyon::prelude::tess::StrokeTessellator);

impl StrokeTessellator {
    pub fn new() -> Self {
        StrokeTessellator(bevy_prototype_lyon::prelude::tess::StrokeTessellator::new())
    }
}

#[derive(Resource, Deref, DerefMut, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct CurrentFile(pub Option<String>);

/// Resource controlling the background color of the application.
///
/// Simple toggle between the original BERT beige background and clean white background
/// for screenshots and documentation. All other visual elements remain unchanged.
#[derive(Debug, Resource, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Resource)]
pub enum Theme {
    /// Original BERT background (beige)
    Normal,
    /// White background for clean screenshots
    White,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Normal
    }
}

impl Theme {
    /// Toggle between normal beige and white background
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Normal => Theme::White,
            Theme::White => Theme::Normal,
        }
    }

    /// Check if the current background is white
    pub fn is_white(&self) -> bool {
        matches!(self, Theme::White)
    }

    /// Check if the current background is normal (beige)
    pub fn is_normal(&self) -> bool {
        matches!(self, Theme::Normal)
    }
}
