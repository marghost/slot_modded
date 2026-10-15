use slot_gfx::{Draw, TexId};
use slot_power::{Battery, Charge};

use crate::footer::Printed;
use crate::hud::HUD_INK;
use crate::icon::{icon_face, Icon};
use crate::plate::{blit, word_face, UndoFace, HINT_H};

pub const BOLT_PX: f32 = 18.0;

pub const GAUGE_W: f32 = 22.0;
pub const GAUGE_H: f32 = 11.0;
const _: () = assert!(GAUGE_H < GAUGE_W);
const NUB_W: f32 = 2.5;
const NUB_H: f32 = 4.0;
pub const WALL: f32 = 1.5;
const GAP: f32 = 7.0;

const BOLT_W: f32 = 14.0;
const BOLT_H: f32 = 14.0;
const BOLT_GAP: f32 = 5.0;
const _: () = assert!(BOLT_GAP > 0.0);

pub fn draw_gauge(
    x: f32,
    y: f32,
    battery: Option<Battery>,
    percent: Printed,
    bolt: Option<TexId>,
    ink: [u8; 3],
    out: &mut Vec<Draw>,
) {
    let Some(b) = battery else {
        return;
    };
    let colour = [
        ink[0] as f32 / 255.0,
        ink[1] as f32 / 255.0,
        ink[2] as f32 / 255.0,
        1.0,
    ];

    let rect = |x: f32, y: f32, w: f32, h: f32, out: &mut Vec<Draw>| {
        out.push(Draw::Rect { x, y, w, h, colour });
    };

    let cx = x + BOLT_W + BOLT_GAP;

    rect(cx, y, GAUGE_W, WALL, out);
    rect(cx, y + GAUGE_H - WALL, GAUGE_W, WALL, out);
    rect(cx, y, WALL, GAUGE_H, out);
    rect(cx + GAUGE_W - WALL, y, WALL, GAUGE_H, out);
    rect(cx + GAUGE_W, y + (GAUGE_H - NUB_H) / 2.0, NUB_W, NUB_H, out);

    let inner = GAUGE_W - 4.0 * WALL;
    let fill = inner * f32::from(b.percent.min(100)) / 100.0;
    if fill > 0.0 {
        rect(
            cx + 2.0 * WALL,
            y + 2.0 * WALL,
            fill,
            GAUGE_H - 4.0 * WALL,
            out,
        );
    }

    if let (Charge::Charging, Some(tex)) = (b.charge, bolt) {
        out.push(Draw::Tex {
            x,
            y: y + (GAUGE_H - BOLT_H) / 2.0,
            w: BOLT_W,
            h: BOLT_H,
            tex,
            alpha: 1.0,
        });
    }

    if percent.w > 0 {
        let px = cx + GAUGE_W + NUB_W + GAP;
        let py = y + (GAUGE_H - HINT_H as f32) / 2.0;
        out.push(match percent.face {
            Some(tex) => Draw::Tex {
                x: px,
                y: py,
                w: percent.w as f32,
                h: HINT_H as f32,
                tex,
                alpha: 1.0,
            },
            None => Draw::Rect {
                x: px,
                y: py,
                w: percent.w as f32,
                h: HINT_H as f32,
                colour: [1.0, 1.0, 1.0, 0.08],
            },
        });
    }
}

pub fn level_colour(percent: u8) -> [u8; 3] {
    match percent {
        51.. => [96, 204, 96],
        26..=50 => [240, 208, 64],
        11..=25 => [245, 140, 40],
        _ => [230, 64, 52],
    }
}

pub fn battery_badge_face(battery: Battery) -> UndoFace {
    let percent = word_face(&format!("{}%", battery.percent));
    let bolt = icon_face(Icon::Charging, BOLT_PX, HUD_INK);

    let gauge_x = BOLT_W + BOLT_GAP;
    let text_x = (gauge_x + GAUGE_W + NUB_W + GAP).ceil() as u32;
    let w = text_x + percent.w;
    let (bolt_w, bolt_h) = (BOLT_W as u32, BOLT_H as u32);
    let h = HINT_H.max(bolt_h).max(GAUGE_H.ceil() as u32);
    let mut rgba = vec![0u8; (w * h * 4) as usize];

    let gy = (h as f32 - GAUGE_H) / 2.0;
    let mut rect =
        |x: f32, y: f32, rw: f32, rh: f32, ink: [u8; 3]| fill(&mut rgba, w, h, x, y, rw, rh, ink);
    rect(gauge_x, gy, GAUGE_W, WALL, HUD_INK);
    rect(gauge_x, gy + GAUGE_H - WALL, GAUGE_W, WALL, HUD_INK);
    rect(gauge_x, gy, WALL, GAUGE_H, HUD_INK);
    rect(gauge_x + GAUGE_W - WALL, gy, WALL, GAUGE_H, HUD_INK);
    rect(
        gauge_x + GAUGE_W,
        gy + (GAUGE_H - NUB_H) / 2.0,
        NUB_W,
        NUB_H,
        HUD_INK,
    );
    let inner = GAUGE_W - 4.0 * WALL;
    let level = inner * f32::from(battery.percent.min(100)) / 100.0;
    if level > 0.0 {
        rect(
            gauge_x + 2.0 * WALL,
            gy + 2.0 * WALL,
            level,
            GAUGE_H - 4.0 * WALL,
            level_colour(battery.percent),
        );
    }

    if battery.charge == Charge::Charging && bolt.w > 0 {
        let small = shrink(&bolt.rgba, bolt.w, bolt.h, bolt_w, bolt_h);
        blit(&mut rgba, w, &small, bolt_w, bolt_h, 0, (h - bolt_h) / 2);
    }
    blit(
        &mut rgba,
        w,
        &percent.rgba,
        percent.w,
        percent.h,
        text_x,
        (h - percent.h) / 2,
    );

    trim_right_bottom(UndoFace { rgba, w, h })
}

#[allow(clippy::too_many_arguments)]
fn fill(rgba: &mut [u8], w: u32, h: u32, x: f32, y: f32, rw: f32, rh: f32, ink: [u8; 3]) {
    let (x0, y0) = (x.floor().max(0.0) as u32, y.floor().max(0.0) as u32);
    let (x1, y1) = (
        ((x + rw).ceil() as u32).min(w),
        ((y + rh).ceil() as u32).min(h),
    );
    for py in y0..y1 {
        let cover_y = (y + rh).min(py as f32 + 1.0) - y.max(py as f32);
        for px in x0..x1 {
            let cover_x = (x + rw).min(px as f32 + 1.0) - x.max(px as f32);
            let a = (cover_x * cover_y * 255.0).round() as u8;
            let i = ((py * w + px) * 4) as usize;
            if a > rgba[i + 3] {
                rgba[i..i + 3].copy_from_slice(&ink);
                rgba[i + 3] = a;
            }
        }
    }
}

fn shrink(rgba: &[u8], sw: u32, sh: u32, dw: u32, dh: u32) -> Vec<u8> {
    let (fx, fy) = (sw as f32 / dw as f32, sh as f32 / dh as f32);
    let mut out = vec![0u8; (dw * dh * 4) as usize];
    for dy in 0..dh {
        let (y0, y1) = (dy as f32 * fy, (dy + 1) as f32 * fy);
        for dx in 0..dw {
            let (x0, x1) = (dx as f32 * fx, (dx + 1) as f32 * fx);
            let mut acc = [0f32; 4];
            for sy in y0.floor() as u32..(y1.ceil() as u32).min(sh) {
                let cy = y1.min(sy as f32 + 1.0) - y0.max(sy as f32);
                for sx in x0.floor() as u32..(x1.ceil() as u32).min(sw) {
                    let cover = cy * (x1.min(sx as f32 + 1.0) - x0.max(sx as f32));
                    let p = &rgba[((sy * sw + sx) * 4) as usize..][..4];
                    let a = cover * f32::from(p[3]);
                    for c in 0..3 {
                        acc[c] += a * f32::from(p[c]);
                    }
                    acc[3] += a;
                }
            }
            if acc[3] > 0.0 {
                let o = &mut out[((dy * dw + dx) * 4) as usize..][..4];
                for c in 0..3 {
                    o[c] = (acc[c] / acc[3]).round() as u8;
                }
                o[3] = (acc[3] / (fx * fy)).round().min(255.0) as u8;
            }
        }
    }
    out
}

fn trim_right_bottom(face: UndoFace) -> UndoFace {
    let inked = |x: u32, y: u32| face.rgba[((y * face.w + x) * 4 + 3) as usize] > 0;
    let w = (1..=face.w)
        .rev()
        .find(|&x| (0..face.h).any(|y| inked(x - 1, y)))
        .unwrap_or(face.w);
    let h = (1..=face.h)
        .rev()
        .find(|&y| (0..face.w).any(|x| inked(x, y - 1)))
        .unwrap_or(face.h);
    let rgba = face
        .rgba
        .chunks_exact((face.w * 4) as usize)
        .take(h as usize)
        .flat_map(|row| &row[..(w * 4) as usize])
        .copied()
        .collect();
    UndoFace { rgba, w, h }
}
