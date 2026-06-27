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

/// Paint the remote and return any key pressed this frame.
pub fn remote_panel(ui: &mut egui::Ui) -> Option<RemoteKey> {
    let mut pressed = None;
    ui.heading("Remote transmitter");
    ui.label("RF link modeled as a direct wire to `$75`.");
    ui.add_space(4.0);

    let avail = ui.available_width().min(280.0);
    let key_w = (avail - 12.0) / 4.0;
    let key_h = 40.0;

    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_gray(14))
        .inner_margin(8.0)
        .show(ui, |ui| {
            ui.set_width(avail);
            for row in [ROW1, ROW2, ROW3, ROW4, ROW5, ROW6] {
                ui.horizontal(|ui| {
                    for def in row {
                        if let Some(k) = paint_key(ui, def, key_w, key_h) {
                            pressed = Some(k);
                        }
                    }
                });
                ui.add_space(2.0);
            }
            if let Some(k) = paint_key(ui, POWER, avail - 4.0, 28.0) {
                pressed = Some(k);
            }
        });

    pressed
}

fn paint_key(ui: &mut egui::Ui, def: KeyDef, w: f32, h: f32) -> Option<RemoteKey> {
    let fill = if def.home {
        egui::Color32::from_rgb(200, 120, 0)
    } else if def.wide {
        egui::Color32::from_gray(110)
    } else {
        egui::Color32::from_gray(130)
    };

    let orange = def
        .orange
        .map(|n| format!("{n}\n"))
        .unwrap_or_default();
    let face = if def.wide {
        def.key.label()
    } else {
        def.key.faceplate()
    };
    let caption = format!("{orange}{face}");

    let response = ui.add(
        egui::Button::new(egui::RichText::new(caption).size(if def.wide { 10.0 } else { 8.0 }))
            .min_size(egui::vec2(w, h))
            .fill(fill),
    );

    if ui.is_rect_visible(response.rect) {
        let painter = ui.painter_at(response.rect);
        if let Some(c) = def.indicator {
            painter.circle_filled(
                response.rect.center_bottom() + egui::vec2(0.0, -6.0),
                3.0,
                c,
            );
        }
        painter.text(
            response.rect.right_top() + egui::vec2(-4.0, 4.0),
            egui::Align2::RIGHT_TOP,
            def.key.matrix().to_string(),
            egui::FontId::monospace(7.0),
            egui::Color32::from_rgb(100, 200, 255),
        );
    }

    if response.clicked() {
        Some(def.key)
    } else {
        None
    }
}