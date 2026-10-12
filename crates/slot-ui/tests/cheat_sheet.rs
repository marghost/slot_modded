use slot_ui::{
    cheat_sheet_face, cheat_width, CHEAT_BAR, CHEAT_DESC_X, CHEAT_FOOTER_H, CHEAT_GAP,
    CHEAT_GROUND, CHEAT_HEADER_H, CHEAT_KEY, CHEAT_MARGIN, CHEAT_PAGES, CHEAT_TEXT_PX, OUT_H,
    OUT_W,
};

fn at(rgba: &[u8], x: u32, y: u32) -> [u8; 3] {
    let i = ((y * OUT_W + x) * 4) as usize;
    [rgba[i], rgba[i + 1], rgba[i + 2]]
}

#[test]
fn every_line_fits_its_column() {
    for page in &CHEAT_PAGES {
        for (key, what) in page.rows.iter().chain(page.after_rule) {
            let key_end = CHEAT_MARGIN + cheat_width(key, CHEAT_TEXT_PX);
            assert!(
                key_end + CHEAT_GAP <= CHEAT_DESC_X,
                "{key:?} runs into the description column"
            );
            let what_end = CHEAT_DESC_X + cheat_width(what, CHEAT_TEXT_PX);
            assert!(
                what_end <= OUT_W as f32 - CHEAT_MARGIN,
                "{what:?} runs off the screen"
            );
        }
    }
}

#[test]
fn each_page_fills_the_screen_with_its_bars() {
    for page in 0..CHEAT_PAGES.len() {
        let face = cheat_sheet_face(page);
        assert_eq!((face.w, face.h), (OUT_W, OUT_H));
        assert!(
            face.rgba.chunks_exact(4).all(|p| p[3] == 255),
            "page {page} has holes"
        );
        assert_eq!(at(&face.rgba, 2, 2), CHEAT_BAR, "page {page}: no header");
        assert_eq!(
            at(&face.rgba, 2, OUT_H - 2),
            CHEAT_BAR,
            "page {page}: no footer"
        );
        assert_eq!(
            at(&face.rgba, 2, OUT_H / 2),
            CHEAT_GROUND,
            "page {page}: no ground"
        );
    }
}

#[test]
fn the_buttons_are_inked_in_the_key_colour_inside_the_body() {
    for page in 0..CHEAT_PAGES.len() {
        let face = cheat_sheet_face(page);
        let body = CHEAT_HEADER_H..OUT_H - CHEAT_FOOTER_H;
        let keyed = body
            .flat_map(|y| (0..CHEAT_DESC_X as u32).map(move |x| (x, y)))
            .any(|(x, y)| at(&face.rgba, x, y) == CHEAT_KEY);
        assert!(keyed, "page {page} shows no buttons");
    }
}

#[test]
fn the_pages_differ() {
    let faces: Vec<_> = (0..CHEAT_PAGES.len())
        .map(|p| cheat_sheet_face(p).rgba)
        .collect();
    for (i, a) in faces.iter().enumerate() {
        for b in &faces[i + 1..] {
            assert_ne!(a, b);
        }
    }
}
