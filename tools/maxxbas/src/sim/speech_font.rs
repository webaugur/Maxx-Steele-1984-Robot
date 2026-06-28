//! Comic-style speech bubble font (Comic Sans MS when available).

use std::path::Path;

use eframe::egui;
use egui::epaint::text::{FontData, FontInsert, FontPriority, InsertFontFamily};

pub const FAMILY: &str = "speech_comic";

const FALLBACK: &[u8] = include_bytes!("../../assets/fonts/Overpass-Regular.ttf");

fn load_comic_sans_bytes() -> Vec<u8> {
    const CANDIDATES: &[&str] = &[
        "/usr/share/fonts/truetype/msttcorefonts/Comic_Sans_MS.ttf",
        "/usr/share/fonts/truetype/microsoft-fonts/Comic_Sans_MS.ttf",
        "/usr/share/fonts/TTF/Comic Sans MS.ttf",
        "/usr/share/fonts/truetype/comic-neue/ComicNeue-Regular.ttf",
        "C:\\Windows\\Fonts\\comic.ttf",
        "C:\\Windows\\Fonts\\COMIC.TTF",
    ];
    for path in CANDIDATES {
        if Path::new(path).is_file() {
            if let Ok(data) = std::fs::read(path) {
                if !data.is_empty() {
                    return data;
                }
            }
        }
    }
    FALLBACK.to_vec()
}

/// Install speech-bubble font (Comic Sans MS, else bundled Overpass).
pub fn install(ctx: &egui::Context) {
    let data = load_comic_sans_bytes();
    ctx.add_font(FontInsert::new(
        FAMILY,
        FontData::from_owned(data),
        vec![InsertFontFamily {
            family: egui::FontFamily::Name(FAMILY.into()),
            priority: FontPriority::Highest,
        }],
    ));
}

pub fn id(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name(FAMILY.into()))
}