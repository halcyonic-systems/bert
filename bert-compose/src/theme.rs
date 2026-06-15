//! bert-compose design system (Halcyonic shell language, ported from hal-console) — warm paper, real typography, quiet color.
//!
//! The identity: a reading-room light theme (most technical tools are dark;
//! warm paper is the Halcyonic signature, shared with germen-gui). Inter
//! carries the UI, JetBrains Mono carries data. Color is reserved for
//! meaning: green = local/sovereign, amber = cloud/rented, accent blue =
//! interactive, gold = verdict.

use eframe::egui::{
    self, Color32, CornerRadius, FontFamily, FontId, Margin, RichText, Stroke, TextStyle,
};

// ── Palette ───────────────────────────────────────────────────────────────────

pub const CREAM: Color32 = Color32::from_rgb(246, 243, 236); // app background
pub const PAPER: Color32 = Color32::from_rgb(252, 251, 247); // cards, windows
pub const INPUT_BG: Color32 = Color32::from_rgb(255, 254, 251);
pub const HAIRLINE: Color32 = Color32::from_rgb(225, 219, 206);
pub const HOVER: Color32 = Color32::from_rgb(238, 234, 224);
pub const PRIMARY: Color32 = Color32::from_rgb(40, 42, 50); // ink
pub const SECONDARY: Color32 = Color32::from_rgb(126, 128, 137);
pub const ACCENT: Color32 = Color32::from_rgb(64, 99, 154);
pub const ACCENT_SOFT: Color32 = Color32::from_rgb(231, 237, 246);
pub const GREEN: Color32 = Color32::from_rgb(73, 124, 92); // local / sovereign
pub const GREEN_SOFT: Color32 = Color32::from_rgb(228, 240, 231);
#[allow(dead_code)] // kept for shell-crate parity with hal-console/germen
pub const AMBER: Color32 = Color32::from_rgb(163, 121, 47); // cloud / rented
#[allow(dead_code)]
pub const AMBER_SOFT: Color32 = Color32::from_rgb(247, 238, 219);
pub const RED: Color32 = Color32::from_rgb(170, 76, 72);
pub const GOLD: Color32 = Color32::from_rgb(168, 130, 50); // verdict / crown

// ── Fonts & global style ──────────────────────────────────────────────────────

pub fn medium() -> FontFamily {
    FontFamily::Name("medium".into())
}

pub fn semibold() -> FontFamily {
    FontFamily::Name("semibold".into())
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    for (name, bytes) in [
        (
            "inter",
            &include_bytes!("../assets/fonts/Inter-Regular.ttf")[..],
        ),
        (
            "inter-medium",
            &include_bytes!("../assets/fonts/Inter-Medium.ttf")[..],
        ),
        (
            "inter-semibold",
            &include_bytes!("../assets/fonts/Inter-SemiBold.ttf")[..],
        ),
        (
            "jbmono",
            &include_bytes!("../assets/fonts/JetBrainsMono-Regular.ttf")[..],
        ),
    ] {
        fonts
            .font_data
            .insert(name.to_string(), egui::FontData::from_static(bytes).into());
    }
    // Prepend so egui's built-ins stay as glyph fallbacks (emoji, symbols).
    let prop = fonts.families.entry(FontFamily::Proportional).or_default();
    prop.insert(0, "inter".to_string());
    let mono = fonts.families.entry(FontFamily::Monospace).or_default();
    mono.insert(0, "jbmono".to_string());
    let fallbacks: Vec<String> = fonts.families[&FontFamily::Proportional][1..].to_vec();
    for (family, font) in [("medium", "inter-medium"), ("semibold", "inter-semibold")] {
        let mut list = vec![font.to_string()];
        list.extend(fallbacks.iter().cloned());
        fonts.families.insert(FontFamily::Name(family.into()), list);
    }
    ctx.set_fonts(fonts);
}

pub fn apply(ctx: &egui::Context) {
    install_fonts(ctx);
    // Germen-family default scale (germen-gui uses 1.45; hal-console runs
    // multi-column layouts, so a notch lower). Cmd +/- adjusts live.
    ctx.set_zoom_factor(1.3);
    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (TextStyle::Heading, FontId::new(16.5, semibold())),
        (TextStyle::Body, FontId::new(13.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(12.5, medium())),
        (
            TextStyle::Small,
            FontId::new(10.5, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(11.5, FontFamily::Monospace),
        ),
    ]
    .into();

    style.spacing.item_spacing = egui::vec2(8.0, 7.0);
    style.spacing.button_padding = egui::vec2(11.0, 4.5);
    style.spacing.interact_size.y = 24.0;
    style.spacing.scroll = egui::style::ScrollStyle::thin();

    let v = &mut style.visuals;
    v.panel_fill = CREAM;
    v.window_fill = PAPER;
    v.extreme_bg_color = INPUT_BG;
    v.override_text_color = Some(PRIMARY);
    v.faint_bg_color = HOVER;

    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, HAIRLINE);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, PRIMARY);

    v.widgets.inactive.weak_bg_fill = PAPER;
    v.widgets.inactive.bg_fill = PAPER;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, HAIRLINE);
    v.widgets.inactive.corner_radius = CornerRadius::same(6);

    v.widgets.hovered.weak_bg_fill = HOVER;
    v.widgets.hovered.bg_fill = HOVER;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(200, 193, 178));
    v.widgets.hovered.corner_radius = CornerRadius::same(6);

    v.widgets.active.weak_bg_fill = ACCENT_SOFT;
    v.widgets.active.bg_fill = ACCENT_SOFT;
    v.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    v.widgets.active.corner_radius = CornerRadius::same(6);

    v.widgets.open.weak_bg_fill = ACCENT_SOFT;
    v.widgets.open.bg_fill = PAPER;
    v.widgets.open.bg_stroke = Stroke::new(1.0, ACCENT);
    v.widgets.open.corner_radius = CornerRadius::same(6);

    v.selection.bg_fill = ACCENT_SOFT;
    v.selection.stroke = Stroke::new(1.0, ACCENT);

    v.window_corner_radius = CornerRadius::same(12);
    v.window_stroke = Stroke::new(1.0, HAIRLINE);
    v.window_shadow = egui::epaint::Shadow {
        offset: [0, 6],
        blur: 24,
        spread: 0,
        color: Color32::from_black_alpha(28),
    };
    v.popup_shadow = egui::epaint::Shadow {
        offset: [0, 3],
        blur: 12,
        spread: 0,
        color: Color32::from_black_alpha(22),
    };

    ctx.set_style(style);
}

// ── Reusable pieces ───────────────────────────────────────────────────────────

/// Letter-spaced small-caps section header — the sidebar/roster signature.
pub fn section_header(ui: &mut egui::Ui, label: &str) {
    let mut job = egui::text::LayoutJob::default();
    job.append(
        label,
        0.0,
        egui::TextFormat {
            font_id: FontId::new(10.0, semibold()),
            color: SECONDARY,
            extra_letter_spacing: 1.4,
            ..Default::default()
        },
    );
    ui.label(job);
}

/// Small rounded badge.
pub fn pill(ui: &mut egui::Ui, text: &str, fg: Color32, bg: Color32) {
    egui::Frame::new()
        .fill(bg)
        .corner_radius(CornerRadius::same(9))
        .inner_margin(Margin::symmetric(6, 1))
        .show(ui, |ui| {
            ui.label(RichText::new(text).color(fg).size(9.5).family(medium()));
        });
}

/// The one signal hal is about, as a badge.
#[allow(dead_code)]
pub fn locality_pill(ui: &mut egui::Ui, local: bool) {
    if local {
        pill(ui, "local", GREEN, GREEN_SOFT);
    } else {
        pill(ui, "cloud", AMBER, AMBER_SOFT);
    }
}

/// Standard content card.
#[allow(dead_code)]
pub fn card() -> egui::Frame {
    egui::Frame::new()
        .fill(PAPER)
        .stroke(Stroke::new(1.0, HAIRLINE))
        .corner_radius(CornerRadius::same(10))
        .inner_margin(Margin::same(12))
}

/// Small filled status dot, vertically centered on the text baseline row.
pub fn dot(ui: &mut egui::Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(7.0, 7.0), egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), 3.0, color);
}

/// Primary action button (filled accent).
pub fn primary_button(text: &str) -> egui::Button<'static> {
    egui::Button::new(RichText::new(text).color(Color32::WHITE).family(semibold()))
        .fill(ACCENT)
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(7))
}

/// Full-width clickable row with hover/selected background. Returns the
/// response; the closure draws the row content.
#[allow(dead_code)]
pub fn hover_row(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    selected: bool,
    add_contents: impl FnOnce(&mut egui::Ui),
) -> egui::Response {
    let response = ui
        .scope_builder(
            egui::UiBuilder::new()
                .id_salt(id)
                .sense(egui::Sense::click()),
            |ui| {
                let hovered = ui.response().hovered();
                let fill = if selected {
                    ACCENT_SOFT
                } else if hovered {
                    HOVER
                } else {
                    Color32::TRANSPARENT
                };
                egui::Frame::new()
                    .fill(fill)
                    .corner_radius(CornerRadius::same(6))
                    .inner_margin(Margin::symmetric(7, 3))
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        add_contents(ui);
                    });
            },
        )
        .response;
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response
}

/// Canvas empty state for bert-compose.
/// Empty-canvas orientation. Returns true if the user clicked the "what is
/// this?" link (the caller opens the About window — keeps theme UI-state free).
pub fn empty_state_inline(ui: &mut egui::Ui) -> bool {
    let mut want_about = false;
    ui.add_space(ui.available_height() * 0.24);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("⚒").color(HAIRLINE).size(44.0));
        ui.add_space(4.0);
        ui.label(
            RichText::new("Build a system from work processes")
                .color(PRIMARY)
                .family(semibold())
                .size(14.0),
        );
        ui.add_space(8.0);
        // Three concrete on-ramps, easiest first.
        ui.label(
            RichText::new("①  Load an example  (Examples ▾, top bar)")
                .color(SECONDARY)
                .size(12.0),
        );
        ui.add_space(2.0);
        ui.label(
            RichText::new("②  Stamp a process  (Systems Processes, left palette)")
                .color(SECONDARY)
                .size(12.0),
        );
        ui.add_space(2.0);
        ui.label(
            RichText::new("③  Or add a primitive, wire ◦ → component, press Run")
                .color(SECONDARY)
                .size(12.0),
        );
        ui.add_space(10.0);
        if ui
            .add(
                egui::Label::new(RichText::new("what is this? →").color(ACCENT).size(12.0))
                    .sense(egui::Sense::click()),
            )
            .on_hover_text("open the orientation")
            .clicked()
        {
            want_about = true;
        }
    });
    want_about
}
