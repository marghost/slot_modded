use slot_gfx::{OUT_H, OUT_W};

use crate::art;

const CLASSIC_GB_PNG: &[u8] = include_bytes!("../../../assets/bezel_gb_classic.png");
const CLASSIC_GBC_PNG: &[u8] = include_bytes!("../../../assets/bezel_gbc_classic.png");

pub const BEZEL_PANEL_W: u32 = 640;
pub const BEZEL_PANEL_H: u32 = 480;
const MIRROR_ROWS: usize = 8;

pub fn classic_bezel_face(colour: bool) -> Option<Vec<u8>> {
    let png = if colour {
        CLASSIC_GBC_PNG
    } else {
        CLASSIC_GB_PNG
    };
    art::cover_bytes(png, BEZEL_PANEL_W, BEZEL_PANEL_H)
}

pub fn panel_band() -> (u32, u32) {
    let (_, _, _, h) = slot_gfx::screen_rect((BEZEL_PANEL_W, BEZEL_PANEL_H));
    let bottom = (BEZEL_PANEL_H as i32 - h) / 2;
    ((BEZEL_PANEL_H as i32 - bottom - h) as u32, h as u32)
}

pub fn panel_bezel_face(rgba: &[u8]) -> Option<Vec<u8>> {
    let (top, h) = panel_band();
    let (top, h) = (top as usize, h as usize);
    let band = art::cover_rgba(rgba, OUT_W, OUT_H, BEZEL_PANEL_W, h as u32)?;
    let row = BEZEL_PANEL_W as usize * 4;
    let mut out = vec![0u8; row * BEZEL_PANEL_H as usize];
    out[top * row..(top + h) * row].copy_from_slice(&band);
    let rows = MIRROR_ROWS.min(h);
    for d in 0..top {
        let from = mirror(d, rows);
        let to = top - 1 - d;
        out.copy_within((top + from) * row..(top + from + 1) * row, to * row);
    }
    for d in 0..BEZEL_PANEL_H as usize - top - h {
        let from = top + h - 1 - mirror(d, rows);
        let to = top + h + d;
        out.copy_within(from * row..(from + 1) * row, to * row);
    }
    Some(out)
}

fn mirror(n: usize, rows: usize) -> usize {
    let i = n % (2 * rows);
    if i < rows {
        i
    } else {
        2 * rows - 1 - i
    }
}
