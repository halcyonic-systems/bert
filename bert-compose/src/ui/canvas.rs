//! The canvas: nodes, wires, live flow, and the wiring interaction.

use crate::app::App;
use crate::circuit::{self, NodeKind, Wire};
use crate::glyph;
use crate::theme::{
    self, ACCENT, ACCENT_SOFT, GOLD, GREEN_SOFT, HAIRLINE, PAPER, PRIMARY, SECONDARY,
};
use crate::ui::{substance_color, NODE_R};
use egui::{vec2, Sense, Stroke};

pub fn show(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(theme::CREAM))
        .show(ctx, |ui| {
            if app.circuit.nodes.is_empty() {
                if theme::empty_state_inline(ui) {
                    app.show_about = true;
                }
                return;
            }
            // Pan: drag empty canvas to move the whole diagram. The background
            // senses DRAG ONLY — never clicks — so it can't steal a node's
            // click (that bug ate node selection). Node interacts (added
            // later, on top) win their own drags; empty-space drags pan.
            let canvas_rect = ui.max_rect();
            app.canvas_origin = canvas_rect.min;
            let bg = ui.interact(canvas_rect, ui.id().with("canvas_bg"), Sense::drag());
            if bg.dragged() {
                app.pan += bg.drag_delta();
            }
            let pan = app.pan;
            let painter = ui.painter();
            let time = ui.input(|i| i.time) as f32;

            // Lens caption — labels the diagram in the corner rather than
            // crowding the top bar. The active domain reading lives here.
            if app.lens != 0 {
                let l = &crate::lens::LENSES[app.lens];
                let top_left = ui.max_rect().left_top() + vec2(12.0, 10.0);
                painter.text(
                    top_left,
                    egui::Align2::LEFT_TOP,
                    format!("🔍 {}", l.name),
                    egui::FontId::proportional(13.0),
                    ACCENT,
                );
                painter.text(
                    top_left + vec2(0.0, 18.0),
                    egui::Align2::LEFT_TOP,
                    l.tagline,
                    egui::FontId::proportional(10.5),
                    SECONDARY,
                );
            }

            // Wires first (under nodes).
            for (k, wire) in app.circuit.wires.iter().enumerate() {
                let a = app.circuit.nodes[wire.from].pos + pan;
                let b = app.circuit.nodes[wire.to].pos + pan;
                let substance = app.circuit.wire_substance(wire);
                let color = substance_color(substance);
                let dir = (b - a).normalized();
                let (a_edge, b_edge) = (a + dir * NODE_R, b - dir * (NODE_R + 4.0));
                let gradient = wire.mode == circuit::FlowMode::Gradient;
                // Gradient flows: thickness scales with the LIVE potential
                // difference — the wire visibly thins as the two stocks
                // equalize. You watch the field relax.
                let delta = (app.circuit.level(wire.from) - app.circuit.level(wire.to)).max(0.0);
                let width: f32 = if gradient {
                    (0.8 + 0.5 * delta).min(5.0)
                } else {
                    1.6
                };
                if gradient {
                    // dashed to read as a field, not a pushed pipe
                    let seg = b_edge - a_edge;
                    let steps = (seg.length() / 9.0).max(1.0) as i32;
                    for s in 0..steps {
                        if s % 2 == 0 {
                            let p0 = a_edge + seg * (s as f32 / steps as f32);
                            let p1 = a_edge + seg * ((s as f32 + 1.0) / steps as f32);
                            painter.line_segment([p0, p1], Stroke::new(width, color));
                        }
                    }
                } else {
                    painter.line_segment([a_edge, b_edge], Stroke::new(width, color));
                }
                // arrowhead
                let n = vec2(-dir.y, dir.x);
                painter.line_segment(
                    [b_edge, b_edge - dir * 7.0 + n * 4.0],
                    Stroke::new(1.6, color),
                );
                painter.line_segment(
                    [b_edge, b_edge - dir * 7.0 - n * 4.0],
                    Stroke::new(1.6, color),
                );
                // live amount + moving pulse
                let amount = if gradient {
                    wire.conductance * delta
                } else {
                    app.circuit.nodes[wire.from].activity
                };
                let mid = a_edge + (b_edge - a_edge) * 0.5;
                let unit = &app.circuit.nodes[wire.from].out_substance.unit;
                painter.text(
                    mid + vec2(0.0, -10.0),
                    egui::Align2::CENTER_CENTER,
                    format!(
                        "{amount:.1}{}{}",
                        if unit.is_empty() {
                            String::new()
                        } else {
                            format!(" {unit}")
                        },
                        if gradient { " ⤓" } else { "" }
                    ),
                    egui::FontId::monospace(9.5),
                    if amount > 0.005 { color } else { HAIRLINE },
                );
                if app.running && amount > 0.005 {
                    let t = (time * 0.7 + k as f32 * 0.37).fract();
                    let p = a_edge + (b_edge - a_edge) * t;
                    painter.circle_filled(p, 3.0, color);
                }
            }

            // Substance mismatches: nodes silently ignoring a flow they can't use.
            let mut mismatched: std::collections::HashSet<usize> = app
                .circuit
                .substance_mismatches()
                .iter()
                .map(|(i, _, _)| *i)
                .collect();
            mismatched.extend(app.circuit.underpowered_amplifiers());

            // Nodes.
            let mut clicked_body: Option<usize> = None;
            let mut clicked_port: Option<usize> = None;
            for i in 0..app.circuit.nodes.len() {
                let pos = app.circuit.nodes[i].pos + pan;
                let rect = egui::Rect::from_center_size(pos, vec2(NODE_R * 2.0, NODE_R * 2.0));
                let resp = ui.interact(rect, ui.id().with(("node", i)), Sense::click_and_drag());
                if resp.dragged() {
                    app.circuit.nodes[i].pos += resp.drag_delta();
                }
                if resp.clicked() {
                    clicked_body = Some(i);
                }
                // Out-port handle.
                let port = pos + vec2(NODE_R + 7.0, 0.0);
                let port_rect = egui::Rect::from_center_size(port, vec2(14.0, 14.0));
                let port_resp = ui.interact(port_rect, ui.id().with(("port", i)), Sense::click());
                if port_resp.clicked() {
                    clicked_port = Some(i);
                }

                let node = &app.circuit.nodes[i];
                let painter = ui.painter();
                let (ring, ring_w) = if app.pending_wire == Some(i) {
                    (GOLD, 2.2)
                } else if app.selected == Some(i) {
                    (ACCENT, 2.0)
                } else if mismatched.contains(&i) {
                    (theme::AMBER, 2.0)
                } else {
                    (HAIRLINE, 1.2)
                };
                let fill = match node.kind {
                    NodeKind::Source => GREEN_SOFT,
                    NodeKind::Sink => ACCENT_SOFT,
                    _ => PAPER,
                };
                // The shape IS the semantics — vessel, funnel, valve, eye…
                glyph::draw(painter, node.kind, pos, fill, Stroke::new(ring_w, ring));
                // Buffer fill: the vessel visibly holds its stock.
                if matches!(
                    node.kind,
                    NodeKind::Process(bert_core::ProcessPrimitive::Buffering)
                ) {
                    // Fill fraction relative to capacity if bounded, else the
                    // initial stock (a sensible visual scale).
                    let full = if node.capacity > 0.0 {
                        node.capacity
                    } else {
                        node.initial_storage.max(10.0)
                    };
                    let frac = (node.storage / full).clamp(0.0, 1.0);
                    let h = glyph::R * 1.6;
                    let inner = egui::Rect::from_center_size(
                        pos + vec2(0.0, 2.0 + h * 0.5 * (1.0 - frac)),
                        vec2(glyph::R * 1.3, h * frac),
                    );
                    painter.rect_filled(inner, 2.0, GOLD.gamma_multiply(0.55));
                }
                painter.text(
                    pos + vec2(0.0, 8.0),
                    egui::Align2::CENTER_CENTER,
                    format!("{:.1}", node.activity),
                    egui::FontId::monospace(9.0),
                    PRIMARY,
                );
                painter.text(
                    pos + vec2(0.0, glyph::R + 11.0),
                    egui::Align2::CENTER_CENTER,
                    crate::lens::display_name(app.lens, node.kind, &node.name),
                    egui::FontId::proportional(10.0),
                    SECONDARY,
                );
                if mismatched.contains(&i) {
                    painter.text(
                        pos + vec2(0.0, -NODE_R - 8.0),
                        egui::Align2::CENTER_CENTER,
                        "⚠",
                        egui::FontId::proportional(14.0),
                        theme::AMBER,
                    );
                }
                // port
                let port_color = if port_resp.hovered() {
                    GOLD
                } else {
                    substance_color(node.out_substance.base)
                };
                painter.circle(port, 5.0, PAPER, Stroke::new(1.6, port_color));
            }

            // Wiring interaction: port starts, body completes. Boundary
            // discipline: a Sink only absorbs (nothing flows out of it) and a
            // Source only emits (flows can't run back into the environment
            // input) — both would break conservation.
            if let Some(i) = clicked_port {
                if matches!(app.circuit.nodes[i].kind, NodeKind::Sink) {
                    app.status = format!(
                        "{} is a sink — it only absorbs; nothing flows out of it",
                        app.circuit.nodes[i].name
                    );
                } else {
                    app.pending_wire = Some(i);
                    app.status = format!(
                        "wiring from {} — click a target component (esc cancels)",
                        app.circuit.nodes[i].name
                    );
                }
            } else if let Some(i) = clicked_body {
                if let Some(from) = app.pending_wire.take() {
                    if matches!(app.circuit.nodes[i].kind, NodeKind::Source) {
                        app.status = format!(
                            "{} is a source — it only emits; flows can't run back into it",
                            app.circuit.nodes[i].name
                        );
                    } else if from != i
                        && !app
                            .circuit
                            .wires
                            .iter()
                            .any(|w| w.from == from && w.to == i)
                    {
                        app.circuit.wires.push(Wire::new(from, i));
                        app.status = format!(
                            "bond: {} → {} (internalized — both endpoints inside)",
                            app.circuit.nodes[from].name, app.circuit.nodes[i].name
                        );
                    }
                } else {
                    app.selected = Some(i);
                }
            } else {
                // A click that landed on empty canvas (not a node or port)
                // deselects — computed here, after node hits are known, so it
                // never fights node selection.
                let clicked_empty = ui.input(|i| i.pointer.primary_clicked())
                    && ui
                        .ctx()
                        .pointer_interact_pos()
                        .is_some_and(|p| canvas_rect.contains(p));
                if clicked_empty {
                    app.selected = None;
                    app.pending_wire = None;
                }
            }

            // Pending wire follows the pointer.
            if let Some(from) = app.pending_wire {
                if let Some(p) = ui.ctx().pointer_latest_pos() {
                    ui.painter().line_segment(
                        [app.circuit.nodes[from].pos + pan, p],
                        Stroke::new(1.2, GOLD),
                    );
                }
                ui.ctx().request_repaint();
            }

            let _ = theme::pill; // theme helpers kept for parity
        });
}
