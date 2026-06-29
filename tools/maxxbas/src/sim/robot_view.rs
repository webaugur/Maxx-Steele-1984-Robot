//! Shared robot playfield — plan grid + front pose (live GUI and storyboard GUI).

use std::f32::consts::PI;

use eframe::egui;

use crate::StepKind;

use super::robot::RobotState;
use super::visual::action_glyph;

/// Plan grid (bottom) + animated front view (top), matching the offline storyboard GUI.
pub fn paint_robot_playfield(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &RobotState,
    kind: &StepKind,
    led: Option<&str>,
    show_action_banner: bool,
) {
    painter.rect_filled(rect, 8.0, egui::Color32::from_gray(28));

    let center = rect.center();
    let plan_h = rect.height() * 0.38;
    let robot_h = rect.height() - plan_h - 16.0;
    let plan_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 12.0, rect.bottom() - plan_h - 8.0),
        egui::pos2(rect.right() - 12.0, rect.bottom() - 8.0),
    );
    let robot_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 12.0, rect.top() + 8.0),
        egui::pos2(rect.right() - 12.0, rect.top() + 8.0 + robot_h),
    );

    paint_plan_view(painter, plan_rect, state);
    paint_front_view(painter, robot_rect, state, kind, led);

    if show_action_banner {
        let banner = action_glyph(kind);
        let font = egui::FontId::proportional(22.0);
        let galley = painter.layout_no_wrap(
            banner.to_string(),
            font,
            egui::Color32::from_rgb(255, 220, 80),
        );
        painter.galley(
            egui::pos2(center.x - galley.size().x * 0.5, rect.top() + 12.0),
            galley,
            egui::Color32::WHITE,
        );
    }
}

pub fn paint_plan_view(painter: &egui::Painter, rect: egui::Rect, state: &RobotState) {
    painter.rect_filled(rect, 6.0, egui::Color32::from_gray(40));
    painter.text(
        rect.left_top() + egui::vec2(8.0, 6.0),
        egui::Align2::LEFT_TOP,
        "Plan view",
        egui::FontId::proportional(14.0),
        egui::Color32::LIGHT_GRAY,
    );

    let margin = 24.0;
    let span = (rect.width().min(rect.height()) - margin * 2.0).max(40.0);
    let origin = egui::pos2(rect.center().x, rect.center().y);
    let scale = span / 40.0;

    for i in -2..=2 {
        let o = i as f32 * 10.0 * scale;
        painter.line_segment(
            [
                egui::pos2(origin.x + o, rect.top() + margin),
                egui::pos2(origin.x + o, rect.bottom() - margin),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_gray(55)),
        );
        painter.line_segment(
            [
                egui::pos2(rect.left() + margin, origin.y + o),
                egui::pos2(rect.right() - margin, origin.y + o),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_gray(55)),
        );
    }

    let px = origin.x + state.x * scale;
    let py = origin.y - state.y * scale;
    let body = egui::vec2(18.0, 14.0);
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(px, py), body),
        3.0,
        egui::Color32::from_rgb(90, 140, 200),
    );

    let rad = state.heading_deg.to_radians();
    let arrow_len = 22.0;
    let tip = egui::pos2(
        px + arrow_len * rad.sin(),
        py - arrow_len * rad.cos(),
    );
    painter.line_segment(
        [egui::pos2(px, py), tip],
        egui::Stroke::new(2.5, egui::Color32::from_rgb(120, 200, 255)),
    );
}

pub fn paint_front_view(
    painter: &egui::Painter,
    rect: egui::Rect,
    state: &RobotState,
    kind: &StepKind,
    led: Option<&str>,
) {
    painter.rect_filled(rect, 6.0, egui::Color32::from_gray(32));
    painter.text(
        rect.left_top() + egui::vec2(8.0, 6.0),
        egui::Align2::LEFT_TOP,
        "Robot status",
        egui::FontId::proportional(14.0),
        egui::Color32::LIGHT_GRAY,
    );

    let cx = rect.center().x;
    let base_y = rect.bottom() - 36.0;
    let body_w = 120.0;
    let body_h = 90.0 + (state.arms.min(64) as f32 / 64.0) * 40.0;

    let wheel_r = 16.0;
    painter.circle_filled(
        egui::pos2(cx - 42.0, base_y),
        wheel_r,
        egui::Color32::from_gray(70),
    );
    painter.circle_filled(
        egui::pos2(cx + 42.0, base_y),
        wheel_r,
        egui::Color32::from_gray(70),
    );

    let body_top = base_y - body_h;
    let body_rect = egui::Rect::from_min_max(
        egui::pos2(cx - body_w * 0.5, body_top),
        egui::pos2(cx + body_w * 0.5, base_y - 6.0),
    );
    painter.rect_filled(body_rect, 8.0, egui::Color32::from_rgb(70, 110, 160));

    let lamp_color = if state.lamp {
        egui::Color32::from_rgb(255, 230, 80)
    } else {
        egui::Color32::from_gray(90)
    };
    painter.circle_filled(egui::pos2(cx, body_top - 14.0), 10.0, lamp_color);
    if state.lamp {
        painter.circle_stroke(
            egui::pos2(cx, body_top - 14.0),
            16.0,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 60)),
        );
    }

    let head_rect = egui::Rect::from_center_size(
        egui::pos2(cx, body_top + 22.0),
        egui::vec2(70.0, 28.0),
    );
    painter.rect_filled(head_rect, 4.0, egui::Color32::from_gray(20));
    let face = led
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| opcode_display_for_kind(kind));
    painter.text(
        head_rect.center(),
        egui::Align2::CENTER_CENTER,
        face,
        egui::FontId::monospace(18.0),
        egui::Color32::from_rgb(80, 255, 120),
    );

    let arm_y = body_top + 38.0;
    let arm_drop = (state.arms.min(64) as f32 / 64.0) * 30.0;
    for sign in [-1.0_f32, 1.0] {
        let shoulder = egui::pos2(cx + sign * 48.0, arm_y);
        let elbow = egui::pos2(cx + sign * 72.0, arm_y + arm_drop);
        let wrist_y = arm_y + arm_drop + (state.wrist.min(64) as f32 / 64.0) * 18.0;
        let wrist = egui::pos2(cx + sign * 88.0, wrist_y);
        painter.line_segment(
            [shoulder, elbow],
            egui::Stroke::new(5.0, egui::Color32::from_rgb(100, 150, 210)),
        );
        painter.line_segment(
            [elbow, wrist],
            egui::Stroke::new(4.0, egui::Color32::from_rgb(120, 170, 220)),
        );

        let claw_gap = if state.claw_open { 8.0 } else { 2.0 };
        let rot = (state.claw_rotate % 4) as f32 * PI * 0.5;
        let claw_base = wrist + egui::vec2(sign * 12.0, 0.0);
        let p1 = claw_base + egui::vec2(rot.cos() * 10.0 * sign, rot.sin() * 10.0);
        let p2 = claw_base + egui::vec2(-rot.sin() * claw_gap * sign, rot.cos() * claw_gap);
        painter.line_segment(
            [claw_base, p1],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(200, 200, 210)),
        );
        painter.line_segment(
            [claw_base, p2],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(200, 200, 210)),
        );
    }

    if matches!(
        kind,
        StepKind::Forward { .. }
            | StepKind::Back { .. }
            | StepKind::Left { .. }
            | StepKind::Right { .. }
    ) {
        painter.rect_stroke(
            body_rect.expand(6.0),
            10.0,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 220, 255)),
            egui::StrokeKind::Outside,
        );
    }
}

fn opcode_display_for_kind(kind: &StepKind) -> &'static str {
    match kind {
        StepKind::Forward { .. } => "F",
        StepKind::Back { .. } => "b",
        StepKind::Left { .. } => "L",
        StepKind::Right { .. } => "r",
        StepKind::Delay { .. } => "d",
        StepKind::Lamp { .. } => "HL",
        StepKind::Home => "init",
        StepKind::Play { .. } => "P",
        StepKind::SpeakRom { .. } | StepKind::SpeakRam { .. } => "S",
        _ => action_glyph(kind),
    }
}