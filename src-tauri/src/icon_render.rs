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
        draw_count_corner(&mut rgba, &label);
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

    let transform = resvg::tiny_skia::Transform::from_translate(tx, ty).pre_scale(scale, scale);

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

fn draw_count_corner(buf: &mut [u8], text: &str) {
    const GLYPH_W: i32 = 6;
    const GLYPH_H: i32 = 8;
    const GLYPH_GAP: i32 = 1;
    const BADGE_LAYOUT_SCALE: i32 = 3;
    const TEXT_SCALE: i32 = 2;
    const PAD_X: i32 = 1;
    const PAD_Y: i32 = 1;
    const RADIUS: i32 = 5;
    const EDGE_INSET: i32 = 1;

    let char_count = text.chars().count() as i32;
    let layout_scale = if char_count >= 3 { 2 } else { BADGE_LAYOUT_SCALE };
    let text_scale = TEXT_SCALE;

    let badge_inner_w = char_count * GLYPH_W * layout_scale + (char_count - 1).max(0) * GLYPH_GAP * layout_scale;
    let badge_inner_h = GLYPH_H * layout_scale;
    let badge_w = badge_inner_w + PAD_X * 2;
    let badge_h = badge_inner_h + PAD_Y * 2;

    let text_w = char_count * GLYPH_W * text_scale + (char_count - 1).max(0) * GLYPH_GAP * text_scale;
    let text_h = GLYPH_H * text_scale;

    let badge_x = SIZE as i32 - EDGE_INSET - badge_w;
    let badge_y = SIZE as i32 - EDGE_INSET - badge_h;
    let text_x = badge_x + (badge_w - text_w) / 2;
    let text_y = badge_y + (badge_h - text_h) / 2;

    fill_rounded_rect(buf, badge_x, badge_y, badge_w, badge_h, RADIUS, badge_bg_color());
    draw_text(buf, text, text_x, text_y, GLYPH_W, GLYPH_GAP, text_scale, badge_text_color());
}

fn badge_bg_color() -> [u8; 4] {
    [0, 0, 0, 255]
}

#[cfg(target_os = "macos")]
fn badge_text_color() -> [u8; 4] {
    // Template icons are monochrome: punch holes so the menu bar shows through as contrast.
    [0, 0, 0, 0]
}

#[cfg(not(target_os = "macos"))]
fn badge_text_color() -> [u8; 4] {
    [255, 255, 255, 255]
}

fn fill_rounded_rect(buf: &mut [u8], x: i32, y: i32, w: i32, h: i32, radius: i32, color: [u8; 4]) {
    let radius = radius.min(w / 2).min(h / 2);
    for py in y..y + h {
        for px in x..x + w {
            if in_rounded_rect(px, py, x, y, w, h, radius) {
                set_rgba(buf, px, py, color);
            }
        }
    }
}

fn in_rounded_rect(px: i32, py: i32, rx: i32, ry: i32, w: i32, h: i32, r: i32) -> bool {
    if px < rx || py < ry || px >= rx + w || py >= ry + h {
        return false;
    }

    if px < rx + r && py < ry + r {
        let dx = px - (rx + r);
        let dy = py - (ry + r);
        return dx * dx + dy * dy <= r * r;
    }
    if px >= rx + w - r && py < ry + r {
        let dx = px - (rx + w - r - 1);
        let dy = py - (ry + r);
        return dx * dx + dy * dy <= r * r;
    }
    if px < rx + r && py >= ry + h - r {
        let dx = px - (rx + r);
        let dy = py - (ry + h - r - 1);
        return dx * dx + dy * dy <= r * r;
    }
    if px >= rx + w - r && py >= ry + h - r {
        let dx = px - (rx + w - r - 1);
        let dy = py - (ry + h - r - 1);
        return dx * dx + dy * dy <= r * r;
    }

    true
}

fn draw_text(
    buf: &mut [u8],
    text: &str,
    start_x: i32,
    start_y: i32,
    glyph_w: i32,
    glyph_gap: i32,
    scale: i32,
    color: [u8; 4],
) {
    for (idx, ch) in text.chars().enumerate() {
        let Some(glyph) = bold_glyph(ch) else {
            continue;
        };
        let ox = start_x + idx as i32 * (glyph_w + glyph_gap) * scale;
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..glyph_w {
                if (bits >> (glyph_w - 1 - col)) & 1 == 1 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            set_rgba(buf, ox + col * scale + dx, start_y + row as i32 * scale + dy, color);
                        }
                    }
                }
            }
        }
    }
}

fn set_rgba(buf: &mut [u8], x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= SIZE as i32 || y >= SIZE as i32 {
        return;
    }
    let idx = ((y as u32 * SIZE + x as u32) * 4) as usize;
    buf[idx..idx + 4].copy_from_slice(&color);
}

fn bold_glyph(ch: char) -> Option<&'static [u8; 8]> {
    Some(match ch {
        '0' => &[
            0b011110, 0b110011, 0b110101, 0b110101, 0b110101, 0b110101, 0b110011, 0b011110,
        ],
        '1' => &[
            0b001100, 0b011100, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0b111100,
        ],
        '2' => &[
            0b011110, 0b110011, 0b000011, 0b000110, 0b001100, 0b011000, 0b110000, 0b111111,
        ],
        '3' => &[
            0b011110, 0b110011, 0b000011, 0b001110, 0b000011, 0b000011, 0b110011, 0b011110,
        ],
        '4' => &[
            0b000110, 0b001110, 0b011110, 0b110110, 0b111111, 0b000110, 0b000110, 0b000110,
        ],
        '5' => &[
            0b111111, 0b110000, 0b111110, 0b000011, 0b000011, 0b000011, 0b110011, 0b011110,
        ],
        '6' => &[
            0b001110, 0b011000, 0b110000, 0b111110, 0b110011, 0b110011, 0b110011, 0b011110,
        ],
        '7' => &[
            0b111111, 0b000011, 0b000110, 0b001100, 0b011000, 0b011000, 0b011000, 0b011000,
        ],
        '8' => &[
            0b011110, 0b110011, 0b110011, 0b011110, 0b110011, 0b110011, 0b110011, 0b011110,
        ],
        '9' => &[
            0b011110, 0b110011, 0b110011, 0b110011, 0b011111, 0b000011, 0b000110, 0b011100,
        ],
        '+' => &[
            0b001100, 0b001100, 0b111111, 0b111111, 0b001100, 0b001100, 0b000000, 0b000000,
        ],
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
