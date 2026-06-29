//! General UI type — Noto Sans / Noto Sans Mono (remote keeps Overpass).

use std::path::Path;

use eframe::egui;
use egui::epaint::text::{FontData, FontInsert, FontPriority, InsertFontFamily};

const NOTO_SANS: &[u8] = include_bytes!("../../assets/fonts/NotoSans-Regular.ttf");
const NOTO_MONO: &[u8] = include_bytes!("../../assets/fonts/NotoSansMono-Regular.ttf");

/// Extra Noto script fonts from the system (Cherokee, Devanagari, …).
const SCRIPT_FALLBACKS: &[(&str, &str)] = &[
    (
        "noto_cherokee",
        "/usr/share/fonts/truetype/noto/NotoSansCherokee-Regular.ttf",
    ),
    (
        "noto_devanagari",
        "/usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf",
    ),
    (
        "noto_arabic",
        "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
    ),
    (
        "noto_hebrew",
        "/usr/share/fonts/truetype/noto/NotoSansHebrew-Regular.ttf",
    ),
];

fn load_system_font(path: &str) -> Option<Vec<u8>> {
    if !Path::new(path).is_file() {
        return None;
    }
    std::fs::read(path).ok().filter(|data| !data.is_empty())
}

/// Replace egui proportional/monospace defaults with bundled Noto.
pub fn install(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "noto_sans",
        FontData::from_static(NOTO_SANS),
        vec![InsertFontFamily {
            family: egui::FontFamily::Proportional,
            priority: FontPriority::Highest,
        }],
    ));
    ctx.add_font(FontInsert::new(
        "noto_mono",
        FontData::from_static(NOTO_MONO),
        vec![InsertFontFamily {
            family: egui::FontFamily::Monospace,
            priority: FontPriority::Highest,
        }],
    ));

    for (name, path) in SCRIPT_FALLBACKS {
        let Some(data) = load_system_font(path) else {
            continue;
        };
        ctx.add_font(FontInsert::new(
            name,
            FontData::from_owned(data),
            vec![InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: FontPriority::Lowest,
            }],
        ));
    }
}