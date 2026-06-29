//! Simple egui window for stepping through simulated robot status.

use std::f32::consts::PI;

use eframe::egui;

use crate::StepKind;

use super::report::SimulationReport;
use super::robot::{RobotState, RobotStep};
use super::visual::{action_glyph, opcode_display};

pub fn run_gui(report: SimulationReport) -> Result<(), String> {
    let title = format!("Maxx Steele — {}", report.input);
    let app = MaxxSimApp::new(report);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 720.0])
            .with_min_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(&title, options, Box::new(|cc| {
        super::ui_font::install(&cc.egui_ctx);
        Ok(Box::new(app))
    }))
    .map_err(|e| format!("GUI error: {e}"))
}

struct MaxxSimApp {
    report: SimulationReport,
    step_index: usize,
    playing: bool,
    speed: f32,
    step_elapsed: f32,
}

impl MaxxSimApp {
    fn new(report: SimulationReport) -> Self {
        Self {
            report,
            step_index: 0,
            playing: false,
            speed: 1.0,
            step_elapsed: 0.0,
        }
    }

    fn steps(&self) -> &[RobotStep] {
        &self.report.robot.steps
    }

    fn current_step(&self) -> Option<&RobotStep> {
        self.steps().get(self.step_index)
    }

    fn max_index(&self) -> usize {
        self.steps().len().saturating_sub(1)
    }

    fn step_duration(step: &RobotStep, speed: f32) -> f32 {
        let base = match &step.kind {
            StepKind::Delay { seconds } => f32::from(*seconds).max(0.2),
            StepKind::End => 0.1,
            _ => 0.45,
        };
        base / speed.max(0.1)
    }

    fn advance(&mut self) {
        if self.step_index < self.max_index() {
            self.step_index += 1;
            self.step_elapsed = 0.0;
        } else {
            self.playing = false;
        }
    }

    fn retreat(&mut self) {
        self.step_index = self.step_index.saturating_sub(1);
        self.step_elapsed = 0.0;
    }
}

impl eframe::App for MaxxSimApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.playing {
            let dt = ctx.input(|i| i.unstable_dt);
            self.step_elapsed += dt;
            if let Some(step) = self.current_step() {
                if self.step_elapsed >= Self::step_duration(step, self.speed) {
                    self.advance();
                }
            }
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let max_idx = self.max_index();

        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Maxx Steele Simulator");
                ui.separator();
                if ui.button("⏮").clicked() {
                    self.step_index = 0;
                    self.step_elapsed = 0.0;
                }
                if ui.button("◀").clicked() {
                    self.retreat();
                }
                let play_label = if self.playing { "⏸ Pause" } else { "▶ Play" };
                if ui.button(play_label).clicked() {
                    self.playing = !self.playing;
                }
                if ui.button("▶").clicked() {
                    self.advance();
                }
                if ui.button("⏭").clicked() {
                    self.step_index = max_idx;
                    self.step_elapsed = 0.0;
                }
                ui.separator();
                ui.add(egui::Slider::new(&mut self.step_index, 0..=max_idx).text("step"));
                ui.separator();
                ui.add(egui::Slider::new(&mut self.speed, 0.25..=4.0).text("speed"));
            });
        });

        let step_rows: Vec<(usize, String)> = self
            .steps()
            .iter()
            .enumerate()
            .filter(|(_, s)| !matches!(s.kind, StepKind::End))
            .map(|(i, s)| {
                (
                    i,
                    format!(
                        "{:>2} ${:02X} {:02X} {}",
                        s.index, s.opcode, s.operand, s.comment
                    ),
                )
            })
            .collect();

        egui::Panel::left("steps")
            .resizable(true)
            .default_size(260.0)
            .show_inside(ui, |ui| {
                ui.heading("Program");
                ui.label(&self.report.program.copyright);
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (i, label) in &step_rows {
                        let selected = *i == self.step_index;
                        if ui.selectable_label(selected, label).clicked() {
                            self.step_index = *i;
                            self.step_elapsed = 0.0;
                            self.playing = false;
                        }
                    }
                });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let Some(step) = self.current_step() else {
                ui.label("No program steps.");
                return;
            };

            ui.heading(&format!("Step {} — {}", step.index, step.comment));
            ui.horizontal(|ui| {
                ui.monospace(format!(
                    "opcode ${:02X} {:02X}",
                    step.opcode, step.operand
                ));
                ui.label(format!("display [{}]", opcode_display(step.opcode)));
                ui.label(format!("action [{}]", action_glyph(&step.kind)));
            });

            ui.separator();

            let (rect, _resp) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), ui.available_height() - 8.0),
                egui::Sense::hover(),
            );
            paint_robot(ui, rect, &step.state, &step.kind);

            ui.separator();
            let s = &step.state;
            ui.horizontal(|ui| {
                ui.label(format!("t = {:.1}s", s.time_s));
                ui.separator();
                ui.label(format!("pos ({:.1}, {:.1})", s.x, s.y));
                ui.separator();
                ui.label(format!("heading {:.0}°", s.heading_deg));
                ui.separator();
                ui.label(format!(
                    "arms {}  wrist {}  claw {}",
                    s.arms,
                    s.wrist,
                    if s.claw_open { "open" } else { "closed" }
                ));
                ui.separator();
                ui.label(format!("lamp {}", if s.lamp { "ON" } else { "off" }));
            });
            if !s.events.is_empty() {
                ui.label(format!("events: {}", s.events.join(", ")));
            }

            if let Some(fw) = &self.report.firmware {
                ui.separator();
                ui.collapsing("Firmware boot (summary)", |ui| {
                    ui.label(format!(
                        "cycles {}  pc ${:04X}  traps {}",
                        fw.cycles,
                        fw.final_pc,
                        fw.traps_hit.len()
                    ));
                });
            }
        });
    }
}

fn paint_robot(ui: &egui::Ui, rect: egui::Rect, state: &RobotState, kind: &StepKind) {
    let painter = ui.painter_at(rect);
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

    paint_plan_view(&painter, plan_rect, state);
    paint_front_view(&painter, robot_rect, state, kind);

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

fn paint_plan_view(painter: &egui::Painter, rect: egui::Rect, state: &RobotState) {
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

fn paint_front_view(painter: &egui::Painter, rect: egui::Rect, state: &RobotState, kind: &StepKind) {
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
    painter.circle_filled(
        egui::pos2(cx, body_top - 14.0),
        10.0,
        lamp_color,
    );
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
    painter.text(
        head_rect.center(),
        egui::Align2::CENTER_CENTER,
        opcode_display_for_kind(kind),
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