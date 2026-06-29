//! Simple egui window for stepping through simulated robot status.

use eframe::egui;

use crate::StepKind;

use super::report::SimulationReport;
use super::robot::RobotStep;
use super::robot_view;
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
            paint_robot(ui, rect, step);

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

fn paint_robot(ui: &egui::Ui, rect: egui::Rect, step: &RobotStep) {
    let painter = ui.painter_at(rect);
    robot_view::paint_robot_playfield(
        &painter,
        rect,
        &step.state,
        &step.kind,
        None,
        true,
    );
}