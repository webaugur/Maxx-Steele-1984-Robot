//! On-screen Maxx Steele remote — layout from `Transmitter/Photos/Product/Remote-Front.svg`.

use eframe::egui;

use super::keypad::RemoteKey;

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

const POWER: KeyDef = KeyDef {
    key: RemoteKey::PowerStop,
    orange: None,
    indicator: None,
    home: false,
    wide: true,
};

const KEY_FACE: f32 = 16.0;
const KEY_FACE_WIDE: f32 = 20.0;
const KEY_MATRIX: f32 = 14.0;
const KEY_INSET: f32 = 3.0;
const KEY_BLACK: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
const KEY_ORANGE: egui::Color32 = egui::Color32::from_rgb(255, 136, 0);
const KEY_LABEL: egui::Color32 = egui::Color32::from_rgb(235, 235, 235);
const FRAME_FACE: egui::Color32 = egui::Color32::from_rgb(28, 28, 30);
const INSET_SHADOW: egui::Color32 = egui::Color32::from_rgb(8, 8, 10);
const INSET_HIGHLIGHT: egui::Color32 = egui::Color32::from_rgb(58, 58, 62);

/// Paint the remote and return any key pressed this frame.
pub fn remote_panel(ui: &mut egui::Ui) -> Option<RemoteKey> {
    let mut pressed = None;
    ui.heading("Remote transmitter");
    ui.label("RF link modeled as a direct wire to `$75`.");
    ui.add_space(4.0);

    let avail = ui.available_width().min(320.0);
    let bezel = 14.0;
    let key_gap = 4.0;
    let grid_w = avail - bezel * 2.0;
    let key_w = (grid_w - key_gap * 3.0) / 4.0;
    let key_h = 56.0;

    let frame_rect = egui::Frame::group(ui.style())
        .fill(FRAME_FACE)
        .inner_margin(bezel)
        .show(ui, |ui| {
            ui.set_width(avail - bezel * 2.0);
            for row in [ROW1, ROW2, ROW3, ROW4, ROW5, ROW6] {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = key_gap;
                    for def in row {
                        if let Some(k) = paint_key(ui, def, key_w, key_h) {
                            pressed = Some(k);
                        }
                    }
                });
                ui.add_space(key_gap);
            }
            if let Some(k) = paint_key(ui, POWER, grid_w, 40.0) {
                pressed = Some(k);
            }
        })
        .response
        .rect;

    if ui.is_rect_visible(frame_rect) {
        paint_frame_inset(ui, frame_rect);
    }

    pressed
}

fn paint_frame_inset(ui: &egui::Ui, rect: egui::Rect) {
    let painter = ui.painter_at(rect);
    let lip = rect.shrink(1.0);
    painter.rect_stroke(
        lip,
        6.0,
        egui::Stroke::new(1.5, INSET_SHADOW),
        egui::StrokeKind::Inside,
    );
    let inner = lip.shrink(4.0);
    painter.rect_stroke(
        inner,
        4.0,
        egui::Stroke::new(1.0, INSET_HIGHLIGHT),
        egui::StrokeKind::Inside,
    );
}

fn paint_key(ui: &mut egui::Ui, def: KeyDef, w: f32, h: f32) -> Option<RemoteKey> {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(w, h), egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);
        paint_recessed_key(&painter, rect, response.hovered(), response.is_pointer_button_down_on());

        let well = rect.shrink(KEY_INSET + 1.0);
        if let Some(digit) = def.orange {
            painter.text(
                well.center_top() + egui::vec2(0.0, 10.0),
                egui::Align2::CENTER_TOP,
                digit,
                egui::FontId::proportional(KEY_FACE),
                KEY_ORANGE,
            );
        }

        let face = if def.wide {
            def.key.label()
        } else {
            def.key.faceplate()
        };
        let face_size = if def.wide { KEY_FACE_WIDE } else { KEY_FACE };
        let face_y = if def.orange.is_some() {
            well.center().y + 2.0
        } else if def.indicator.is_some() {
            well.center().y - 4.0
        } else {
            well.center().y
        };
        painter.text(
            egui::pos2(well.center().x, face_y),
            egui::Align2::CENTER_CENTER,
            face,
            egui::FontId::proportional(face_size),
            if def.home { egui::Color32::from_rgb(255, 200, 80) } else { KEY_LABEL },
        );

        if let Some(c) = def.indicator {
            painter.circle_filled(
                well.center_bottom() + egui::vec2(0.0, -10.0),
                4.0,
                c,
            );
        }
        painter.text(
            well.right_top() + egui::vec2(-5.0, 5.0),
            egui::Align2::RIGHT_TOP,
            def.key.matrix().to_string(),
            egui::FontId::monospace(KEY_MATRIX),
            egui::Color32::from_rgb(100, 200, 255),
        );
    }

    if response.clicked() {
        Some(def.key)
    } else {
        None
    }
}

fn paint_recessed_key(
    painter: &egui::Painter,
    rect: egui::Rect,
    hovered: bool,
    pressed: bool,
) {
    let well = rect.shrink(KEY_INSET);
    let depth = if pressed { 1.0 } else { 2.0 };
    let inner = well.shrink(depth);

    painter.rect_filled(well, 4.0, INSET_SHADOW);
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