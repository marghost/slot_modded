use slot_ui::{
    classic_bezel_face, default_game_bezel_face, panel_band, panel_bezel_face, BEZEL_PANEL_H,
    BEZEL_PANEL_W, OUT_H, OUT_W,
};

const PICTURE: (f32, f32, f32, f32) = (120.0, 24.0, 600.0, 456.0);

fn alpha(rgba: &[u8], x: u32, y: u32) -> u8 {
    rgba[((y * BEZEL_PANEL_W + x) * 4 + 3) as usize]
}

fn picture_on_panel() -> (f32, f32, f32, f32) {
    let (top, h) = panel_band();
    let sx = BEZEL_PANEL_W as f32 / OUT_W as f32;
    let sy = h as f32 / OUT_H as f32;
    let (x0, y0, x1, y1) = PICTURE;
    (x0 * sx, top as f32 + y0 * sy, x1 * sx, top as f32 + y1 * sy)
}

#[test]
fn the_game_frame_sits_in_the_middle_of_a_640x480_panel() {
    assert_eq!(panel_band(), (27, 427));
}

#[test]
fn classic_bezels_fill_the_whole_panel() {
    for colour in [false, true] {
        let face = classic_bezel_face(colour, false).unwrap();
        assert_eq!(face.len(), (BEZEL_PANEL_W * BEZEL_PANEL_H * 4) as usize);
    }
}

#[test]
fn the_classic_window_lets_the_whole_game_picture_through() {
    let (x0, y0, x1, y1) = picture_on_panel();
    for colour in [false, true] {
        let face = classic_bezel_face(colour, false).unwrap();
        for y in y0.ceil() as u32..y1.floor() as u32 {
            for x in x0.ceil() as u32..x1.floor() as u32 {
                assert!(alpha(&face, x, y) < 128, "covered at {x},{y}");
            }
        }
    }
}

#[test]
fn the_classic_body_leaves_no_gap_around_the_game_picture() {
    let (x0, y0, x1, y1) = picture_on_panel();
    for colour in [false, true] {
        let face = classic_bezel_face(colour, false).unwrap();
        for y in 0..BEZEL_PANEL_H {
            for x in 0..BEZEL_PANEL_W {
                let (fx, fy) = (x as f32, y as f32);
                let outside = fx + 1.0 <= x0 || fx >= x1 || fy + 1.0 <= y0 || fy >= y1;
                if outside {
                    assert_eq!(alpha(&face, x, y), 255, "gap at {x},{y}");
                }
            }
        }
    }
}

#[test]
fn a_3_by_2_bezel_reaches_the_top_and_bottom_of_the_panel() {
    for colour in [false, true] {
        let art = default_game_bezel_face(colour).unwrap();
        let face = panel_bezel_face(&art).unwrap();
        let (top, h) = panel_band();
        for y in (0..top).chain(top + h..BEZEL_PANEL_H) {
            for x in 0..BEZEL_PANEL_W {
                assert_eq!(alpha(&face, x, y), 255, "see-through at {x},{y}");
            }
        }
    }
}

#[test]
fn the_filled_bars_carry_the_art_and_not_black() {
    let art = default_game_bezel_face(false).unwrap();
    let face = panel_bezel_face(&art).unwrap();
    let row = BEZEL_PANEL_W as usize * 4;
    let (top, h) = panel_band();
    for y in [0, top - 1, top + h, BEZEL_PANEL_H - 1] {
        let line = &face[y as usize * row..(y as usize + 1) * row];
        let lit = line
            .chunks_exact(4)
            .any(|p| p[0] > 8 || p[1] > 8 || p[2] > 8);
        assert!(lit, "row {y} is black");
    }
}

#[test]
fn the_middle_of_a_filled_bezel_is_the_art_scaled_to_the_panel() {
    let art = default_game_bezel_face(true).unwrap();
    let face = panel_bezel_face(&art).unwrap();
    let (top, h) = panel_band();
    let (cx, cy) = (OUT_W / 2, OUT_H / 2);
    let (px, py) = (BEZEL_PANEL_W / 2, top + h / 2);
    let src = &art[((cy * OUT_W + cx) * 4) as usize..][..4];
    let dst = &face[((py * BEZEL_PANEL_W + px) * 4) as usize..][..4];
    assert_eq!(src[3], dst[3]);
}

#[test]
fn the_integer_classic_window_is_the_3x_picture_and_nothing_more() {
    let (x0, y0, x1, y1) = (80, 24, 560, 456);
    for colour in [false, true] {
        let face = classic_bezel_face(colour, true).unwrap();
        assert_eq!(face.len(), (BEZEL_PANEL_W * BEZEL_PANEL_H * 4) as usize);
        for y in 0..BEZEL_PANEL_H {
            for x in 0..BEZEL_PANEL_W {
                let inside = (x0..x1).contains(&x) && (y0..y1).contains(&y);
                match inside {
                    true => assert!(alpha(&face, x, y) < 128, "covered at {x},{y}"),
                    false => assert_eq!(alpha(&face, x, y), 255, "gap at {x},{y}"),
                }
            }
        }
    }
}
