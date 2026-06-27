//! Interactive simulator window — live firmware + remote keypad.

use std::path::Path;

use eframe::egui;

use super::interactive::InteractiveFirmware;
use super::keypad::RemoteKey;
use super::remote_panel;
use crate::CartImage;

const SIM_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Frames stepped synchronously before the window opens (ROM boot + first LED prompt).
const STARTUP_WARMUP_FRAMES: u64 = 180;

/// One `step_frame` per egui logic tick while idling.
const FRAMES_PER_TICK: u32 = 1;

/// CPU frames for synchronous digest after a keypress (cart dispatch + auto-ENTER).
const KEYPRESS_DIGEST_FRAMES: u32 = 800;

/// Faster stepping while digesting a key (real-time default is too few cycles/frame).
const DIGEST_CYCLES_PER_FRAME: u64 = 16_000;

pub fn run_live_gui(cart: CartImage, label: impl Into<String>) -> Result<(), String> {
    let label = label.into();
    let cart_name = short_label(&label);
    let title = format!("Maxx Steele Live v{SIM_VERSION} — {cart_name}");
    let mut fw = InteractiveFirmware::new(cart, title.clone())?;
    fw.set_auto_submit_enter(true);
    fw.warmup(STARTUP_WARMUP_FRAMES);
    let trace_display = fw.trace_text();
    let app = LiveSimApp {
        firmware: fw,
        cart_label: cart_name,
        sim_version: SIM_VERSION.to_string(),
        trace_display,
        pending_key: None,
        last_input: None,
        trace_editable: false,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 860.0])
            .with_min_inner_size([720.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(&title, options, Box::new(|_cc| Ok(Box::new(app))))
        .map_err(|e| format!("GUI error: {e}"))
}

struct LiveSimApp {
    firmware: InteractiveFirmware,
    cart_label: String,
    sim_version: String,
    trace_display: String,
    pending_key: Option<RemoteKey>,
    last_input: Option<String>,
    trace_editable: bool,
}

fn short_label(label: &str) -> String {
    Path::new(label)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(label)
        .to_string()
}

fn poll_keyboard(ctx: &egui::Context) -> Option<RemoteKey> {
    const ROW: &[(egui::Key, u8)] = &[
        (egui::Key::Num0, 0),
        (egui::Key::Num1, 1),
        (egui::Key::Num2, 2),
        (egui::Key::Num3, 3),
        (egui::Key::Num4, 4),
        (egui::Key::Num5, 5),
        (egui::Key::Num6, 6),
        (egui::Key::Num7, 7),
        (egui::Key::Num8, 8),
        (egui::Key::Num9, 9),
    ];
    ctx.input(|input| {
        for &(key, digit) in ROW {
            if input.key_pressed(key) {
                return RemoteKey::from_digit(digit);
            }
        }
        if input.key_pressed(egui::Key::Enter) {
            return Some(RemoteKey::Enter);
        }
        None
    })
}

fn queue_key(app: &mut LiveSimApp, key: RemoteKey, source: &str) {
    app.pending_key = Some(key);
    app.last_input = Some(format!("{source}: {}", key.label()));
}

fn deliver_key(app: &mut LiveSimApp) {
    let saved_cpf = app.firmware.options.cycles_per_frame;
    app.firmware.options.cycles_per_frame = DIGEST_CYCLES_PER_FRAME;
    if let Some(key) = app.pending_key.take() {
        app.firmware.press_key(key);
    }
    app.firmware.digest_keypress(KEYPRESS_DIGEST_FRAMES);
    app.firmware.options.cycles_per_frame = saved_cpf;
    app.trace_display = app.firmware.trace_text();
}

impl eframe::App for LiveSimApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.pending_key.is_none() {
            if let Some(key) = poll_keyboard(ctx) {
                queue_key(self, key, "keyboard");
            }
        }

        if self.pending_key.is_some() {
            deliver_key(self);
            ctx.request_repaint();
            return;
        }

        if !self.firmware.status().running {
            return;
        }

        for _ in 0..FRAMES_PER_TICK {
            self.firmware.step_frame();
        }
        ctx.request_repaint();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 4.0);

            ui.horizontal(|ui| {
                let st = self.firmware.status();
                let run_label = if st.running { "Pause" } else { "Run" };
                if ui.button(format!("{run_label} CPU")).clicked() {
                    self.firmware.set_running(!st.running);
                }
                if ui.button("Reset").clicked() {
                    let _ = self.firmware.reset();
                    self.pending_key = None;
                    self.last_input = None;
                }
                ui.separator();
                ui.label(egui::RichText::new(&self.cart_label).strong());
                ui.separator();
                ui.colored_label(
                    egui::Color32::from_rgb(120, 200, 255),
                    format!("sim {}", self.sim_version),
                );
            });

            ui.horizontal_wrapped(|ui| {
                let st = self.firmware.status();
                ui.monospace(format!("PC ${:04X}", st.pc));
                ui.monospace(format!("$75={:02X}", st.key_ready));
                ui.monospace(format!("$15={:02X}", st.last_key));
                ui.monospace(format!(
                    "pending={}",
                    st.pending_raw
                        .map(|k| format!("{k:02X}"))
                        .unwrap_or_else(|| "--".into())
                ));
                ui.monospace(format!(
                    "latched={}",
                    st.latched_raw
                        .map(|k| format!("{k:02X}"))
                        .unwrap_or_else(|| "--".into())
                ));
                ui.monospace(format!(
                    "gui={}",
                    st.gui_raw
                        .map(|k| format!("{k:02X}"))
                        .unwrap_or_else(|| "--".into())
                ));
                ui.monospace(format!("armed={}", u8::from(st.gui_armed)));
                ui.monospace(format!("keys={}", st.keys_pressed));
                if let Some(ref src) = self.last_input {
                    ui.colored_label(egui::Color32::from_rgb(120, 255, 160), src);
                }
                if st.answer < 0x0A {
                    ui.monospace(format!("$35={}", st.answer));
                }
                ui.label(format!("mode={}", st.mode));
                if !st.running {
                    ui.colored_label(egui::Color32::YELLOW, "CPU paused");
                } else if st.key_pending {
                    ui.colored_label(egui::Color32::from_rgb(255, 200, 80), "key pending…");
                } else if st.needs_enter {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 200, 80),
                        format!("answer {} — press yellow ENTER", st.answer),
                    );
                } else if st.keypad_waiting {
                    ui.colored_label(egui::Color32::LIGHT_GREEN, "waiting for digit");
                } else if st.pc < 0xA000 {
                    ui.colored_label(egui::Color32::GRAY, format!("ROM boot @ ${:04X}", st.pc));
                } else {
                    ui.colored_label(egui::Color32::GRAY, format!("cart @ ${:04X}", st.pc));
                }
            });

            ui.horizontal(|ui| {
                ui.label("Answer:");
                for digit in 0u8..=9 {
                    if let Some(key) = RemoteKey::from_digit(digit) {
                        if ui.button(format!("{digit}")).clicked() {
                            queue_key(self, key, "toolbar");
                            ui.ctx().request_repaint();
                        }
                    }
                }
                if ui.button("ENTER").clicked() {
                    queue_key(self, RemoteKey::Enter, "toolbar");
                    ui.ctx().request_repaint();
                }
            });

            ui.horizontal(|ui| {
                let st = self.firmware.status();
                ui.label(format!("cycles {}", st.cycles));
                ui.separator();
                let khz = self.firmware.options.cycles_per_frame.saturating_mul(60) / 1000;
                ui.label(format!("~{khz} kHz"));
                ui.add(
                    egui::Slider::new(
                        &mut self.firmware.options.cycles_per_frame,
                        500..=super::interactive::CYCLES_PER_FRAME_REALTIME * 4,
                    )
                    .logarithmic(true)
                    .show_value(false),
                );
            });
        });

        egui::Panel::left("remote")
            .resizable(true)
            .default_size(280.0)
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if let Some(key) = remote_panel::remote_panel(ui) {
                            queue_key(self, key, "remote");
                            ui.ctx().request_repaint();
                        }
                    });
            });

        self.trace_display = self.firmware.trace_text();
        egui::Panel::bottom("cpu_trace")
            .resizable(true)
            .default_size(240.0)
            .min_size(140.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.strong("CPU trace");
                    ui.separator();
                    let enabled = self.firmware.trace_enabled();
                    let mut trace_on = enabled;
                    if ui.checkbox(&mut trace_on, "Record").changed() {
                        self.firmware.set_trace_enabled(trace_on);
                    }
                    if ui.button("Clear").clicked() {
                        self.firmware.clear_trace();
                        self.trace_display.clear();
                    }
                    if ui.button("Copy trace").clicked() {
                        ui.ctx().copy_text(self.firmware.trace_text());
                    }
                    ui.checkbox(&mut self.trace_editable, "Edit trace");
                });
                ui.separator();
                let trace_h = ui.available_height().max(60.0);
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .max_height(trace_h)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.trace_display)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY)
                                .desired_rows(8)
                                .interactive(self.trace_editable),
                        );
                    });
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical(|ui| {
                ui.heading("Robot");
                ui.label(format!("LED: [{}]", self.firmware.led_chars()));
                ui.label(
                    egui::RichText::new(
                        "Success: keys increments, then $35 shows your digit. LED [6?] is the quiz only.",
                    )
                    .small()
                    .weak(),
                );
                ui.separator();

                let tip_h = ui.spacing().interact_size.y * 2.0 + ui.spacing().item_spacing.y;
                let robot_h = (ui.available_height() - tip_h).max(80.0);
                let (rect, _resp) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), robot_h),
                    egui::Sense::hover(),
                );
                paint_live_robot(ui, rect, &self.firmware.led_chars());
            });
        });
    }
}

fn paint_live_robot(ui: &egui::Ui, rect: egui::Rect, led: &str) {
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8.0, egui::Color32::from_gray(24));

    let cx = rect.center().x;
    let base_y = rect.bottom() - 48.0;
    let body_top = base_y - 120.0;

    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(cx - 70.0, body_top),
            egui::pos2(cx + 70.0, base_y - 8.0),
        ),
        10.0,
        egui::Color32::from_rgb(70, 110, 160),
    );

    let head = egui::Rect::from_center_size(
        egui::pos2(cx, body_top + 28.0),
        egui::vec2(90.0, 32.0),
    );
    painter.rect_filled(head, 4.0, egui::Color32::from_gray(12));
    let display = if led.trim().is_empty() { "__" } else { led };
    painter.text(
        head.center(),
        egui::Align2::CENTER_CENTER,
        display,
        egui::FontId::monospace(26.0),
        egui::Color32::from_rgb(80, 255, 120),
    );

    for dx in [-42.0_f32, 42.0] {
        painter.circle_filled(egui::pos2(cx + dx, base_y), 18.0, egui::Color32::from_gray(70));
    }
}