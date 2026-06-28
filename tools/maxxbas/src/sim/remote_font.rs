//! Highway-style faceplate font for the on-screen remote (Overpass, OFL).

use eframe::egui;
use egui::epaint::text::{FontData, FontInsert, FontPriority, InsertFontFamily};

const FONT_DATA: &[u8] = include_bytes!("../../assets/fonts/Overpass-Regular.ttf");
pub const FAMILY: &str = "remote_highway";

/// Install Overpass (open-source Interstate highway successor) for remote key labels.
pub fn install(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        FAMILY,
        FontData::from_static(FONT_DATA),
        vec![InsertFontFamily {
            family: egui::FontFamily::Name(FAMILY.into()),
            priority: FontPriority::Highest,
        }],
    ));
}

pub fn id(size: f32) -> egui::FontId {
    egui::FontId::new(size, egui::FontFamily::Name(FAMILY.into()))
}