use crate::draw::{Canvas, Draw, TexId, OUT_H};
use crate::plate::{arrows_hint_face, blit, hint_face, UndoFace, HINT_EDGE, LEGEND_GAP};
use crate::power_menu::MENU_INK;
use crate::text::{draw_line, label_font, line_width};

pub struct CheatPage {
    pub title: &'static str,
    pub rows: &'static [(&'static str, &'static str)],
    pub after_rule: &'static [(&'static str, &'static str)],
}

pub const CHEAT_PAGES: [CheatPage; 4] = [
    CheatPage {
        title: "CAROUSEL",
        rows: &[
            ("Left / Right", "Browse games"),
            ("Up / Down", "Jump to next letter"),
            ("L1 / R1", "Switch shelf"),
            ("Y", "Add / remove favorite"),
            ("Tap A", "Resume last state"),
            ("Hold A", "Start game fresh"),
            ("START", "Pick emulator (GBA)"),
            ("MENU", "Settings"),
        ],
        after_rule: &[],
    },
    CheatPage {
        title: "IN GAME",
        rows: &[
            ("MENU", "In-game menu"),
            ("Hold MENU", "Save state + eject"),
            ("MENU x2", "Save state switcher"),
            ("SEL+R1 / L1", "Save / load latest"),
            ("Hold L2", "Rewind"),
            ("Hold R2", "Fast-forward"),
            ("R2 x2", "Lock fast-forward"),
            ("Hold X / Y", "Turbo A / Turbo B"),
            ("SELECT+MENU", "Link screen"),
        ],
        after_rule: &[],
    },
    CheatPage {
        title: "SCREEN & SOUND",
        rows: &[
            ("SEL+Up/Down", "Brightness"),
            ("SEL+Left/Right", "Blue light"),
            ("SEL+B / A", "Prev / next shader"),
            ("SEL+Y", "Color correction"),
            ("SEL+L2 / R2", "GB palette (if on)"),
            ("L1 / R1", "GB/GBC stretch / 1:1"),
            ("VOL+ / VOL-", "Volume"),
            ("VOL+ + VOL-", "Mute"),
            ("SEL+VOL+/-", "Mute (alternate)"),
        ],
        after_rule: &[],
    },
    CheatPage {
        title: "STATE SWITCHER",
        rows: &[
            ("Left / Right", "Browse states"),
            ("A", "Load this state"),
            ("Y", "Delete this state"),
            ("X", "Undo save/load (30s)"),
            ("B", "Back to game or menu"),
            ("START", "Button cheat sheet"),
        ],
        after_rule: &[
            ("Close lid", "Save state, screen off"),
            ("Shut 3 min", "Power off, resumes"),
            ("Hold POWER", "Power off"),
        ],
    },
];

pub const CHEAT_CHIP: [u8; 3] = [0xf6, 0xf4, 0xef];
pub const CHEAT_CHIP_INK: [u8; 3] = [0x1a, 0x19, 0x17];
pub const CHEAT_INK: [u8; 3] = MENU_INK;
pub const CHEAT_DIM: [u8; 3] = [0x9a, 0x9a, 0xa4];

pub const CHEAT_TITLE_PX: f32 = 30.0;
pub const CHEAT_TEXT_PX: f32 = 22.0;
pub const CHEAT_KEY_PX: f32 = 16.0;
pub const CHEAT_CHIP_H: u32 = 26;
pub const CHEAT_CHIP_PAD: f32 = 9.0;
pub const CHEAT_GAP: f32 = 22.0;
pub const CHEAT_LEGEND_Y: u32 = 427;

const TITLE_MID: f32 = 38.0;
const DOTS_MID: f32 = 70.0;
const DOT: f32 = 7.0;
const DOT_GAP: f32 = 11.0;
const DOT_DIM: f32 = 0.35;
const FIRST_ROW: f32 = 108.0;
const PITCH: f32 = 34.0;
const RULE_DROP: f32 = 10.0;
const RULE_H: u32 = 1;
const CHIP_RADIUS: f32 = 5.0;

/// The transparent cheat sheet for a canvas `w` wide: title, page dots, a chip
/// per button with its action beside it, and the B / page legend. The menu
/// ground is drawn under it, so it reads as one more settings page.
pub fn cheat_sheet_face(page: usize, w: u32) -> UndoFace {
    let h = OUT_H;
    let mut rgba = vec![0u8; (w * h * 4) as usize];
    let page = page % CHEAT_PAGES.len();
    let sheet = &CHEAT_PAGES[page];

    let title_w = width(sheet.title, CHEAT_TITLE_PX);
    text(
        &mut rgba,
        w,
        (w as f32 - title_w) / 2.0,
        TITLE_MID,
        sheet.title,
        CHEAT_TITLE_PX,
        CHEAT_INK,
    );
    dots(&mut rgba, w, page);

    let (key_col, block) = cheat_columns();
    let left = ((w as f32 - block) / 2.0).round().max(0.0);
    let desc_x = left + key_col + CHEAT_GAP;
    let mut mid = FIRST_ROW;
    for (key, what) in sheet.rows {
        row(&mut rgba, w, left, desc_x, mid, key, what);
        mid += PITCH;
    }
    if !sheet.after_rule.is_empty() {
        let y = (mid - PITCH / 2.0 + RULE_DROP).round() as u32;
        rule(&mut rgba, w, y, left as u32, (left + block) as u32);
        mid += 2.0 * RULE_DROP;
        for (key, what) in sheet.after_rule {
            row(&mut rgba, w, left, desc_x, mid, key, what);
            mid += PITCH;
        }
    }

    legend(&mut rgba, w);
    UndoFace { rgba, w, h }
}

pub fn draw_cheat_sheet(canvas: Canvas, face: Option<TexId>, out: &mut Vec<Draw>) {
    let Some(tex) = face else {
        return;
    };
    out.push(Draw::Tex {
        x: 0.0,
        y: 0.0,
        w: canvas.w,
        h: OUT_H as f32,
        tex,
        alpha: 1.0,
    });
}

pub fn cheat_width(line: &str, px: f32) -> f32 {
    width(line, px)
}

pub fn cheat_chip_width(key: &str) -> f32 {
    (width(key, CHEAT_KEY_PX) + 2.0 * CHEAT_CHIP_PAD).ceil()
}

/// The widest chip and the whole row block over every page, so the columns
/// stay put while paging.
pub fn cheat_columns() -> (f32, f32) {
    let all = || {
        CHEAT_PAGES
            .iter()
            .flat_map(|p| p.rows.iter().chain(p.after_rule.iter()))
    };
    let key_col = all().map(|(k, _)| cheat_chip_width(k)).fold(0.0, f32::max);
    let desc = all()
        .map(|(_, d)| width(d, CHEAT_TEXT_PX))
        .fold(0.0, f32::max);
    (key_col, (key_col + CHEAT_GAP + desc).ceil())
}

fn row(rgba: &mut [u8], w: u32, left: f32, desc_x: f32, mid: f32, key: &str, what: &str) {
    chip(rgba, w, left, mid, key);
    text(rgba, w, desc_x, mid, what, CHEAT_TEXT_PX, CHEAT_INK);
}

fn chip(rgba: &mut [u8], w: u32, x: f32, mid: f32, key: &str) {
    let cw = cheat_chip_width(key);
    let top = (mid - CHEAT_CHIP_H as f32 / 2.0).round();
    let ch = CHEAT_CHIP_H as f32;
    for y in top as i32..(top + ch) as i32 {
        for xi in x as i32..(x + cw) as i32 {
            if xi < 0 || y < 0 || xi >= w as i32 || y >= OUT_H as i32 {
                continue;
            }
            let (px, py) = (xi as f32 + 0.5 - x, y as f32 + 0.5 - top);
            let dx = (CHIP_RADIUS - px).max(px - (cw - CHIP_RADIUS)).max(0.0);
            let dy = (CHIP_RADIUS - py).max(py - (ch - CHIP_RADIUS)).max(0.0);
            let cover = (CHIP_RADIUS - (dx * dx + dy * dy).sqrt() + 0.5).clamp(0.0, 1.0);
            if cover <= 0.0 {
                continue;
            }
            paint(rgba, w, xi as u32, y as u32, CHEAT_CHIP, cover);
        }
    }
    let kx = x + (cw - width(key, CHEAT_KEY_PX)) / 2.0;
    text(
        rgba,
        w,
        kx,
        top + ch / 2.0,
        key,
        CHEAT_KEY_PX,
        CHEAT_CHIP_INK,
    );
}

fn dots(rgba: &mut [u8], w: u32, page: usize) {
    let n = CHEAT_PAGES.len() as f32;
    let total = n * DOT + (n - 1.0) * DOT_GAP;
    let mut cx = (w as f32 - total) / 2.0 + DOT / 2.0;
    for i in 0..CHEAT_PAGES.len() {
        let alpha = if i == page { 1.0 } else { DOT_DIM };
        let r = DOT / 2.0;
        for y in (DOTS_MID - r - 1.0) as i32..=(DOTS_MID + r + 1.0) as i32 {
            for x in (cx - r - 1.0) as i32..=(cx + r + 1.0) as i32 {
                let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - DOTS_MID);
                let cover = (r - (dx * dx + dy * dy).sqrt() + 0.5).clamp(0.0, 1.0);
                if cover > 0.0 && x >= 0 && (x as u32) < w {
                    paint(rgba, w, x as u32, y as u32, CHEAT_INK, cover * alpha);
                }
            }
        }
        cx += DOT + DOT_GAP;
    }
}

fn legend(rgba: &mut [u8], w: u32) {
    let faces = [arrows_hint_face("Page"), hint_face("B", "Back")];
    let seen = |f: &UndoFace| f.w.saturating_sub(HINT_EDGE) as f32;
    let total = faces.iter().map(seen).sum::<f32>() + LEGEND_GAP;
    let mut x = ((w as f32 - total) / 2.0).round();
    for f in &faces {
        if x >= 0.0 && x as u32 + f.w <= w {
            blit(rgba, w, &f.rgba, f.w, f.h, x as u32, CHEAT_LEGEND_Y);
        }
        x = (x + seen(f) + LEGEND_GAP).round();
    }
}

fn paint(rgba: &mut [u8], w: u32, x: u32, y: u32, colour: [u8; 3], a: f32) {
    let i = ((y * w + x) * 4) as usize;
    let px = &mut rgba[i..i + 4];
    let under = px[3] as f32 / 255.0;
    let out = a + under * (1.0 - a);
    for c in 0..3 {
        px[c] = ((colour[c] as f32 * a + px[c] as f32 * under * (1.0 - a)) / out).round() as u8;
    }
    px[3] = (out * 255.0).round() as u8;
}

fn text(rgba: &mut [u8], w: u32, x: f32, mid: f32, line: &str, px: f32, colour: [u8; 3]) {
    let Some(font) = label_font() else { return };
    let cap = font.metrics('H', px).height as f32;
    let baseline = (mid + cap / 2.0).round();
    draw_line(rgba, w, OUT_H, x.round(), baseline, line, px, colour);
}

fn width(line: &str, px: f32) -> f32 {
    label_font().map_or(0.0, |font| line_width(font, line, px, 0.0))
}

fn rule(rgba: &mut [u8], w: u32, top: u32, x0: u32, x1: u32) {
    for y in top..top + RULE_H {
        for x in x0..x1.min(w) {
            paint(rgba, w, x, y, CHEAT_DIM, 0.6);
        }
    }
}
