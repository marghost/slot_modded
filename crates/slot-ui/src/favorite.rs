use std::path::Path;

use crate::art;
use crate::CartFace;

pub const FAVORITE_FILE: &str = "System/favorite.png";
pub const FAVORITE_PX: u32 = 112;
pub const FAVORITE_D: f32 = 39.0;

const CELL: u32 = 8;

const HEART: [&str; 12] = [
    "..KKK...KKK..",
    ".KRRRK.KRRDK.",
    "KRWWRRKRRRRDK",
    "KRWRRRRRRRRDK",
    "KRRRRRRRRRRDK",
    "KRWRRRRRRRRDK",
    ".KRRRRRRRRDK.",
    "..KRRRRRRDK..",
    "...KRRRRDK...",
    "....KRRDK....",
    ".....KDK.....",
    "......K......",
];

const OUTLINE: [u8; 4] = [24, 18, 20, 255];
const RED: [u8; 4] = [255, 24, 24, 255];
const SHINE: [u8; 4] = [255, 236, 236, 255];
const SHADE: [u8; 4] = [176, 0, 0, 255];

pub fn favorite_face(root: Option<&Path>) -> CartFace {
    root.map(|r| r.join(FAVORITE_FILE))
        .and_then(|p| art::sticker(&p, FAVORITE_PX))
        .map(|rgba| CartFace {
            rgba,
            w: FAVORITE_PX,
            h: FAVORITE_PX,
        })
        .unwrap_or_else(favorite_heart_face)
}

pub fn favorite_heart_face() -> CartFace {
    let n = FAVORITE_PX;
    let (cols, rows) = (HEART[0].len() as u32, HEART.len() as u32);
    let (x0, y0) = ((n - cols * CELL) / 2, (n - rows * CELL) / 2);
    let mut rgba = vec![0u8; (n * n * 4) as usize];
    for (row, line) in HEART.iter().enumerate() {
        for (col, cell) in line.bytes().enumerate() {
            let colour = match cell {
                b'K' => OUTLINE,
                b'R' => RED,
                b'W' => SHINE,
                b'D' => SHADE,
                _ => continue,
            };
            for dy in 0..CELL {
                for dx in 0..CELL {
                    let x = x0 + col as u32 * CELL + dx;
                    let y = y0 + row as u32 * CELL + dy;
                    let o = ((y * n + x) * 4) as usize;
                    rgba[o..o + 4].copy_from_slice(&colour);
                }
            }
        }
    }
    CartFace { rgba, w: n, h: n }
}
