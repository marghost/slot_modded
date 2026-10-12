use crate::draw::{Draw, TexId, OUT_H, OUT_W};
use crate::plate::UndoFace;
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
            ("L1 / R1", "Switch platform"),
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
            ("Hold MENU", "Save state + eject"),
            ("MENU x2", "Save state switcher"),
            ("SELECT+R1", "Save state"),
            ("SELECT+L1", "Load latest state"),
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
            ("B", "Back to game"),
        ],
        after_rule: &[
            ("Close lid", "Save state, screen off"),
            ("Shut 3 min", "Power off, resumes"),
        ],
    },
];

pub const CHEAT_GROUND: [u8; 3] = [22, 24, 30];
pub const CHEAT_BAR: [u8; 3] = [52, 54, 64];
pub const CHEAT_KEY: [u8; 3] = [250, 200, 80];
pub const CHEAT_INK: [u8; 3] = [236, 236, 240];
pub const CHEAT_DIM: [u8; 3] = [150, 152, 165];

pub const CHEAT_HEADER_H: u32 = 52;
pub const CHEAT_FOOTER_H: u32 = 42;
pub const CHEAT_MARGIN: f32 = 16.0;
pub const CHEAT_DESC_X: f32 = 298.0;
pub const CHEAT_TEXT_PX: f32 = 26.0;
pub const CHEAT_TITLE_PX: f32 = 30.0;
pub const CHEAT_FOOT_PX: f32 = 24.0;
pub const CHEAT_GAP: f32 = 12.0;

const FIRST_ROW: f32 = 81.0;
const PITCH: f32 = 39.0;
const RULE_H: u32 = 2;
const FOOT_LEFT: &str = "D-pad / L R: page";
const FOOT_RIGHT: &str = "Hold MENU: exit";

pub fn cheat_sheet_face(page: usize) -> UndoFace {
    let (w, h) = (OUT_W, OUT_H);
    let mut rgba =
        [CHEAT_GROUND[0], CHEAT_GROUND[1], CHEAT_GROUND[2], 255].repeat((w * h) as usize);
    let page = page % CHEAT_PAGES.len();
    let sheet = &CHEAT_PAGES[page];

    band(&mut rgba, w, 0, CHEAT_HEADER_H, CHEAT_BAR);
    band(&mut rgba, w, h - CHEAT_FOOTER_H, CHEAT_FOOTER_H, CHEAT_BAR);

    let header_mid = CHEAT_HEADER_H as f32 / 2.0;
    text(
        &mut rgba,
        CHEAT_MARGIN,
        header_mid,
        sheet.title,
        CHEAT_TITLE_PX,
        CHEAT_INK,
    );
    let number = format!("{}/{}", page + 1, CHEAT_PAGES.len());
    let right = w as f32 - CHEAT_MARGIN - width(&number, CHEAT_TEXT_PX);
    text(
        &mut rgba,
        right,
        header_mid,
        &number,
        CHEAT_TEXT_PX,
        CHEAT_DIM,
    );

    let mut mid = FIRST_ROW;
    for (key, what) in sheet.rows {
        row(&mut rgba, mid, key, what);
        mid += PITCH;
    }
    if !sheet.after_rule.is_empty() {
        let y = (mid - PITCH / 2.0 + 12.0).round() as u32;
        rule(
            &mut rgba,
            w,
            y,
            CHEAT_MARGIN as u32,
            w - CHEAT_MARGIN as u32,
        );
        mid += PITCH / 2.0 + 12.0;
        for (key, what) in sheet.after_rule {
            row(&mut rgba, mid, key, what);
            mid += PITCH;
        }
    }

    let foot_mid = (h - CHEAT_FOOTER_H / 2) as f32;
    text(
        &mut rgba,
        2.0 * CHEAT_MARGIN,
        foot_mid,
        FOOT_LEFT,
        CHEAT_FOOT_PX,
        CHEAT_DIM,
    );
    let right = w as f32 - 2.0 * CHEAT_MARGIN - width(FOOT_RIGHT, CHEAT_FOOT_PX);
    text(
        &mut rgba,
        right,
        foot_mid,
        FOOT_RIGHT,
        CHEAT_FOOT_PX,
        CHEAT_DIM,
    );

    UndoFace { rgba, w, h }
}

pub fn draw_cheat_sheet(face: Option<TexId>, out: &mut Vec<Draw>) {
    let Some(tex) = face else {
        return;
    };
    out.push(Draw::Tex {
        x: 0.0,
        y: 0.0,
        w: OUT_W as f32,
        h: OUT_H as f32,
        tex,
        alpha: 1.0,
    });
}

pub fn cheat_width(line: &str, px: f32) -> f32 {
    width(line, px)
}

fn row(rgba: &mut [u8], mid: f32, key: &str, what: &str) {
    text(rgba, CHEAT_MARGIN, mid, key, CHEAT_TEXT_PX, CHEAT_KEY);
    text(rgba, CHEAT_DESC_X, mid, what, CHEAT_TEXT_PX, CHEAT_INK);
}

fn text(rgba: &mut [u8], x: f32, mid: f32, line: &str, px: f32, colour: [u8; 3]) {
    let Some(font) = label_font() else { return };
    let cap = font.metrics('H', px).height as f32;
    let baseline = (mid + cap / 2.0).round();
    draw_line(rgba, OUT_W, OUT_H, x, baseline, line, px, colour);
}

fn width(line: &str, px: f32) -> f32 {
    label_font().map_or(0.0, |font| line_width(font, line, px, 0.0))
}

fn band(rgba: &mut [u8], w: u32, top: u32, h: u32, colour: [u8; 3]) {
    for px in rgba[(top * w * 4) as usize..((top + h) * w * 4) as usize].chunks_exact_mut(4) {
        px.copy_from_slice(&[colour[0], colour[1], colour[2], 255]);
    }
}

fn rule(rgba: &mut [u8], w: u32, top: u32, x0: u32, x1: u32) {
    for y in top..top + RULE_H {
        for x in x0..x1 {
            let i = ((y * w + x) * 4) as usize;
            rgba[i..i + 4].copy_from_slice(&[CHEAT_BAR[0], CHEAT_BAR[1], CHEAT_BAR[2], 255]);
        }
    }
}
