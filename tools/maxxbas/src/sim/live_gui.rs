//! Interactive simulator window — live firmware + remote keypad.

use std::path::Path;

use eframe::egui;
use egui::TextBuffer as _;

use super::interactive::InteractiveFirmware;
use super::keypad::RemoteKey;
use super::plastic_skin;
use super::remote_branding::{self, RemoteBranding, RemoteStatusLeds};
use super::remote_panel;
use super::speech_font;
use crate::CartImage;

const SIM_VERSION: &str = env!("CARGO_PKG_VERSION");

// IEC 60417 graphical symbols (toolbar transport controls).
const IEC_PAUSE: &str = "⏸"; // 60417-5008 — hold / pause processor
const IEC_RUN: &str = "▶"; // 60417-5014 — run / start
const IEC_RESET: &str = "⟲"; // reset / restart (common ISO equipment symbol)
const IEC_STEP_INSN: &str = "⏭"; // step one instruction
const IEC_STEP_FRAME: &str = "⏩"; // step one frame

// Trace toolbar glyphs — only U+2398 / U+239A / U+23F3 / U+23F9 / U+23FA render here
// (egui fallback font); other Misc. Technical code points show as tofu.
// IEC 60417 has no software breakpoint; ⏹ stop is the closest IEC media symbol.
const TRACE_RECORD: &str = "⏺"; // U+23FA record
const TRACE_CLEAR: &str = "⏏"; // U+23CF eject (same media cluster as ⏺/⏹/⏳)
const TRACE_COPY: &str = "⎘"; // U+2398 copy
const TRACE_FREEZE: &str = "⏳"; // U+23F3 hourglass
const TRACE_CLEAR_BP: &str = "×"; // U+00D7 (bundled Noto)
const TRACE_BREAK: &str = "⏹"; // U+23F9 IEC stop / break-on-hit

/// Square remote-style transport keys in the top toolbar.
const TOOLBAR_KEY_SIZE: f32 = 36.0;
const TOOLBAR_KEY_DEPTH: f32 = 2.0;
const TOOLBAR_ROW_GAP: f32 = 4.0;
const TOOLBAR_KEY_GLYPH: f32 = 17.0;
const TOOLBAR_KEY_FACE: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
const TOOLBAR_KEY_GLYPH_COLOR: egui::Color32 = egui::Color32::from_rgb(235, 235, 235);
const TOOLBAR_KEY_DISABLED_GLYPH: egui::Color32 = egui::Color32::from_rgb(88, 88, 92);
const TOOLBAR_KEY_DISABLED_FACE: egui::Color32 = egui::Color32::from_rgb(28, 28, 30);

/// One `step_frame` per egui logic tick while idling.
const FRAMES_PER_TICK: u32 = 1;

/// CPU frames for synchronous digest after a keypress (cart dispatch + auto-ENTER).
const KEYPRESS_DIGEST_FRAMES: u32 = 800;

/// Faster stepping while digesting a key (real-time default is too few cycles/frame).
const DIGEST_CYCLES_PER_FRAME: u64 = 16_000;

/// Full `ui()` passes before we treat the window as laid out.
const BOOT_PAINT_PASSES: u32 = 2;
/// Extra frames at rest after paint before the 6502 leaves reset.
const BOOT_IDLE_PASSES: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BootPhase {
    /// Window opening; CPU held, speech off.
    AwaitPaint,
    /// Layout painted; wait a few idle frames so the first frame is on screen.
    AwaitIdle,
    /// Normal run.
    Live,
}

#[derive(Debug, Clone, Copy)]
struct BootGate {
    phase: BootPhase,
    paint_passes: u32,
    idle_passes: u32,
}

impl BootGate {
    const fn holding() -> Self {
        Self {
            phase: BootPhase::AwaitPaint,
            paint_passes: 0,
            idle_passes: 0,
        }
    }

    fn note_ui_painted(&mut self, ctx: &egui::Context) {
        if self.phase != BootPhase::AwaitPaint {
            return;
        }
        // If egui is painting us, the window is live. Some platforms never populate
        // `inner_rect` early; do not block boot on that alone.
        let obscured = ctx.input(|input| input.viewport().visible() == Some(false));
        if obscured {
            return;
        }
        self.paint_passes += 1;
        if self.paint_passes >= BOOT_PAINT_PASSES {
            self.phase = BootPhase::AwaitIdle;
            self.idle_passes = 0;
        }
    }

    fn tick_await_idle(&mut self) -> bool {
        debug_assert_eq!(self.phase, BootPhase::AwaitIdle);
        self.idle_passes += 1;
        if self.idle_passes >= BOOT_IDLE_PASSES {
            self.phase = BootPhase::Live;
            true
        } else {
            false
        }
    }

    fn force_live(&mut self) {
        self.phase = BootPhase::Live;
        self.paint_passes = BOOT_PAINT_PASSES;
        self.idle_passes = BOOT_IDLE_PASSES;
    }

    fn holding_cpu(&self) -> bool {
        self.phase != BootPhase::Live
    }
}

pub fn run_live_gui(cart: CartImage, label: impl Into<String>) -> Result<(), String> {
    let label = label.into();
    let cart_name = short_label(&label);
    let title = format!("Maxx Steele Live v{SIM_VERSION} — {cart_name}");
    let mut fw = InteractiveFirmware::new(cart, title.clone())?;
    // Match hardware: digit then explicit ENTER (no auto-submit).
    fw.set_auto_submit_enter(false);
    // Hold the 6502 until the window has painted and idled (see `BootGate`).
    fw.set_running(false);
    fw.set_speech_enabled(false);
    let app = LiveSimApp {
        firmware: fw,
        cart_label: cart_name,
        sim_version: SIM_VERSION.to_string(),
        trace_display: String::new(),
        pending_key: None,
        last_input: None,
        trace_editable: false,
        boot_gate: BootGate::holding(),
        branding: RemoteBranding::default(),
        skins: plastic_skin::PlasticSkins::default(),
        status_leds: RemoteStatusLeds {
            tx_blink_until: 0.0,
            link_up: true,
            power_on: false,
        },
        speech_bubble: None,
        trace_breakpoint_hint: None,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 860.0])
            .with_min_inner_size([720.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(&title, options, Box::new(|cc| {
        super::ui_font::install(&cc.egui_ctx);
        super::remote_font::install(&cc.egui_ctx);
        super::emoji_font::install(&cc.egui_ctx);
        super::speech_font::install(&cc.egui_ctx);
        super::plastic_skin::install_window_theme(&cc.egui_ctx);
        Ok(Box::new(app))
    }))
        .map_err(|e| format!("GUI error: {e}"))
}

struct LiveSimApp {
    firmware: InteractiveFirmware,
    cart_label: String,
    sim_version: String,
    trace_display: String,
    pending_key: Option<RemoteKey>,
    last_input: Option<(bool, char)>,
    trace_editable: bool,
    boot_gate: BootGate,
    branding: RemoteBranding,
    skins: plastic_skin::PlasticSkins,
    status_leds: RemoteStatusLeds,
    speech_bubble: Option<SpeechBubble>,
    trace_breakpoint_hint: Option<String>,
}

#[derive(Clone)]
struct SpeechBubble {
    phrase: u8,
    text: String,
    hide_at: f64,
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

fn queue_key(app: &mut LiveSimApp, key: RemoteKey, remote: bool) {
    app.pending_key = Some(key);
    app.last_input = Some((remote, key.matrix()));
}

fn toolbar_glyph_font(size: f32) -> egui::FontId {
    egui::FontId::proportional(size)
}

fn toolbar_chip(ui: &mut egui::Ui, glyph: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 1.0;
        ui.label(egui::RichText::new(glyph).font(toolbar_glyph_font(15.0)));
        if !value.is_empty() {
            ui.label(egui::RichText::new(value).monospace().size(13.0));
        }
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ToolbarKeyMode {
    Normal { enabled: bool },
    Toggle { active: bool },
}

fn paint_toolbar_key_btn(
    ui: &mut egui::Ui,
    glyph: &str,
    tip: &str,
    mode: ToolbarKeyMode,
) -> bool {
    let enabled = match mode {
        ToolbarKeyMode::Normal { enabled } => enabled,
        ToolbarKeyMode::Toggle { .. } => true,
    };
    let sense = if enabled {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(TOOLBAR_KEY_SIZE, TOOLBAR_KEY_SIZE), sense);
    let hover_tip = if enabled {
        tip.to_string()
    } else {
        format!("{tip} (halt CPU first)")
    };
    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);
        let active = matches!(mode, ToolbarKeyMode::Toggle { active: true });
        if active {
            painter.rect_filled(rect, 3.0, TOOLBAR_KEY_GLYPH_COLOR);
            painter.rect_stroke(
                rect,
                3.0,
                egui::Stroke::new(1.0, TOOLBAR_KEY_FACE),
                egui::StrokeKind::Outside,
            );
        } else if enabled {
            paint_toolbar_recessed_key(
                &painter,
                rect,
                response.hovered(),
                response.is_pointer_button_down_on(),
            );
        } else {
            painter.rect_filled(rect, 3.0, TOOLBAR_KEY_DISABLED_FACE);
        }
        let glyph_color = if active {
            TOOLBAR_KEY_FACE
        } else if enabled {
            TOOLBAR_KEY_GLYPH_COLOR
        } else {
            TOOLBAR_KEY_DISABLED_GLYPH
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            glyph,
            toolbar_glyph_font(TOOLBAR_KEY_GLYPH),
            glyph_color,
        );
    }
    enabled && response.on_hover_text(hover_tip).clicked()
}

fn paint_toolbar_recessed_key(
    painter: &egui::Painter,
    rect: egui::Rect,
    hovered: bool,
    pressed: bool,
) {
    let depth = if pressed {
        TOOLBAR_KEY_DEPTH * 0.55
    } else {
        TOOLBAR_KEY_DEPTH
    };
    let face = rect.shrink(depth);
    painter.rect_filled(face, 3.0, TOOLBAR_KEY_FACE);
    let shadow = egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 0, 0));
    let highlight = egui::Stroke::new(
        1.0,
        if hovered {
            egui::Color32::from_rgb(72, 72, 78)
        } else {
            plastic_skin::WINDOW_PLASTIC_LIGHT
        },
    );
    painter.line_segment([face.left_top(), face.right_top()], shadow);
    painter.line_segment([face.left_top(), face.left_bottom()], shadow);
    painter.line_segment([face.left_bottom(), face.right_bottom()], highlight);
    painter.line_segment([face.right_top(), face.right_bottom()], highlight);
}

fn with_toolbar_row<R>(ui: &mut egui::Ui, body: impl FnOnce(&mut egui::Ui) -> R) -> R {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(TOOLBAR_ROW_GAP, 0.0);
        ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);
        ui.spacing_mut().interact_size = egui::vec2(TOOLBAR_KEY_SIZE, TOOLBAR_KEY_SIZE);
        body(ui)
    })
    .inner
}

fn paint_toolbar_transport_btn(
    ui: &mut egui::Ui,
    glyph: &str,
    tip: &str,
    enabled: bool,
) -> bool {
    paint_toolbar_key_btn(ui, glyph, tip, ToolbarKeyMode::Normal { enabled })
}

fn trace_selection_text(text: &str, range: egui::text::CCursorRange) -> String {
    let mut span = range.as_sorted_char_range();
    if let Some((start, end)) =
        super::trace_breakpoint::expand_trace_selection(text, range.primary.index)
    {
        span = start..end;
    }
    text.char_range(span).to_string()
}

fn paint_cpu_trace_panel(ui: &mut egui::Ui, app: &mut LiveSimApp) {
    let mut set_break_on_selection = false;
    with_toolbar_row(ui, |ui| {
        let trace_on = app.firmware.trace_enabled();
        if paint_toolbar_key_btn(
            ui,
            TRACE_RECORD,
            "Record trace",
            ToolbarKeyMode::Toggle {
                active: trace_on,
            },
        ) {
            app.firmware.set_trace_enabled(!trace_on);
        }
        if paint_toolbar_key_btn(ui, TRACE_CLEAR, "Clear trace", ToolbarKeyMode::Normal { enabled: true })
        {
            app.firmware.clear_trace();
            app.trace_display.clear();
        }
        if paint_toolbar_key_btn(ui, TRACE_COPY, "Copy trace", ToolbarKeyMode::Normal { enabled: true })
        {
            ui.ctx().copy_text(app.firmware.trace_text());
        }
        if paint_toolbar_key_btn(
            ui,
            TRACE_FREEZE,
            "Freeze trace text for manual edits",
            ToolbarKeyMode::Toggle {
                active: app.trace_editable,
            },
        ) {
            app.trace_editable = !app.trace_editable;
        }
        set_break_on_selection = paint_toolbar_key_btn(
            ui,
            TRACE_BREAK,
            "Break on selection: address ($E617), opcode (A9 00), or value (A=$02, #$0F, JSR)",
            ToolbarKeyMode::Normal { enabled: true },
        );
        if let Some(bp) = app.firmware.trace_breakpoint() {
            ui.add_space(TOOLBAR_ROW_GAP);
            toolbar_chip(ui, IEC_PAUSE, &bp.label);
            if paint_toolbar_key_btn(
                ui,
                TRACE_CLEAR_BP,
                "Clear breakpoint",
                ToolbarKeyMode::Normal { enabled: true },
            ) {
                app.firmware.set_trace_breakpoint(None);
                app.trace_breakpoint_hint = None;
            }
        }
        if let Some(hint) = &app.trace_breakpoint_hint {
            ui.colored_label(egui::Color32::from_rgb(255, 140, 80), hint);
        }
    });
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), ui.available_height().max(60.0)),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.set_min_height(ui.max_rect().height());
                    let mut output = egui::TextEdit::multiline(&mut app.trace_display)
                        .id(ui.id().with("cpu_trace_edit"))
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .interactive(true)
                        .show(ui);
                    if output.response.clicked() {
                        if let Some(range) = output.cursor_range {
                            if let Some((start, end)) = super::trace_breakpoint::expand_trace_selection(
                                &app.trace_display,
                                range.primary.index,
                            ) {
                                let expanded = egui::text::CCursorRange::two(
                                    egui::text::CCursor::new(start),
                                    egui::text::CCursor::new(end),
                                );
                                output.state.cursor.set_char_range(Some(expanded));
                                output.state.store(ui.ctx(), output.response.id);
                            }
                        }
                    }
                    if set_break_on_selection {
                        if let Some(range) = output.cursor_range {
                            let selected = trace_selection_text(&app.trace_display, range);
                            if let Some(bp) =
                                super::trace_breakpoint::parse_trace_selection(&selected)
                            {
                                app.firmware.set_trace_breakpoint(Some(bp));
                                app.trace_breakpoint_hint = None;
                            } else if !selected.trim().is_empty() {
                                app.trace_breakpoint_hint =
                                    Some(format!("Unrecognized selection: {selected}"));
                            } else {
                                app.trace_breakpoint_hint =
                                    Some("Select text in the trace first".into());
                            }
                        } else {
                            app.trace_breakpoint_hint =
                                Some("Select text in the trace first".into());
                        }
                    }
                });
        },
    );
}

fn paint_status_toolbar(
    ui: &mut egui::Ui,
    app: &LiveSimApp,
    st: &super::interactive::FirmwareStatus,
) {
    ui.spacing_mut().item_spacing.x = 6.0;
    toolbar_chip(ui, "PC", &format!("${:04X}", st.pc));
    toolbar_chip(ui, "$75", &format!("{:02X}", st.key_ready));
    toolbar_chip(ui, "$15", &format!("{:02X}", st.last_key));
    if let Some(k) = st.pending_raw {
        toolbar_chip(ui, "pnd", &format!("{k:02X}"));
    }
    if let Some(k) = st.latched_raw {
        toolbar_chip(ui, "lat", &format!("{k:02X}"));
    }
    if let Some(k) = st.gui_raw {
        toolbar_chip(ui, "gui", &format!("{k:02X}"));
    }
    if st.gui_armed {
        toolbar_chip(ui, "arm", "1");
    }
    if st.keys_pressed > 0 {
        toolbar_chip(ui, "keys", &st.keys_pressed.to_string());
    }
    if let Some((remote, key)) = app.last_input {
        toolbar_chip(ui, if remote { "RF" } else { "KY" }, &key.to_string());
    }
    if st.answer < 0x0A {
        toolbar_chip(ui, "$35", &st.answer.to_string());
    }
    paint_live_status_chip(ui, app, st);
}

fn update_speech_bubble(app: &mut LiveSimApp, now: f64) {
    let st = app.firmware.status();
    if let Some(phrase) = st.speech_phrase {
        let fresh = app.speech_bubble.as_ref().map(|b| b.phrase) != Some(phrase);
        if fresh {
            let dur = super::speech::phrase_duration_secs(phrase);
            let text = super::speech::phrase_label(phrase)
                .unwrap_or("…")
                .to_string();
            app.speech_bubble = Some(SpeechBubble {
                phrase,
                text,
                hide_at: now + dur + 1.0,
            });
        }
    }
    if app
        .speech_bubble
        .as_ref()
        .is_some_and(|b| now >= b.hide_at)
    {
        app.speech_bubble = None;
    }
}

fn deliver_key(app: &mut LiveSimApp, now: f64) {
    let saved_cpf = app.firmware.options.cycles_per_frame;
    app.firmware.options.cycles_per_frame = DIGEST_CYCLES_PER_FRAME;
    if let Some(key) = app.pending_key.take() {
        app.firmware.press_key(key);
        app.status_leds.note_transmit(now);
    }
    app.firmware.digest_keypress(KEYPRESS_DIGEST_FRAMES);
    app.firmware.options.cycles_per_frame = saved_cpf;
    app.trace_display = app.firmware.trace_text();
}

fn release_boot_cpu(app: &mut LiveSimApp) {
    app.firmware.set_speech_enabled(true);
    app.firmware.set_running(true);
}

impl eframe::App for LiveSimApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.boot_gate.holding_cpu() {
            match self.boot_gate.phase {
                BootPhase::AwaitPaint => {}
                BootPhase::AwaitIdle => {
                    if self.boot_gate.tick_await_idle() {
                        release_boot_cpu(self);
                    } else {
                        ctx.request_repaint_after(std::time::Duration::from_millis(16));
                    }
                }
                BootPhase::Live => unreachable!("holding_cpu inconsistent with Live"),
            }
            if self.boot_gate.phase == BootPhase::AwaitPaint {
                ctx.request_repaint();
            } else if self.boot_gate.phase == BootPhase::AwaitIdle {
                ctx.request_repaint_after(std::time::Duration::from_millis(16));
            }
            if self.pending_key.is_some() {
                let now = ctx.input(|i| i.time);
                deliver_key(self, now);
            }
            return;
        }

        self.status_leds.power_on = self.firmware.status().running;

        if self.pending_key.is_none() {
            if let Some(key) = poll_keyboard(ctx) {
                queue_key(self, key, false);
            }
        }

        if self.pending_key.is_some() {
            let now = ctx.input(|i| i.time);
            deliver_key(self, now);
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
        let now = ui.input(|i| i.time);
        update_speech_bubble(self, now);

        egui::Panel::top("toolbar")
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(6, 3),
                ..Default::default()
            })
            .show_inside(ui, |ui| {
            let window_tile = self.skins.window_tile(ui.ctx());
            plastic_skin::paint_rect(ui, ui.clip_rect(), window_tile);

            with_toolbar_row(ui, |ui| {
                let st = self.firmware.status();
                let (halt_glyph, halt_tip) = if st.running {
                    (IEC_PAUSE, "Halt CPU (IEC 60417-5008 pause)")
                } else {
                    (IEC_RUN, "Run CPU (IEC 60417-5014 start)")
                };
                let can_step = st.running == false && !self.boot_gate.holding_cpu();
                if paint_toolbar_transport_btn(ui, halt_glyph, halt_tip, true) {
                    self.firmware.set_running(!st.running);
                }
                if paint_toolbar_transport_btn(ui, IEC_STEP_INSN, "Step one instruction", can_step) {
                    self.firmware.step_instruction_halted();
                    self.trace_display = self.firmware.trace_text();
                    ui.ctx().request_repaint();
                }
                if paint_toolbar_transport_btn(ui, IEC_STEP_FRAME, "Step one frame", can_step) {
                    self.firmware.step_frame_halted();
                    self.trace_display = self.firmware.trace_text();
                    ui.ctx().request_repaint();
                }
                if paint_toolbar_transport_btn(ui, IEC_RESET, "Reset CPU (power-on restart)", true) {
                    let _ = self.firmware.reset();
                    self.pending_key = None;
                    self.last_input = None;
                    self.speech_bubble = None;
                    if self.boot_gate.holding_cpu() {
                        self.boot_gate.force_live();
                        release_boot_cpu(self);
                    }
                }
                ui.separator();
                toolbar_chip(ui, "⚙", &st.cycles.to_string());
                ui.separator();
                ui.add(
                    egui::Slider::new(
                        &mut self.firmware.options.cycles_per_frame,
                        500..=super::interactive::CYCLES_PER_FRAME_REALTIME * 4,
                    )
                    .logarithmic(true)
                    .show_value(false),
                );
                ui.separator();
                ui.label(egui::RichText::new(&self.cart_label).strong());

                let remaining = ui.available_width();
                if remaining > 40.0 {
                    ui.allocate_ui_with_layout(
                        egui::vec2(remaining, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Max),
                        |ui| {
                            ui.horizontal(|ui| {
                                paint_status_toolbar(ui, self, &st);
                            });
                        },
                    );
                }
            });

            });

        if !self.trace_editable {
            self.trace_display = self.firmware.trace_text();
        }

        egui::Panel::left("remote")
            .resizable(false)
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO,
                ..Default::default()
            })
            .default_width(remote_panel::REMOTE_PANEL_W)
            .width_range(remote_panel::REMOTE_PANEL_W..=remote_panel::REMOTE_PANEL_W)
            .show_inside(ui, |ui| {
                ui.set_min_width(remote_panel::REMOTE_PANEL_W);
                ui.set_max_width(remote_panel::REMOTE_PANEL_W);
                let window_tile = self.skins.window_tile(ui.ctx());
                plastic_skin::paint_rect(ui, ui.clip_rect(), window_tile);
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(remote_panel::REMOTE_SHELL_W);
                        let now = ui.input(|i| i.time);
                        let (key, _shell) = remote_panel::paint_transmitter_face(
                            ui,
                            &mut self.skins,
                            &self.status_leds,
                            now,
                        );
                        if let Some(key) = key {
                            queue_key(self, key, true);
                            ui.ctx().request_repaint();
                        }
                        remote_branding::paint_logo(ui, &mut self.branding);
                    });
            });

        egui::Panel::left("cpu_trace")
            .resizable(false)
            .frame(egui::Frame {
                inner_margin: egui::Margin::symmetric(6, 3),
                ..Default::default()
            })
            .default_width(remote_panel::REMOTE_PANEL_W)
            .width_range(remote_panel::REMOTE_PANEL_W..=remote_panel::REMOTE_PANEL_W)
            .show_inside(ui, |ui| {
                ui.set_min_width(remote_panel::REMOTE_PANEL_W);
                ui.set_max_width(remote_panel::REMOTE_PANEL_W);
                let window_tile = self.skins.window_tile(ui.ctx());
                plastic_skin::paint_rect(ui, ui.clip_rect(), window_tile);
                paint_cpu_trace_panel(ui, self);
            });

        egui::CentralPanel::default()
            .frame(egui::Frame {
                inner_margin: egui::Margin::ZERO,
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                let window_tile = self.skins.window_tile(ui.ctx());
                plastic_skin::paint_rect(ui, ui.clip_rect(), window_tile);
                let (rect, _resp) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), ui.available_height().max(80.0)),
                    egui::Sense::hover(),
                );
                paint_live_robot(
                    ui,
                    rect,
                    &self.firmware.led_chars_settled(),
                    self.speech_bubble.as_ref(),
                );
            });

        self.boot_gate.note_ui_painted(ui.ctx());
    }
}

fn paint_live_status_chip(ui: &mut egui::Ui, app: &LiveSimApp, st: &super::interactive::FirmwareStatus) {
    if app.boot_gate.holding_cpu() {
        let icon = match app.boot_gate.phase {
            BootPhase::AwaitPaint => "□",
            BootPhase::AwaitIdle => "⋯",
            BootPhase::Live => "",
        };
        if !icon.is_empty() {
            ui.colored_label(
                egui::Color32::from_rgb(255, 180, 60),
                egui::RichText::new(icon).font(toolbar_glyph_font(15.0)),
            );
        }
    } else if !st.running {
        ui.colored_label(
            egui::Color32::YELLOW,
            egui::RichText::new(IEC_PAUSE).font(toolbar_glyph_font(15.0)),
        );
    } else if st.key_pending {
        ui.colored_label(
            egui::Color32::from_rgb(255, 200, 80),
            egui::RichText::new("KEY").font(toolbar_glyph_font(13.0)),
        );
    } else if st.needs_enter {
        toolbar_chip(ui, "ENT", &st.answer.to_string());
    } else if st.keypad_waiting {
        ui.colored_label(
            egui::Color32::LIGHT_GREEN,
            egui::RichText::new("WAIT").font(toolbar_glyph_font(13.0)),
        );
    } else if st.pc < 0xA000 {
        toolbar_chip(ui, "PC", &format!("${:04X}", st.pc));
    }
}

fn paint_live_robot(ui: &egui::Ui, rect: egui::Rect, led: &str, speech: Option<&SpeechBubble>) {
    const DESIGN_W: f32 = 360.0;
    const DESIGN_H: f32 = 240.0;

    if rect.width() < 8.0 || rect.height() < 8.0 || !ui.is_rect_visible(rect) {
        return;
    }

    let scale = (rect.width() / DESIGN_W).min(rect.height() / DESIGN_H);
    let drawn = egui::vec2(DESIGN_W * scale, DESIGN_H * scale);
    let origin = rect.center() - drawn * 0.5;
    let s = |v: f32| v * scale;

    let painter = ui.painter_at(rect).with_clip_rect(rect);
    let cx = origin.x + DESIGN_W * 0.5 * scale;
    let base_y = origin.y + (DESIGN_H - 48.0) * scale;
    let body_top = base_y - s(120.0);

    painter.rect_filled(
        egui::Rect::from_min_max(
            egui::pos2(cx - s(70.0), body_top),
            egui::pos2(cx + s(70.0), base_y - s(8.0)),
        ),
        s(10.0),
        egui::Color32::from_rgb(70, 110, 160),
    );

    let head = egui::Rect::from_center_size(
        egui::pos2(cx, body_top + s(28.0)),
        egui::vec2(s(90.0), s(32.0)),
    );
    painter.rect_filled(head, s(4.0), egui::Color32::from_gray(12));
    let display = if led.trim().is_empty() { "__" } else { led };
    painter.text(
        head.center(),
        egui::Align2::CENTER_CENTER,
        display,
        egui::FontId::monospace(s(26.0)),
        egui::Color32::from_rgb(80, 255, 120),
    );

    for dx in [-42.0_f32, 42.0] {
        painter.circle_filled(egui::pos2(cx + s(dx), base_y), s(18.0), egui::Color32::from_gray(70));
    }

    if let Some(bubble) = speech {
        paint_speech_bubble(&painter, head, &bubble.text, rect, scale);
    }
}

fn paint_speech_bubble(
    painter: &egui::Painter,
    head: egui::Rect,
    text: &str,
    bounds: egui::Rect,
    scale: f32,
) {
    let font = speech_font::id(17.0 * scale);
    let wrap_w = 220.0 * scale;
    let galley = painter.layout(
        text.to_owned(),
        font,
        egui::Color32::BLACK,
        wrap_w,
    );
    let pad = egui::vec2(14.0 * scale, 10.0 * scale);
    let bubble_size = galley.size() + pad * 2.0;
    let prefer_right = head.right_top() + egui::vec2(10.0 * scale, -bubble_size.y - 8.0 * scale);
    let prefer_left =
        head.left_top() + egui::vec2(-bubble_size.x - 10.0 * scale, -bubble_size.y - 8.0 * scale);
    let right_rect = egui::Rect::from_min_size(prefer_right, bubble_size);
    let left_rect = egui::Rect::from_min_size(prefer_left, bubble_size);
    let anchor = if right_rect.right() <= bounds.right() {
        prefer_right
    } else if left_rect.left() >= bounds.left() {
        prefer_left
    } else {
        egui::pos2(
            (bounds.right() - bubble_size.x).max(bounds.left()),
            prefer_right.y.max(bounds.top()),
        )
    };
    let bubble_rect = egui::Rect::from_min_size(anchor, bubble_size).intersect(bounds);
    let shadow_rect = bubble_rect.translate(egui::vec2(4.0 * scale, 5.0 * scale));
    let corner = 12.0 * scale;

    painter.rect_filled(shadow_rect, corner, egui::Color32::from_black_alpha(90));
    painter.rect_filled(bubble_rect, corner, egui::Color32::from_rgb(255, 255, 252));
    painter.rect_stroke(
        bubble_rect,
        corner,
        egui::Stroke::new(2.0 * scale, egui::Color32::BLACK),
        egui::StrokeKind::Outside,
    );
    painter.galley(bubble_rect.min + pad, galley, egui::Color32::BLACK);

    let tail_on_right = bubble_rect.left() >= head.right() - 1.0;
    let tail_base = if tail_on_right {
        egui::pos2(bubble_rect.left() + 18.0 * scale, bubble_rect.bottom() - 2.0 * scale)
    } else {
        egui::pos2(bubble_rect.right() - 18.0 * scale, bubble_rect.bottom() - 2.0 * scale)
    };
    let tail_tip = if tail_on_right {
        head.right_center() + egui::vec2(4.0 * scale, 0.0)
    } else {
        head.left_center() + egui::vec2(-4.0 * scale, 0.0)
    };
    painter.add(egui::Shape::convex_polygon(
        vec![
            tail_base,
            tail_base + egui::vec2(14.0 * scale * if tail_on_right { 1.0 } else { -1.0 }, 0.0),
            tail_tip,
        ],
        egui::Color32::from_rgb(255, 255, 252),
        egui::Stroke::new(2.0 * scale, egui::Color32::BLACK),
    ));
}