use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::{TrayStatusKind, TraySummary};
use resvg::usvg::{Options, Tree};
use resvg::tiny_skia::Pixmap;
use tauri::image::Image;

const SIZE: u32 = 44;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SvgKind {
    Idle,
    Available,
    Waiting,
    Soon,
}

static SVG_CACHE: Mutex<Option<HashMap<SvgKind, Tree>>> = Mutex::new(None);
static ICON_CACHE: Mutex<Option<HashMap<IconCacheKey, Image<'static>>>> = Mutex::new(None);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IconCacheKey {
    pub status: TrayStatusKind,
    pub available_count: u8,
}

impl IconCacheKey {
    pub fn from_summary(summary: &TraySummary) -> Self {
        let available_count = match summary.status {
            TrayStatusKind::Available => summary.available_count.min(10) as u8,
            _ => 0,
        };
        Self {
            status: summary.status.clone(),
            available_count,
        }
    }
}

pub fn render_tray_icon(summary: &TraySummary) -> Image<'static> {
    let key = IconCacheKey::from_summary(summary);
    if let Ok(cache) = ICON_CACHE.lock() {
        if let Some(map) = cache.as_ref() {
            if let Some(icon) = map.get(&key) {
                return icon.clone();
            }
        }
    }

    let icon = render_fresh(summary);
    if let Ok(mut cache) = ICON_CACHE.lock() {
        let map = cache.get_or_insert_with(HashMap::new);
        map.insert(key, icon.clone());
    }
    icon
}

fn render_fresh(summary: &TraySummary) -> Image<'static> {
    let kind = match summary.status {
        TrayStatusKind::Idle => SvgKind::Idle,
        TrayStatusKind::Available => SvgKind::Available,
        TrayStatusKind::Waiting => SvgKind::Waiting,
        TrayStatusKind::Soon => SvgKind::Soon,
    };

    let mut rgba = render_svg(kind, SIZE);

    if summary.status == TrayStatusKind::Available && summary.available_count > 0 {
        let label = if summary.available_count > 9 {
            "9+".to_string()
        } else {
            summary.available_count.to_string()
        };
        draw_badge(&mut rgba, &label);
    }

    Image::new_owned(rgba, SIZE, SIZE)
}

fn svg_source(kind: SvgKind) -> &'static str {
    match kind {
        SvgKind::Idle => include_str!("../../assets/tray-icons/idle.svg"),
        SvgKind::Available => include_str!("../../assets/tray-icons/available.svg"),
        SvgKind::Waiting => include_str!("../../assets/tray-icons/waiting.svg"),
        SvgKind::Soon => include_str!("../../assets/tray-icons/soon.svg"),
    }
}

fn render_svg(kind: SvgKind, size: u32) -> Vec<u8> {
    let tree = load_svg_tree(kind);
    let mut pixmap = Pixmap::new(size, size).expect("tray pixmap");
    pixmap.fill(resvg::tiny_skia::Color::TRANSPARENT);

    let svg_size = tree.size();
    let scale = size as f32 / svg_size.width().max(svg_size.height());
    let tx = (size as f32 - svg_size.width() * scale) / 2.0;
    let ty = (size as f32 - svg_size.height() * scale) / 2.0;

    let transform = resvg::tiny_skia::Transform::from_translate(tx, ty)
        .pre_scale(scale, scale);

    resvg::render(&tree, transform, &mut pixmap.as_mut());
    pixmap.data().to_vec()
}

fn load_svg_tree(kind: SvgKind) -> Tree {
    if let Ok(cache) = SVG_CACHE.lock() {
        if let Some(map) = cache.as_ref() {
            if let Some(tree) = map.get(&kind) {
                return tree.clone();
            }
        }
    }

    let mut options = Options::default();
    options.font_family = "system-ui".to_string();
    options.font_size = 16.0;

    let tree = Tree::from_str(svg_source(kind), &options).expect("valid tray svg");

    if let Ok(mut cache) = SVG_CACHE.lock() {
        let map = cache.get_or_insert_with(HashMap::new);
        map.insert(kind, tree.clone());
    }

    tree
}

fn draw_badge(buf: &mut [u8], text: &str) {
    let (r, g, b) = badge_color();
    fill_circle(buf, 30.0, 30.0, 9.0, [r, g, b, 255]);
    draw_text(buf, text, 30.0, 30.5);
}

fn fill_circle(buf: &mut [u8], cx: f32, cy: f32, radius: f32, color: [u8; 4]) {
    let r2 = radius * radius;
    for y in 0..SIZE as i32 {
        for x in 0..SIZE as i32 {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            if dx * dx + dy * dy <= r2 {
                set_rgba(buf, x, y, color);
            }
        }
    }
}

fn draw_text(buf: &mut [u8], text: &str, cx: f32, cy: f32) {
    const GLYPH_W: i32 = 5;
    const GLYPH_H: i32 = 7;
    let char_count = text.chars().count() as i32;
    let total_w = char_count * GLYPH_W + (char_count - 1).max(0);
    let start_x = (cx - total_w as f32 / 2.0).round() as i32;
    let start_y = (cy - GLYPH_H as f32 / 2.0).round() as i32;

    for (idx, ch) in text.chars().enumerate() {
        if let Some(glyph) = glyph(ch) {
            let ox = start_x + idx as i32 * (GLYPH_W + 1);
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..GLYPH_W {
                    if (bits >> (GLYPH_W - 1 - col)) & 1 == 1 {
                        draw_glyph_pixel(buf, ox + col, start_y + row as i32);
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn draw_glyph_pixel(buf: &mut [u8], x: i32, y: i32) {
    clear_pixel(buf, x, y);
}

#[cfg(not(target_os = "macos"))]
fn draw_glyph_pixel(buf: &mut [u8], x: i32, y: i32) {
    set_rgba(buf, x, y, [255, 255, 255, 255]);
}

#[cfg(target_os = "macos")]
fn badge_color() -> (u8, u8, u8) {
    (0, 0, 0)
}

#[cfg(not(target_os = "macos"))]
fn badge_color() -> (u8, u8, u8) {
    (52, 199, 89)
}

fn set_rgba(buf: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= SIZE as i32 || y >= SIZE as i32 {
        return;
    }
    let idx = ((y as u32 * SIZE + x as u32) * 4) as usize;
    buf[idx..idx + 4].copy_from_slice(&color);
}

fn clear_pixel(buf: &mut [u8], x: i32, y: i32) {
    set_rgba(buf, x, y, [0, 0, 0, 0]);
}

fn glyph(ch: char) -> Option<&'static [u8; 7]> {
    Some(match ch {
        '0' => &[0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => &[0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => &[0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        '3' => &[0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110],
        '4' => &[0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => &[0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
        '6' => &[0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => &[0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => &[0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => &[0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
        '+' => &[0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000, 0b00000],
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TraySummary;

    #[test]
    fn renders_all_status_icons() {
        for status in [
            TrayStatusKind::Idle,
            TrayStatusKind::Available,
            TrayStatusKind::Waiting,
            TrayStatusKind::Soon,
        ] {
            let summary = TraySummary {
                available_count: 2,
                waiting_count: 1,
                nearest_ms: Some(60_000),
                nearest_label: Some("Claude".into()),
                status,
            };
            let icon = render_tray_icon(&summary);
            assert_eq!(icon.width(), SIZE);
            assert_eq!(icon.height(), SIZE);
            assert!(icon.rgba().iter().any(|b| *b > 0));
        }
    }
}
