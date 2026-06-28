//! On-screen Maxx Steele remote — layout from `Transmitter/Photos/Product/Remote-Front.svg`.

use eframe::egui;

use super::keypad::RemoteKey;
use super::plastic_skin::{self, PlasticSkins};
use super::remote_font;

#[derive(Clone, Copy)]
struct KeyDef {
    key: RemoteKey,
    orange: Option<&'static str>,
    indicator: Option<egui::Color32>,
    home: bool,
    wide: bool,
}

const ROW1: [KeyDef; 4] = [
    KeyDef {
        key: RemoteKey::DriveU,
        orange: Some("U"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Drive1,
        orange: Some("1"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Drive2,
        orange: Some("2"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Drive3,
        orange: Some("3"),
        indicator: None,
        home: false,
        wide: false,
    },
];

const ROW2: [KeyDef; 4] = [
    KeyDef {
        key: RemoteKey::Wrist4,
        orange: Some("4"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Wrist5,
        orange: Some("5"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Arms6,
        orange: Some("6"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Arms7,
        orange: Some("7"),
        indicator: None,
        home: false,
        wide: false,
    },
];

const ROW3: [KeyDef; 4] = [
    KeyDef {
        key: RemoteKey::Claw8,
        orange: Some("8"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Claw9,
        orange: Some("9"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::LampA,
        orange: Some("A"),
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::HomeB,
        orange: Some("B"),
        indicator: None,
        home: true,
        wide: false,
    },
];

const ROW4: [KeyDef; 4] = [
    KeyDef {
        key: RemoteKey::Wait,
        orange: None,
        indicator: None,
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::ShiftOctave,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Clear,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(255, 136, 0)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Enter,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(255, 204, 0)),
        home: false,
        wide: false,
    },
];

const ROW5: [KeyDef; 4] = [
    KeyDef {
        key: RemoteKey::SongNotes,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::ClockStatus,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Speech,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Motion,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
];

const ROW6: [KeyDef; 4] = [
    KeyDef {
        key: RemoteKey::Game,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Program,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Learn,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
    KeyDef {
        key: RemoteKey::Execute,
        orange: None,
        indicator: Some(egui::Color32::from_rgb(0, 102, 255)),
        home: false,
        wide: false,
    },
];

/// Faceplate captions (DRIVE, CLEAR, NOTE REST, …).
const KEY_TEXT: f32 = 10.0;
/// Orange digit keys (U, 1–9, A, B).
const KEY_ORANGE_MARK: f32 = 13.0;
/// Blue matrix letters (corner codes).
const KEY_MATRIX_MARK: f32 = 12.0;
const KEY_MATRIX_Y_UP: f32 = 5.0;
/// Nudge faceplate captions (DRIVE, CLEAR, …) slightly below center.
const KEY_FACE_Y_NUDGE: f32 = 5.0;
/// Shift all key lettering down together.
const KEY_TEXT_Y_SHIFT: f32 = 5.0;
const KEY_MATRIX_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 200, 255);
const KEY_INSET: f32 = 5.5;
const KEY_RECESS_DEPTH: f32 = 3.5;
const KEY_W: f32 = 77.0;
const KEY_H: f32 = 64.0;
const KEY_H_POWER: f32 = 46.0;
const KEY_BLACK: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
const KEY_ORANGE: egui::Color32 = egui::Color32::from_rgb(255, 136, 0);
const KEY_LABEL: egui::Color32 = egui::Color32::from_rgb(235, 235, 235);
const POWER_KEY_TEXT: f32 = KEY_TEXT + 2.0;
const POWER_KEY_LABEL: egui::Color32 = egui::Color32::from_rgb(210, 28, 24);
const INSET_SHADOW: egui::Color32 = plastic_skin::REMOTE_PLASTIC_DARK;
const INSET_HIGHLIGHT: egui::Color32 = plastic_skin::REMOTE_PLASTIC_LIGHT;
const KEY_GRID_ROWS: usize = 6;

/// Grey plastic lip outside the raised button-box frame.
const REMOTE_BEZEL: f32 = 6.0;
const REMOTE_KEY_GAP: f32 = 3.0;
/// Interior padding between the button-box frame and the key grid.
const BUTTON_BOX_PAD: f32 = 2.0;

const BUTTON_BOX_W: f32 =
    KEY_W * 4.0 + REMOTE_KEY_GAP * 3.0 + BUTTON_BOX_PAD * 2.0;
/// Fixed transmitter shell width (not user-resizable).
pub const REMOTE_SHELL_W: f32 = BUTTON_BOX_W + REMOTE_BEZEL * 2.0;
/// Left panel width — matches shell exactly (no extra slack).
pub const REMOTE_PANEL_W: f32 = REMOTE_SHELL_W;

/// Keypad grid height including inner bezel (not the RF LED strip).
pub fn remote_keypad_shell_height() -> f32 {
    let grid_h = KEY_H * KEY_GRID_ROWS as f32 + REMOTE_KEY_GAP * (KEY_GRID_ROWS as f32 - 1.0);
    grid_h + KEY_H_POWER + REMOTE_BEZEL * 2.0
}

/// Full transmitter face: RF LED strip flush on top, keypad frame directly below.
pub fn paint_transmitter_face(
    ui: &mut egui::Ui,
    skins: &mut PlasticSkins,
    leds: &super::remote_branding::RemoteStatusLeds,
    now: f64,
) -> (Option<RemoteKey>, egui::Rect) {
    let led_h = super::remote_branding::status_led_strip_h(REMOTE_SHELL_W);
    let shell_h = led_h + remote_keypad_shell_height();
    let (shell_rect, _) =
        ui.allocate_exact_size(egui::vec2(REMOTE_SHELL_W, shell_h), egui::Sense::hover());
    let tile = skins.remote_tile(ui.ctx());
    plastic_skin::paint_rect(ui, shell_rect, tile);

    let led_rect = egui::Rect::from_min_size(shell_rect.min, egui::vec2(shell_rect.width(), led_h));
    let keypad_rect = egui::Rect::from_min_max(
        egui::pos2(shell_rect.left(), shell_rect.top() + led_h),
        shell_rect.right_bottom(),
    );

    let mut pressed = None;
    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(led_rect), |ui| {
        super::remote_branding::paint_status_leds(ui, leds, now);
    });
    ui.allocate_new_ui(egui::UiBuilder::new().max_rect(keypad_rect), |ui| {
        pressed = paint_remote_keypad(ui);
    });

    if ui.is_rect_visible(shell_rect) {
        plastic_skin::paint_remote_shell_details(ui, shell_rect);
        plastic_skin::paint_remote_shell_frame(ui, shell_rect);
        paint_led_keypad_seam(ui, led_rect, keypad_rect);
    }

    (pressed, shell_rect)
}

/// Key grid only — caller provides the remote-grey shell behind this.
pub fn paint_remote_keypad(ui: &mut egui::Ui) -> Option<RemoteKey> {
    let mut pressed = None;
    let pad_rect = ui.available_rect_before_wrap();
    let box_left = pad_rect.left() + REMOTE_BEZEL;
    let grid_h = KEY_H * KEY_GRID_ROWS as f32 + REMOTE_KEY_GAP * (KEY_GRID_ROWS as f32 - 1.0);
    let well_rect = egui::Rect::from_min_size(
        egui::pos2(box_left, pad_rect.top() + REMOTE_BEZEL),
        egui::vec2(BUTTON_BOX_W, grid_h + BUTTON_BOX_PAD * 2.0),
    );
    let power_rect = egui::Rect::from_min_max(
        egui::pos2(well_rect.left(), well_rect.bottom()),
        egui::pos2(well_rect.right(), well_rect.bottom() + KEY_H_POWER),
    );
    let total_h = well_rect.height() + KEY_H_POWER;
    ui.allocate_exact_size(egui::vec2(BUTTON_BOX_W, total_h), egui::Sense::hover());

    let key_area = well_rect.shrink(BUTTON_BOX_PAD);
    let mut y = key_area.top();
    for row in [ROW1, ROW2, ROW3, ROW4, ROW5, ROW6] {
        for (col, def) in row.iter().enumerate() {
            let x = key_area.left() + col as f32 * (KEY_W + REMOTE_KEY_GAP);
            let rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(KEY_W, KEY_H));
            if let Some(k) = paint_key_at(ui, *def, rect) {
                pressed = Some(k);
            }
        }
        y += KEY_H + REMOTE_KEY_GAP;
    }

    if let Some(k) = paint_power_key_at(ui, power_rect) {
        pressed = Some(k);
    }

    if ui.is_rect_visible(well_rect) {
        paint_button_box_frame(ui, well_rect.union(power_rect));
    }

    pressed
}

fn paint_button_box_frame(ui: &egui::Ui, rect: egui::Rect) {
    let painter = ui.painter_at(rect);
    painter.rect_stroke(
        rect,
        4.0,
        egui::Stroke::new(1.5, INSET_SHADOW),
        egui::StrokeKind::Inside,
    );
    let inner = rect.shrink(1.0);
    painter.rect_stroke(
        inner,
        3.0,
        egui::Stroke::new(1.0, INSET_HIGHLIGHT.gamma_multiply(0.85)),
        egui::StrokeKind::Inside,
    );
}

fn paint_key_at(ui: &mut egui::Ui, def: KeyDef, rect: egui::Rect) -> Option<RemoteKey> {
    let response = ui.allocate_rect(rect, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);
        paint_recessed_key(
            &painter,
            rect,
            response.hovered(),
            response.is_pointer_button_down_on(),
        );
        paint_key_labels(&painter, def, rect);
    }
    if response.clicked() {
        Some(def.key)
    } else {
        None
    }
}

fn paint_power_key_at(ui: &mut egui::Ui, rect: egui::Rect) -> Option<RemoteKey> {
    let response = ui.allocate_rect(rect, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        paint_raised_power_key(ui, rect, response.hovered(), response.is_pointer_button_down_on());
        paint_bold_text(
            &ui.painter_at(rect),
            rect.center(),
            "POWER/STOP",
            POWER_KEY_TEXT,
            POWER_KEY_LABEL,
        );
    }
    if response.clicked() {
        Some(RemoteKey::PowerStop)
    } else {
        None
    }
}

fn paint_led_keypad_seam(ui: &egui::Ui, led_rect: egui::Rect, keypad_rect: egui::Rect) {
    if !ui.is_rect_visible(led_rect) {
        return;
    }
    let painter = ui.painter_at(led_rect.union(keypad_rect));
    let y = keypad_rect.top();
    painter.line_segment(
        [
            egui::pos2(led_rect.left() + 6.0, y),
            egui::pos2(led_rect.right() - 6.0, y),
        ],
        egui::Stroke::new(0.8, INSET_SHADOW.gamma_multiply(0.55)),
    );
}

fn paint_key_labels(painter: &egui::Painter, def: KeyDef, rect: egui::Rect) {
    let well = rect.shrink(KEY_INSET + 1.0);
    if let Some(digit) = def.orange {
        painter.text(
            well.center(),
            egui::Align2::CENTER_CENTER,
            digit,
            remote_font::id(KEY_ORANGE_MARK),
            KEY_ORANGE,
        );
    }

    let face = if def.wide {
        def.key.label()
    } else {
        def.key.faceplate()
    };
    let face_y = if def.orange.is_some() {
        well.center().y + 2.0 + KEY_FACE_Y_NUDGE + KEY_TEXT_Y_SHIFT
    } else if def.indicator.is_some() {
        well.center().y - 4.0 + KEY_FACE_Y_NUDGE + KEY_TEXT_Y_SHIFT
    } else if def.key == RemoteKey::Wait {
        well.center().y + KEY_FACE_Y_NUDGE - 2.0 + KEY_TEXT_Y_SHIFT
    } else {
        well.center().y + KEY_FACE_Y_NUDGE + KEY_TEXT_Y_SHIFT
    };
    painter.text(
        egui::pos2(well.center().x, face_y),
        egui::Align2::CENTER_CENTER,
        face,
        remote_font::id(KEY_TEXT),
        if def.home {
            egui::Color32::from_rgb(255, 200, 80)
        } else {
            KEY_LABEL
        },
    );

    if let Some(c) = def.indicator {
        painter.circle_filled(well.center_bottom() + egui::vec2(0.0, -10.0), 4.0, c);
    }
    painter.text(
        well.right_top() + egui::vec2(-5.0, 5.0 + KEY_TEXT_Y_SHIFT - KEY_MATRIX_Y_UP),
        egui::Align2::RIGHT_TOP,
        def.key.matrix().to_string(),
        remote_font::id(KEY_MATRIX_MARK),
        KEY_MATRIX_COLOR,
    );
}

fn paint_bold_text(
    painter: &egui::Painter,
    center: egui::Pos2,
    text: &str,
    size: f32,
    color: egui::Color32,
) {
    let font = remote_font::id(size);
    for dx in [0.0, 0.7, -0.7] {
        painter.text(
            center + egui::vec2(dx, 0.0),
            egui::Align2::CENTER_CENTER,
            text,
            font.clone(),
            color,
        );
    }
}

fn paint_raised_power_key(ui: &egui::Ui, rect: egui::Rect, hovered: bool, pressed: bool) {
    let painter = ui.painter_at(rect);
    let face = if pressed {
        rect.shrink(1.5)
    } else if hovered {
        rect.shrink(0.5)
    } else {
        rect
    };
    painter.rect_filled(face, 3.0, KEY_BLACK);
    let shadow = egui::Stroke::new(1.2, plastic_skin::REMOTE_PLASTIC_DARK);
    let highlight = egui::Stroke::new(1.0, egui::Color32::from_rgb(72, 72, 78));
    painter.line_segment([face.left_top(), face.right_top()], highlight);
    painter.line_segment([face.left_top(), face.left_bottom()], highlight);
    painter.line_segment([face.left_bottom(), face.right_bottom()], shadow);
    painter.line_segment([face.right_top(), face.right_bottom()], shadow);
}

fn paint_recessed_key(
    painter: &egui::Painter,
    rect: egui::Rect,
    hovered: bool,
    pressed: bool,
) {
    let well = rect.shrink(KEY_INSET);
    let depth = if pressed {
        KEY_RECESS_DEPTH * 0.55
    } else {
        KEY_RECESS_DEPTH
    };
    let inner = well.shrink(depth);

    painter.rect_filled(inner, 3.0, KEY_BLACK);

    let shadow = egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 0, 0));
    let highlight = egui::Stroke::new(
        1.0,
        if hovered {
            egui::Color32::from_rgb(72, 72, 78)
        } else {
            INSET_HIGHLIGHT
        },
    );

    // Recessed lip: dark top/left, lighter bottom/right.
    painter.line_segment([inner.left_top(), inner.right_top()], shadow);
    painter.line_segment([inner.left_top(), inner.left_bottom()], shadow);
    painter.line_segment([inner.left_bottom(), inner.right_bottom()], highlight);
    painter.line_segment([inner.right_top(), inner.right_bottom()], highlight);
}