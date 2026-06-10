//! The shell, one module per panel. Shared visual vocabulary lives here.

pub mod canvas;
pub mod charts;
pub mod hal_window;
pub mod inspector;
pub mod palette;
pub mod status_bar;
pub mod top_bar;

use crate::theme::ACCENT;
use bert_core::SubstanceType;
use egui::Color32;

pub const NODE_R: f32 = 27.0;

// BERT substance colors (message lifted to accent for visibility on cream).
pub fn substance_color(s: SubstanceType) -> Color32 {
    match s {
        SubstanceType::Energy => Color32::from_rgb(181, 27, 27),
        SubstanceType::Material => Color32::from_rgb(120, 120, 126),
        SubstanceType::Message => ACCENT,
    }
}

/// One-line physics of a base kind — shown under the substance picker so a
/// declared name ("money") carries its conservation law with it.
pub fn substance_blurb(base: SubstanceType) -> &'static str {
    match base {
        SubstanceType::Energy => "conserved — splits across fanouts, meters the amplifier",
        SubstanceType::Material => "conserved — splits and stores, never copies",
        SubstanceType::Message => "information — copies freely, gates and signals, not conserved",
    }
}
