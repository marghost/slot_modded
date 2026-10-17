use slot_ui::{
    draw_modded_badge, modded_badge_face, version_face, Canvas, Draw, TexId, BADGE_H, BADGE_W,
    MOUTH_H, OUT_H, STICKER_H,
};

fn pixel(face: &slot_ui::UndoFace, x: u32, y: u32) -> [u8; 4] {
    let o = ((y * face.w + x) * 4) as usize;
    [
        face.rgba[o],
        face.rgba[o + 1],
        face.rgba[o + 2],
        face.rgba[o + 3],
    ]
}

#[test]
fn the_badge_is_a_round_picture_with_the_tag_across_it() {
    let face = modded_badge_face();
    assert_eq!((face.w, face.h), (BADGE_W, BADGE_H));
    assert_eq!(
        pixel(&face, 0, 0)[3],
        0,
        "a corner of the badge is not clear"
    );
    assert_eq!(
        pixel(&face, BADGE_W / 2 - 40, 6)[3],
        0,
        "the picture is not round"
    );
    let picture = pixel(&face, BADGE_W / 2 - 20, 66);
    assert_eq!(picture[3], 255, "no picture inside the circle");
    let light = face
        .rgba
        .chunks(4)
        .filter(|p| p[3] == 255 && p[0] > 235 && p[1] > 235 && p[2] > 235)
        .count();
    assert!(light > 300, "the tag is missing: {light} light pixels");
}

#[test]
fn the_badge_sits_centred_between_the_sticker_and_the_bottom() {
    for canvas in [Canvas::FRAMED, Canvas { w: 640.0, h: 480.0 }] {
        let mut out = Vec::new();
        draw_modded_badge(canvas, Some(TexId::from_raw(7)), None, &mut out);
        let [Draw::Tex { x, y, w, h, .. }] = out[..] else {
            panic!("{out:?}");
        };
        assert_eq!(x + w / 2.0, canvas.w / 2.0);
        assert!(
            y >= (OUT_H + STICKER_H) as f32 / 2.0,
            "the badge covers the sticker"
        );
        assert!(y + h <= OUT_H as f32, "the badge runs off the screen");
    }
}

#[test]
fn the_version_sits_right_above_the_restart_legend() {
    let face = version_face();
    assert!(face.w > 0 && face.rgba.chunks(4).any(|p| p[3] > 200));
    let canvas = Canvas { w: 640.0, h: 480.0 };
    let mut out = Vec::new();
    draw_modded_badge(
        canvas,
        None,
        Some((TexId::from_raw(8), face.w, face.h)),
        &mut out,
    );
    let [Draw::Tex { x, y, w, h, .. }] = out[..] else {
        panic!("{out:?}");
    };
    assert!(
        x + w <= canvas.w && x + w >= canvas.w - 24.0,
        "not at the right edge"
    );
    assert!(
        y + h <= OUT_H as f32 - MOUTH_H && y + h >= OUT_H as f32 - MOUTH_H - 24.0,
        "not just above the footer"
    );
}

#[test]
fn the_github_link_runs_under_the_picture() {
    let face = modded_badge_face();
    let light = (face.h - 20..face.h)
        .flat_map(|y| (0..face.w).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            let p = pixel(&face, x, y);
            p[3] > 200 && p[0] > 200
        })
        .count();
    assert!(light > 100, "no link under the picture: {light}");
    assert_eq!(slot_ui::MODDED_LINK, "github.com/marghost");
}
