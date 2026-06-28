//! Textured ABS — remote shell sampled from `Remote-Front.jpg`; window chrome at `#111`.

use eframe::egui;

/// Mean remote shell grey from photo sampling (body areas between keys).
pub const REMOTE_PLASTIC: egui::Color32 = egui::Color32::from_rgb(102, 100, 98);
pub const REMOTE_PLASTIC_DARK: egui::Color32 = egui::Color32::from_rgb(90, 82, 71);
pub const REMOTE_PLASTIC_LIGHT: egui::Color32 = egui::Color32::from_rgb(124, 108, 108);

/// Simulator chrome outside the transmitter column.
pub const WINDOW_PLASTIC: egui::Color32 = egui::Color32::from_rgb(17, 17, 17);
pub const WINDOW_PLASTIC_DARK: egui::Color32 = egui::Color32::from_rgb(10, 10, 10);
pub const WINDOW_PLASTIC_LIGHT: egui::Color32 = egui::Color32::from_rgb(28, 28, 28);

const TILE_PX: usize = 128;

#[derive(Clone, Copy)]
struct PlasticPalette {
    base: egui::Color32,
    base_rgb: [f32; 3],
    grain: f32,
    pit_deep: f32,
    pit_high: f32,
    clamp_min: f32,
    clamp_max: f32,
    seed: u32,
}

const REMOTE_PALETTE: PlasticPalette = PlasticPalette {
    base: REMOTE_PLASTIC,
    base_rgb: [102.0, 100.0, 98.0],
    grain: 16.0,
    pit_deep: -9.0,
    pit_high: 5.0,
    clamp_min: 66.0,
    clamp_max: 140.0,
    seed: 0x4D4158,
};

const WINDOW_PALETTE: PlasticPalette = PlasticPalette {
    base: WINDOW_PLASTIC,
    base_rgb: [17.0, 17.0, 17.0],
    grain: 7.0,
    pit_deep: -4.0,
    pit_high: 3.0,
    clamp_min: 8.0,
    clamp_max: 32.0,
    seed: 0x111A1A,
};

pub struct PlasticSkins {
    remote_tile: Option<egui::TextureHandle>,
    window_tile: Option<egui::TextureHandle>,
}

impl Default for PlasticSkins {
    fn default() -> Self {
        Self {
            remote_tile: None,
            window_tile: None,
        }
    }
}

impl PlasticSkins {
    pub fn remote_tile(&mut self, ctx: &egui::Context) -> &egui::TextureHandle {
        if self.remote_tile.is_none() {
            self.remote_tile = Some(build_plastic_tile(ctx, REMOTE_PALETTE, "remote_plastic_tile"));
        }
        self.remote_tile.as_ref().expect("remote tile")
    }

    pub fn window_tile(&mut self, ctx: &egui::Context) -> &egui::TextureHandle {
        if self.window_tile.is_none() {
            self.window_tile = Some(build_plastic_tile(ctx, WINDOW_PALETTE, "window_plastic_tile"));
        }
        self.window_tile.as_ref().expect("window tile")
    }
}

/// Dark `#111` panel chrome for toolbar / robot / trace (texture painted per-panel).
pub fn install_window_theme(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();
    let v = &mut style.visuals;
    v.panel_fill = WINDOW_PLASTIC;
    v.window_fill = WINDOW_PLASTIC;
    v.extreme_bg_color = WINDOW_PLASTIC_DARK;
    v.faint_bg_color = WINDOW_PLASTIC_LIGHT.gamma_multiply(0.55);
    v.widgets.noninteractive.bg_fill = WINDOW_PLASTIC;
    v.widgets.inactive.bg_fill = WINDOW_PLASTIC.gamma_multiply(1.08);
    v.widgets.open.bg_fill = WINDOW_PLASTIC_LIGHT;
    ctx.set_global_style(style);
}

pub fn paint_rect(ui: &egui::Ui, rect: egui::Rect, tile: &egui::TextureHandle) {
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter_at(rect);
    let size = tile.size_vec2();
    let uv = egui::Rect::from_min_max(
        egui::pos2(0.0, 0.0),
        egui::pos2(rect.width() / size.x, rect.height() / size.y),
    );
    painter.image(tile.id(), rect, uv, egui::Color32::WHITE);
}

/// Raised bezel around the physical remote shell (left column keypad block).
pub fn paint_remote_shell_frame(ui: &egui::Ui, rect: egui::Rect) {
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter_at(rect);
    let outer = rect.expand(2.0);
    painter.rect_stroke(
        outer,
        10.0,
        egui::Stroke::new(1.5, REMOTE_PLASTIC_DARK),
        egui::StrokeKind::Outside,
    );
    painter.rect_stroke(
        rect,
        8.0,
        egui::Stroke::new(1.0, REMOTE_PLASTIC_LIGHT.gamma_multiply(0.7)),
        egui::StrokeKind::Inside,
    );
    let shadow = rect.translate(egui::vec2(2.0, 3.0));
    painter.rect_stroke(
        shadow,
        8.0,
        egui::Stroke::new(3.0, egui::Color32::from_black_alpha(55)),
        egui::StrokeKind::Outside,
    );
}

/// Side grip ridges and bottom lip on the grey transmitter shell.
pub fn paint_remote_shell_details(ui: &egui::Ui, rect: egui::Rect) {
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter_at(rect);
    let ridge_dark = REMOTE_PLASTIC_DARK.gamma_multiply(0.85);
    let ridge_light = REMOTE_PLASTIC_LIGHT.gamma_multiply(0.55);
    let inset = 10.0;
    let top = rect.top() + inset;
    let bottom = rect.bottom() - 6.0;
    for side_x in [rect.left() + 5.0, rect.right() - 5.0] {
        for i in 0..7 {
            let t = i as f32 / 6.0;
            let y = egui::lerp(top..=bottom, t);
            let x0 = side_x - 1.0;
            let x1 = side_x + 1.0;
            painter.line_segment(
                [egui::pos2(x0, y - 5.0), egui::pos2(x0, y + 5.0)],
                egui::Stroke::new(1.0, ridge_dark),
            );
            painter.line_segment(
                [egui::pos2(x1, y - 5.0), egui::pos2(x1, y + 5.0)],
                egui::Stroke::new(0.8, ridge_light),
            );
        }
    }
    let lip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 4.0, rect.bottom() - 5.0),
        egui::pos2(rect.right() - 4.0, rect.bottom() + 1.0),
    );
    painter.rect_filled(lip, 3.0, REMOTE_PLASTIC_LIGHT.gamma_multiply(0.45));
    painter.rect_stroke(
        lip,
        3.0,
        egui::Stroke::new(0.8, REMOTE_PLASTIC_DARK.gamma_multiply(0.7)),
        egui::StrokeKind::Outside,
    );
}

fn build_plastic_tile(
    ctx: &egui::Context,
    palette: PlasticPalette,
    name: &str,
) -> egui::TextureHandle {
    let mut img = egui::ColorImage::new([TILE_PX, TILE_PX], vec![palette.base; TILE_PX * TILE_PX]);
    for y in 0..TILE_PX {
        for x in 0..TILE_PX {
            let speckle = hash01(x as u32, y as u32, palette.seed);
            let micro = hash01(x as u32, y as u32, palette.seed ^ 0x5831);
            let pit = hash01(x as u32, y as u32, palette.seed ^ 0x5050);
            let grain = (speckle - 0.5) * palette.grain + (micro - 0.5) * palette.grain * 0.45;
            let bump = if pit > 0.93 {
                palette.pit_deep
            } else if pit < 0.07 {
                palette.pit_high
            } else {
                0.0
            };
            let checker = ((x ^ y) & 1) as f32 * 1.2 - 0.6;
            let r = (palette.base_rgb[0] + grain + bump + checker)
                .clamp(palette.clamp_min, palette.clamp_max) as u8;
            let g = (palette.base_rgb[1] + grain * 0.96 + bump + checker * 0.9)
                .clamp(palette.clamp_min, palette.clamp_max) as u8;
            let b = (palette.base_rgb[2] + grain * 0.9 + bump + checker * 0.85)
                .clamp(palette.clamp_min, palette.clamp_max) as u8;
            img.pixels[y * TILE_PX + x] = egui::Color32::from_rgb(r, g, b);
        }
    }
    ctx.load_texture(
        name,
        img,
        egui::TextureOptions {
            wrap_mode: egui::TextureWrapMode::Repeat,
            ..egui::TextureOptions::LINEAR
        },
    )
}

fn hash01(x: u32, y: u32, seed: u32) -> f32 {
    let mut v = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(seed);
    v = (v ^ (v >> 13)).wrapping_mul(1274126177);
    v = v ^ (v >> 16);
    (v & 0xFFFF) as f32 / 65535.0
}