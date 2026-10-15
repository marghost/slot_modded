use slot_ui::{
    cheat_columns, cheat_sheet_face, CHEAT_CHIP, CHEAT_CHIP_H, CHEAT_LEGEND_Y, CHEAT_PAGES, OUT_H,
    OUT_W,
};

const NATIVE_W: u32 = 640;

fn at(rgba: &[u8], w: u32, x: u32, y: u32) -> [u8; 4] {
    let i = ((y * w + x) * 4) as usize;
    [rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3]]
}

#[test]
fn every_row_fits_the_native_and_framed_canvas() {
    let (_, block) = cheat_columns();
    assert!(block + 32.0 <= NATIVE_W as f32, "rows are {block} px wide");
    assert!(block + 32.0 <= OUT_W as f32);
}

#[test]
fn every_page_fits_above_the_legend() {
    for page in &CHEAT_PAGES {
        let rows = page.rows.len() + page.after_rule.len();
        assert!(rows <= 9, "{} has {rows} rows", page.title);
    }
    assert!(108.0 + 8.0 * 34.0 + CHEAT_CHIP_H as f32 / 2.0 < CHEAT_LEGEND_Y as f32);
}

#[test]
fn the_sheet_is_see_through_so_the_menu_ground_shows() {
    for w in [NATIVE_W, OUT_W] {
        let face = cheat_sheet_face(0, w);
        assert_eq!((face.w, face.h), (w, OUT_H));
        assert_eq!(at(&face.rgba, w, 2, 2)[3], 0);
        assert_eq!(at(&face.rgba, w, w - 3, OUT_H - 3)[3], 0);
    }
}

#[test]
fn every_page_shows_button_chips() {
    for page in 0..CHEAT_PAGES.len() {
        let face = cheat_sheet_face(page, NATIVE_W);
        let chips = (90..CHEAT_LEGEND_Y)
            .flat_map(|y| (0..NATIVE_W / 2).map(move |x| (x, y)))
            .filter(|&(x, y)| {
                at(&face.rgba, NATIVE_W, x, y) == [CHEAT_CHIP[0], CHEAT_CHIP[1], CHEAT_CHIP[2], 255]
            })
            .count();
        assert!(chips > 500, "page {page} shows no buttons");
    }
}

#[test]
fn the_pages_differ() {
    let faces: Vec<_> = (0..CHEAT_PAGES.len())
        .map(|p| cheat_sheet_face(p, NATIVE_W).rgba)
        .collect();
    for (i, a) in faces.iter().enumerate() {
        for b in &faces[i + 1..] {
            assert_ne!(a, b);
        }
    }
}
