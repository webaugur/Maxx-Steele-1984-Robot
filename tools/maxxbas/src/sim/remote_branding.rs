//! MAXX logo and transmitter status LEDs (CR701 TX, link, power).

use eframe::egui;

use super::emoji_font;
use super::plastic_skin;

const LOGO_SVG: &[u8] = include_bytes!("../../assets/Maxx-Steele-MAXX-Logo.svg");
const TX_LED_BLINK_SECS: f64 = 0.14;
/// Transmitter shell width on the physical CR701 remote (mm).
const REMOTE_FACE_WIDTH_MM: f32 = 92.0;
pub const LED_DIAMETER_MM: f32 = 7.0;

pub fn led_diameter_px(shell_w_px: f32) -> f32 {
    LED_DIAMETER_MM * (shell_w_px / REMOTE_FACE_WIDTH_MM)
}

pub fn status_led_strip_h(shell_w_px: f32) -> f32 {
    led_diameter_px(shell_w_px) + 14.0
}

const LED_HOLE_PAD: f32 = 3.5;
const LED_SLOT_GAP: f32 = 12.0;
const LED_EDGE_MARGIN: f32 = 10.0;

pub const LOGO_GAP_BELOW_SHELL: f32 = 5.0;
pub const LOGO_PAD_ABOVE: f32 = 10.0;

pub struct RemoteBranding {
    logo: Option<egui::TextureHandle>,
    logo_error: Option<String>,
}

impl Default for RemoteBranding {
    fn default() -> Self {
        Self {
            logo: None,
            logo_error: None,

        }
    }
}

pub struct RemoteStatusLeds {
    pub tx_blink_until: f64,
    pub link_up: bool,
    pub power_on: bool,
}

impl RemoteStatusLeds {
    pub fn note_transmit(&mut self, now: f64) {
        self.tx_blink_until = now + TX_LED_BLINK_SECS;
    }

    pub fn tx_active(&self, now: f64) -> bool {
        now < self.tx_blink_until
    }
}

/// Status strip above the remote pad — ⚡ 🔗 📡 right to left on the grey shell.
pub fn paint_status_leds(ui: &mut egui::Ui, leds: &RemoteStatusLeds, now: f64) {
    let width = ui.available_width();
    let led_d = led_diameter_px(width);
    let strip_h = status_led_strip_h(width);
    let (strip_rect, _) =
        ui.allocate_exact_size(egui::vec2(width, strip_h), egui::Sense::hover());
    if !ui.is_rect_visible(strip_rect) {
        return;
    }

    let painter = ui.painter_at(strip_rect);
    let cy = strip_rect.center().y;
    let hole_r = led_d * 0.5 + LED_HOLE_PAD;
    let slot_pitch = hole_r * 2.0 + LED_SLOT_GAP;
    let slots: [(&str, egui::Color32, bool); 3] = [
        ("📡", egui::Color32::from_rgb(255, 40, 30), leds.tx_active(now)),
        ("🔗", egui::Color32::from_rgb(60, 220, 90), leds.link_up),
        ("⚡", egui::Color32::from_rgb(255, 180, 40), leds.power_on),
    ];

    let mut x = strip_rect.right() - LED_EDGE_MARGIN - hole_r;
    for (emoji, color, on) in slots {
        let center = egui::pos2(x, cy);
        paint_led_bevel_hole(&painter, center, hole_r, led_d * 0.5, color, on);
        painter.text(
            center + egui::vec2(-hole_r - 5.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            emoji,
            emoji_font::id(13.0),
            egui::Color32::WHITE,
        );
        x -= slot_pitch;
    }
}

/// Recessed round well in the remote plastic with a beveled lip (no black backing bar).
fn paint_led_bevel_hole(
    painter: &egui::Painter,
    center: egui::Pos2,
    well_r: f32,
    lens_r: f32,
    color: egui::Color32,
    on: bool,
) {
    let floor = plastic_skin::REMOTE_PLASTIC_DARK.gamma_multiply(0.92);
    let shadow = plastic_skin::REMOTE_PLASTIC_DARK;
    let highlight = plastic_skin::REMOTE_PLASTIC_LIGHT.gamma_multiply(0.75);

    painter.circle_filled(center, well_r, floor);

    let lip = well_r - 1.2;
    painter.circle_stroke(center, well_r, egui::Stroke::new(1.4, shadow));
    painter.circle_stroke(center, lip, egui::Stroke::new(1.0, highlight));

    let n = 28;
    for i in 0..n {
        let t0 = i as f32 / n as f32;
        let t1 = (i + 1) as f32 / n as f32;
        let a0 = std::f32::consts::FRAC_PI_2 + t0 * std::f32::consts::TAU;
        let a1 = std::f32::consts::FRAC_PI_2 + t1 * std::f32::consts::TAU;
        let p0 = center + egui::vec2(a0.cos(), a0.sin()) * lip;
        let p1 = center + egui::vec2(a1.cos(), a1.sin()) * lip;
        let upper = a0.sin() < 0.0 && a0.cos() < 0.0;
        let stroke = if upper {
            egui::Stroke::new(1.1, shadow.gamma_multiply(0.9))
        } else if a0.sin() > 0.0 && a0.cos() > 0.0 {
            egui::Stroke::new(0.9, highlight)
        } else {
            continue;
        };
        painter.line_segment([p0, p1], stroke);
    }

    if on {
        let lit = lens_r * 0.78;
        painter.circle_filled(center, lit, color);
        painter.circle_filled(center, lit, egui::Color32::from_white_alpha(40));
    } else {
        painter.circle_filled(center, lens_r * 0.64, color.gamma_multiply(0.14));
    }
}

/// MAXX logo under the remote keypad.
pub fn paint_logo(ui: &mut egui::Ui, branding: &mut RemoteBranding) {
    if branding.logo.is_none() && branding.logo_error.is_none() {
        match rasterize_logo(ui) {
            Ok(texture) => branding.logo = Some(texture),
            Err(e) => branding.logo_error = Some(e),
        }
    }

    ui.add_space(LOGO_GAP_BELOW_SHELL);
    ui.add_space(LOGO_PAD_ABOVE);
    if let Some(texture) = &branding.logo {
        let width = ui.available_width().min(320.0);
        let aspect = texture.size_vec2().y / texture.size_vec2().x;
        let height = width * aspect;
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.image((texture.id(), egui::vec2(width, height)));
        });
    } else if let Some(err) = &branding.logo_error {
        ui.colored_label(egui::Color32::YELLOW, format!("logo: {err}"));
    }
}

fn rasterize_logo(ui: &egui::Ui) -> Result<egui::TextureHandle, String> {
    let tree = resvg::usvg::Tree::from_data(LOGO_SVG, &resvg::usvg::Options::default())
        .map_err(|e| format!("{e}"))?;
    let size = tree.size();
    if size.width() <= 0.0 || size.height() <= 0.0 {
        return Err("empty SVG bounds".into());
    }

    let target_w = ui.available_width().min(320.0);
    let scale = (target_w / size.width()).clamp(0.5, 4.0);
    let w = (size.width() * scale).round().max(1.0) as u32;
    let h = (size.height() * scale).round().max(1.0) as u32;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(w, h).ok_or("pixmap alloc failed")?;
    pixmap.fill(resvg::tiny_skia::Color::TRANSPARENT);
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Leave room for rainbow stroke (~4.4 SVG units) so edges are not clipped.
    let stroke_pad = (4.5 * scale).ceil().max(3.0) as u32;
    let (cropped, cw, ch) = trim_alpha_bounds(&pixmap, stroke_pad).ok_or("empty logo bounds")?;
    let image = egui::ColorImage::from_rgba_unmultiplied([cw as usize, ch as usize], &cropped);
    Ok(ui.ctx().load_texture(
        "maxx_logo_rainbow",
        image,
        egui::TextureOptions::LINEAR,
    ))
}

fn trim_alpha_bounds(
    pixmap: &resvg::tiny_skia::Pixmap,
    pad: u32,
) -> Option<(Vec<u8>, u32, u32)> {
    let w = pixmap.width();
    let h = pixmap.height();
    let data = pixmap.data();
    let mut min_x = w;
    let mut min_y = h;
    let mut max_x = 0u32;
    let mut max_y = 0u32;
    let mut any = false;

    for y in 0..h {
        for x in 0..w {
            let a = data[((y * w + x) * 4 + 3) as usize];
            if a > 12 {
                any = true;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }
    if !any {
        return None;
    }

    min_x = min_x.saturating_sub(pad);
    min_y = min_y.saturating_sub(pad);
    max_x = (max_x + pad).min(w - 1);
    max_y = (max_y + pad).min(h - 1);

    let cw = max_x - min_x + 1;
    let ch = max_y - min_y + 1;
    let mut out = vec![0u8; (cw * ch * 4) as usize];
    for y in 0..ch {
        for x in 0..cw {
            let src = ((min_y + y) * w + (min_x + x)) as usize * 4;
            let dst = (y * cw + x) as usize * 4;
            out[dst..dst + 4].copy_from_slice(&data[src..src + 4]);
        }
    }
    Some((out, cw, ch))
}