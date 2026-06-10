//! Each primitive gets a shape that *is* its semantics — a vessel holds, a
//! funnel merges, a valve gates. Form follows function so the circuit reads as
//! a diagram of work, not a graph of dots.

use crate::circuit::NodeKind;
use bert_core::ProcessPrimitive;
use egui::{vec2, Color32, Painter, Pos2, Stroke, StrokeKind};

pub const R: f32 = 26.0;

/// Draw the node body. `fill`/`ring` already chosen by the caller (selection,
/// mismatch, etc). Returns nothing — pure paint.
pub fn draw(painter: &Painter, kind: NodeKind, c: Pos2, fill: Color32, ring: Stroke) {
    let s = ring;
    match kind {
        // Source: a right-pointing triangle — it emits.
        NodeKind::Source => {
            tri(painter, c, R, 0.0, fill, s);
        }
        // Sink: a triangle into a wall — it absorbs.
        NodeKind::Sink => {
            tri(painter, c, R, std::f32::consts::PI, fill, s);
            painter.line_segment(
                [c + vec2(R * 0.7, -R), c + vec2(R * 0.7, R)],
                Stroke::new(s.width, s.color),
            );
        }
        NodeKind::Process(p) => process(painter, p, c, fill, s),
    }
}

fn process(painter: &Painter, p: ProcessPrimitive, c: Pos2, fill: Color32, s: Stroke) {
    use ProcessPrimitive::*;
    match p {
        // Buffering: a vessel — open-topped cup that holds stock.
        Buffering => {
            let r = egui::Rect::from_center_size(c + vec2(0.0, 2.0), vec2(R * 1.5, R * 1.7));
            painter.rect(r, 3.0, fill, s, StrokeKind::Inside);
            // open top
            painter.line_segment([r.left_top(), r.right_top()], Stroke::new(s.width, fill));
            painter.line_segment(
                [r.left_top() + vec2(0.0, -1.0), r.left_top() + vec2(4.0, -1.0)],
                s,
            );
            painter.line_segment(
                [r.right_top() + vec2(0.0, -1.0), r.right_top() + vec2(-4.0, -1.0)],
                s,
            );
        }
        // Combining: funnel, wide→narrow (many in, one out).
        Combining => funnel(painter, c, fill, s, false),
        // Splitting: funnel, narrow→wide (one in, many out).
        Splitting => funnel(painter, c, fill, s, true),
        // Amplifying: the classic amp triangle (gain).
        Amplifying => tri(painter, c, R * 1.1, 0.0, fill, s),
        // Modulating: a valve — diamond pinch.
        Modulating => diamond(painter, c, R, fill, s),
        // Sensing: an eye — circle with pupil.
        Sensing => {
            painter.circle(c, R, fill, s);
            painter.circle_filled(c, R * 0.3, s.color);
        }
        // Inverting: circle bisected (high↔low).
        Inverting => {
            painter.circle(c, R, fill, s);
            painter.line_segment([c + vec2(-R, R), c + vec2(R, -R)], s);
        }
        // Copying: stacked offset squares (replication).
        Copying => {
            let sz = vec2(R * 1.3, R * 1.3);
            painter.rect(
                egui::Rect::from_center_size(c + vec2(5.0, 5.0), sz),
                2.0,
                fill,
                Stroke::new(s.width, s.color.gamma_multiply(0.5)),
                StrokeKind::Inside,
            );
            painter.rect(
                egui::Rect::from_center_size(c + vec2(-3.0, -3.0), sz),
                2.0,
                fill,
                s,
                StrokeKind::Inside,
            );
        }
        // Propelling: chevron — a push.
        Propelling => {
            for dx in [-6.0, 4.0] {
                painter.add(egui::Shape::line(
                    vec![
                        c + vec2(dx - 6.0, -R * 0.7),
                        c + vec2(dx + 6.0, 0.0),
                        c + vec2(dx - 6.0, R * 0.7),
                    ],
                    s,
                ));
            }
            painter.circle_stroke(c, R, Stroke::new(0.8, s.color.gamma_multiply(0.4)));
        }
        // Impeding: a resistor zigzag in a pill — resistance.
        Impeding => {
            let r = egui::Rect::from_center_size(c, vec2(R * 1.8, R * 1.1));
            painter.rect(r, R * 0.55, fill, s, StrokeKind::Inside);
            let mut pts = vec![c + vec2(-R * 0.7, 0.0)];
            for i in 0..5 {
                let x = -R * 0.7 + (i as f32 + 0.5) * (R * 1.4 / 5.0);
                pts.push(c + vec2(x, if i % 2 == 0 { -6.0 } else { 6.0 }));
            }
            pts.push(c + vec2(R * 0.7, 0.0));
            painter.add(egui::Shape::line(pts, Stroke::new(s.width, s.color)));
        }
    }
}

// — primitives —

fn tri(painter: &Painter, c: Pos2, r: f32, rot: f32, fill: Color32, s: Stroke) {
    let pt = |a: f32| {
        let aa = a + rot;
        c + vec2(aa.cos(), aa.sin()) * r
    };
    use std::f32::consts::PI;
    let pts = vec![pt(0.0), pt(2.0 * PI / 3.0), pt(4.0 * PI / 3.0)];
    painter.add(egui::Shape::convex_polygon(pts, fill, s));
}

fn diamond(painter: &Painter, c: Pos2, r: f32, fill: Color32, s: Stroke) {
    let pts = vec![
        c + vec2(0.0, -r),
        c + vec2(r, 0.0),
        c + vec2(0.0, r),
        c + vec2(-r, 0.0),
    ];
    painter.add(egui::Shape::convex_polygon(pts, fill, s));
}

fn funnel(painter: &Painter, c: Pos2, fill: Color32, s: Stroke, flip: bool) {
    let w = R * 1.4;
    let (top, bot) = if flip { (R * 0.4, w) } else { (w, R * 0.4) };
    let pts = vec![
        c + vec2(-top, -R),
        c + vec2(top, -R),
        c + vec2(bot, R),
        c + vec2(-bot, R),
    ];
    painter.add(egui::Shape::convex_polygon(pts, fill, s));
}
