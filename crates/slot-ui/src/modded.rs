use slot_gfx::{Canvas, Draw, TexId, OUT_H};

use crate::sticker::STICKER_H;
use crate::text;
use crate::UndoFace;

const AVATAR_PNG: &[u8] = include_bytes!("../assets/avatar.png");

pub const MODDED_TAG: &str = "_modded";
pub const MODDED_VERSION: &str = "v1.6.0-mod.4.0";
pub const MODDED_LINK: &str = "github.com/marghost";
pub const BADGE_W: u32 = 240;
pub const BADGE_H: u32 = 122;

const AVATAR_D: f32 = 88.0;
const AVATAR_Y: f32 = 46.0;
const TEXT_PX: f32 = 20.0;
const LINK_PX: f32 = 15.0;
const LINK_BASELINE: f32 = 118.0;
const TURN: f32 = -25.0;
const TAG_SHIFT: (f32, f32) = (24.0, 18.0);
const EDGE_PX: i32 = 2;
const INK: [u8; 3] = [244, 245, 249];
const EDGE: [u8; 3] = [11, 14, 17];
const VERSION_PX: f32 = 18.0;
const VERSION_INK: [u8; 3] = [0x9a, 0x9a, 0xa4];
const VERSION_MARGIN: f32 = 16.0;

pub fn modded_badge_face() -> UndoFace {
    let (w, h) = (BADGE_W, BADGE_H);
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    if let Some((avatar, aw, ah)) = avatar() {
        draw_avatar(&mut rgba, w, h, &avatar, aw, ah);
    }
    let centre = (w as f32 / 2.0 + TAG_SHIFT.0, AVATAR_Y + TAG_SHIFT.1);
    if let Some((ink, tw, th)) = coverage(MODDED_TAG, TEXT_PX) {
        let edge = dilate(&ink, tw, th, EDGE_PX);
        draw_turned(&mut rgba, w, h, &edge, tw, th, centre, TURN, EDGE);
        draw_turned(&mut rgba, w, h, &ink, tw, th, centre, TURN, INK);
    }
    if let Some((ink, tw, th)) = coverage(MODDED_LINK, LINK_PX) {
        let middle = LINK_BASELINE - link_middle_drop();
        draw_turned(
            &mut rgba,
            w,
            h,
            &ink,
            tw,
            th,
            (w as f32 / 2.0, middle),
            0.0,
            INK,
        );
    }
    UndoFace { rgba, w, h }
}

/// The full release tag, bottom right of the About page.
pub fn version_face() -> UndoFace {
    let Some((cov, w, h)) = coverage(MODDED_VERSION, VERSION_PX) else {
        return UndoFace {
            rgba: Vec::new(),
            w: 0,
            h: 0,
        };
    };
    let rgba = cov
        .iter()
        .flat_map(|&a| [VERSION_INK[0], VERSION_INK[1], VERSION_INK[2], a])
        .collect();
    UndoFace { rgba, w, h }
}

pub fn draw_modded_badge(
    canvas: Canvas,
    face: Option<TexId>,
    version: Option<(TexId, u32, u32)>,
    out: &mut Vec<Draw>,
) {
    if let Some((tex, w, h)) = version {
        out.push(Draw::Tex {
            x: (canvas.w - VERSION_MARGIN - w as f32).round(),
            y: (OUT_H as f32 - VERSION_MARGIN - h as f32).round(),
            w: w as f32,
            h: h as f32,
            tex,
            alpha: 1.0,
        });
    }
    let Some(tex) = face else {
        return;
    };
    let below = (OUT_H as f32 + STICKER_H as f32) / 2.0;
    out.push(Draw::Tex {
        x: canvas.centred(BADGE_W as f32).round(),
        y: below + (OUT_H as f32 - below - BADGE_H as f32) / 2.0,
        w: BADGE_W as f32,
        h: BADGE_H as f32,
        tex,
        alpha: 1.0,
    });
}

fn avatar() -> Option<(Vec<u8>, u32, u32)> {
    let mut dec = png::Decoder::new(std::io::Cursor::new(AVATAR_PNG));
    dec.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = dec.read_info().ok()?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).ok()?;
    let channels = match info.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        _ => return None,
    };
    let rgb = buf[..info.buffer_size()]
        .chunks(channels)
        .flat_map(|p| [p[0], p[1], p[2]])
        .collect();
    Some((rgb, info.width, info.height))
}

fn link_middle_drop() -> f32 {
    text::label_font()
        .and_then(|f| f.horizontal_line_metrics(LINK_PX))
        .map_or(0.0, |m| m.ascent - m.new_line_size / 2.0)
}

fn draw_avatar(dst: &mut [u8], w: u32, h: u32, src: &[u8], sw: u32, sh: u32) {
    let (cx, cy, r) = (w as f32 / 2.0, AVATAR_Y, AVATAR_D / 2.0);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
            let cover = (r - (dx * dx + dy * dy).sqrt() + 0.5).clamp(0.0, 1.0);
            if cover <= 0.0 {
                continue;
            }
            let u = ((dx + r) / AVATAR_D * sw as f32).clamp(0.0, sw as f32 - 1.0) as u32;
            let v = ((dy + r) / AVATAR_D * sh as f32).clamp(0.0, sh as f32 - 1.0) as u32;
            let s = ((v * sw + u) * 3) as usize;
            let o = ((y * w + x) * 4) as usize;
            dst[o..o + 3].copy_from_slice(&src[s..s + 3]);
            dst[o + 3] = (cover * 255.0).round() as u8;
        }
    }
}

fn coverage(line: &str, px: f32) -> Option<(Vec<u8>, u32, u32)> {
    let font = text::label_font()?;
    let metrics = font.horizontal_line_metrics(px)?;
    let pad = EDGE_PX as f32 + 2.0;
    let width: f32 = line
        .chars()
        .map(|c| font.metrics(c, px).advance_width)
        .sum();
    let (tw, th) = (
        (width + 2.0 * pad).ceil() as u32,
        (metrics.new_line_size + 2.0 * pad).ceil() as u32,
    );
    let mut cov = vec![0u8; (tw * th) as usize];
    let baseline = pad + metrics.ascent;
    let mut pen = pad;
    for ch in line.chars() {
        let (m, glyph) = font.rasterize(ch, px);
        let left = (pen + m.xmin as f32).round() as i32;
        let top = (baseline - (m.height as f32 + m.ymin as f32)).round() as i32;
        for gy in 0..m.height {
            for gx in 0..m.width {
                let (x, y) = (left + gx as i32, top + gy as i32);
                if x < 0 || y < 0 || x >= tw as i32 || y >= th as i32 {
                    continue;
                }
                let at = (y as u32 * tw + x as u32) as usize;
                cov[at] = cov[at].max(glyph[gy * m.width + gx]);
            }
        }
        pen += m.advance_width;
    }
    Some((cov, tw, th))
}

fn dilate(cov: &[u8], w: u32, h: u32, by: i32) -> Vec<u8> {
    let mut out = vec![0u8; cov.len()];
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let mut most = 0u8;
            for dy in -by..=by {
                for dx in -by..=by {
                    if dx * dx + dy * dy > by * by + 1 {
                        continue;
                    }
                    let (sx, sy) = (x + dx, y + dy);
                    if sx >= 0 && sy >= 0 && sx < w as i32 && sy < h as i32 {
                        most = most.max(cov[(sy as u32 * w + sx as u32) as usize]);
                    }
                }
            }
            out[(y as u32 * w + x as u32) as usize] = most;
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn draw_turned(
    dst: &mut [u8],
    w: u32,
    h: u32,
    cov: &[u8],
    tw: u32,
    th: u32,
    (cx, cy): (f32, f32),
    turn: f32,
    colour: [u8; 3],
) {
    let (sin, cos) = turn.to_radians().sin_cos();
    let (tcx, tcy) = (tw as f32 / 2.0, th as f32 / 2.0);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
            let u = dx * cos + dy * sin + tcx - 0.5;
            let v = -dx * sin + dy * cos + tcy - 0.5;
            let a = sample(cov, tw, th, u, v);
            if a <= 0.0 {
                continue;
            }
            let o = ((y * w + x) * 4) as usize;
            let under = dst[o + 3] as f32 / 255.0;
            let out_a = a + under * (1.0 - a);
            for c in 0..3 {
                let mixed = colour[c] as f32 * a + dst[o + c] as f32 * under * (1.0 - a);
                dst[o + c] = (mixed / out_a).round() as u8;
            }
            dst[o + 3] = (out_a * 255.0).round() as u8;
        }
    }
}

fn sample(cov: &[u8], w: u32, h: u32, u: f32, v: f32) -> f32 {
    let (x0, y0) = (u.floor(), v.floor());
    let (fx, fy) = (u - x0, v - y0);
    let at = |x: f32, y: f32| -> f32 {
        if x < 0.0 || y < 0.0 || x >= w as f32 || y >= h as f32 {
            return 0.0;
        }
        cov[(y as u32 * w + x as u32) as usize] as f32 / 255.0
    };
    let top = at(x0, y0) * (1.0 - fx) + at(x0 + 1.0, y0) * fx;
    let bottom = at(x0, y0 + 1.0) * (1.0 - fx) + at(x0 + 1.0, y0 + 1.0) * fx;
    top * (1.0 - fy) + bottom * fy
}
