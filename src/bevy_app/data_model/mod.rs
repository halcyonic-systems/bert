//! Thin app-side shim over the extracted kernel crate [`bert_core`].
//!
//! The model contract (types + validation) lives in `bert-core`; this module
//! keeps the historical `bevy_app::data_model::*` paths working, hosts the
//! Bevy-coupled save/load mappers, and carries the impls that bridge kernel
//! types to runtime components.

pub mod complexity_calculator;
pub mod load;
pub mod save;

pub use bert_core::validate;
pub use bert_core::*;
// Explicit re-exports win over glob imports: these kernel names collide with
// `components::*` / `bevy::prelude::*` inside this module and its children.
pub use bert_core::{ExternalEntity, Interaction, Interface, InterfaceType, Parameter, System};

use crate::bevy_app::components::*;
use bevy::prelude::*;

impl From<crate::bevy_app::components::InterfaceType> for InterfaceType {
    fn from(ty: crate::bevy_app::components::InterfaceType) -> Self {
        match ty {
            crate::bevy_app::components::InterfaceType::Export => Self::Export,
            crate::bevy_app::components::InterfaceType::Import => Self::Import,
        }
    }
}


/// Bevy-coupled placement helper for the kernel's [`Transform2d`] — an
/// extension trait because inherent impls can't live outside bert-core.
pub trait Transform2dExt {
    fn as_components(&self, z: f32, zoom: f32) -> (Transform, InitialPosition);
    /// Replaces the old `From<(&Transform, &InitialPosition)>` impl — tuples
    /// are not fundamental, so the orphan rule rejects it now that
    /// `Transform2d` lives in bert-core.
    fn from_components(t: &Transform, ip: &InitialPosition) -> Transform2d;
}

impl Transform2dExt for Transform2d {
    fn from_components(t: &Transform, ip: &InitialPosition) -> Transform2d {
        Transform2d {
            translation: **ip,
            rotation: t.right().truncate().to_angle(),
        }
    }

    fn as_components(&self, z: f32, zoom: f32) -> (Transform, InitialPosition) {
        (
            Transform::from_translation((self.translation * zoom).extend(z))
                .with_rotation(Quat::from_rotation_z(self.rotation)),
            InitialPosition::new(self.translation),
        )
    }
}
