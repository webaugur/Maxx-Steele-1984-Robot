//! Emoji rendering for toolbar / status chips (Noto Color Emoji when available).

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use eframe::egui;
use egui::epaint::text::{FontData, FontInsert, FontPriority, InsertFontFamily};

pub const FAMILY: &str = "toolbar_emoji";

static INSTALLED: AtomicBool = AtomicBool::new(false);

const SYSTEM_CANDIDATES: &[&str] = &[
    "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
    "/usr/share/fonts/noto/NotoColorEmoji.ttf",
    "/usr/share/fonts/truetype/noto/NotoEmoji-Regular.ttf",
    "/usr/share/fonts/opentype/noto/NotoColorEmoji.ttf",
    "/usr/share/fonts/truetype/joypixels/JoyPixels.ttf",
    "C:\\Windows\\Fonts\\seguiemj.ttf",
    "C:\\Windows\\Fonts\\SegoeUIEmoji.ttf",
];

fn load_emoji_bytes() -> Option<Vec<u8>> {
    for path in SYSTEM_CANDIDATES {
        if Path::new(path).is_file() {
            if let Ok(data) = std::fs::read(path) {
                if !data.is_empty() {
                    return Some(data);
                }
            }
        }
    }
    None
}

/// Install emoji font for toolbar glyphs; also registers fallback on default families.
pub fn install(ctx: &egui::Context) {
    let Some(data) = load_emoji_bytes() else {
        return;
    };
    let fallback = vec![
        InsertFontFamily {
            family: egui::FontFamily::Proportional,
            priority: FontPriority::Lowest,
        },
        InsertFontFamily {
            family: egui::FontFamily::Monospace,
            priority: FontPriority::Lowest,
        },
    ];
    ctx.add_font(FontInsert::new(
        FAMILY,
        FontData::from_owned(data.clone()),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Name(FAMILY.into()),
                priority: FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: FontPriority::Lowest,
            },
        ],
    ));
    ctx.add_font(FontInsert::new(
        "toolbar_emoji_fallback",
        FontData::from_owned(data),
        fallback,
    ));
    INSTALLED.store(true, Ordering::Relaxed);
}

pub fn id(size: f32) -> egui::FontId {
    if INSTALLED.load(Ordering::Relaxed) {
        egui::FontId::new(size, egui::FontFamily::Name(FAMILY.into()))
    } else {
        egui::FontId::proportional(size)
    }
}

pub fn rich_emoji(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text).font(id(15.0))
}

pub fn rich_emoji_btn(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text).font(id(16.0))
}

/// Paint emoji through the galley path (required for Noto Color Emoji; `painter.text` fails).
pub fn paint_centered(
    painter: &egui::Painter,
    rect: egui::Rect,
    text: &str,
    size: f32,
) {
    let font = id(size);
    // Color emoji fonts ignore tint; WHITE keeps bitmap glyphs visible.
    let galley = painter.layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE);
    let pos = rect.center() - galley.size() * 0.5;
    painter.galley(pos, galley, egui::Color32::WHITE);
}