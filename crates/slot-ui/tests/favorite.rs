use std::path::Path;

use slot_ui::{favorite_face, favorite_heart_face, FAVORITE_FILE, FAVORITE_PX};

fn write_png(path: &Path, w: u32, h: u32, rgba: &[u8]) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = std::fs::File::create(path).unwrap();
    let mut e = png::Encoder::new(file, w, h);
    e.set_color(png::ColorType::Rgba);
    e.write_header().unwrap().write_image_data(rgba).unwrap();
}

#[test]
fn without_a_card_or_a_file_the_built_in_heart_is_used() {
    let d = tempfile::tempdir().unwrap();
    assert_eq!(favorite_face(None).rgba, favorite_heart_face().rgba);
    assert_eq!(
        favorite_face(Some(d.path())).rgba,
        favorite_heart_face().rgba
    );
}

#[test]
fn a_broken_file_falls_back_to_the_built_in_heart() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("System")).unwrap();
    std::fs::write(d.path().join(FAVORITE_FILE), b"not a png").unwrap();
    assert_eq!(
        favorite_face(Some(d.path())).rgba,
        favorite_heart_face().rgba
    );
}

#[test]
fn the_card_image_replaces_the_heart_at_its_size() {
    let d = tempfile::tempdir().unwrap();
    let red: Vec<u8> = [255, 0, 0, 255].repeat(300 * 300);
    write_png(&d.path().join(FAVORITE_FILE), 300, 300, &red);
    let f = favorite_face(Some(d.path()));
    assert_eq!((f.w, f.h), (FAVORITE_PX, FAVORITE_PX));
    assert!(f.rgba.chunks(4).all(|p| p == [255, 0, 0, 255]));
}

#[test]
fn a_shrunk_image_keeps_its_colour_at_see_through_edges() {
    let d = tempfile::tempdir().unwrap();
    let mut px = Vec::new();
    for _ in 0..224 * 224 {
        px.extend_from_slice(&[0, 0, 0, 0]);
    }
    for y in 0..224 {
        for x in (0..224).step_by(2) {
            let o = (y * 224 + x) * 4;
            px[o..o + 4].copy_from_slice(&[255, 255, 255, 255]);
        }
    }
    write_png(&d.path().join(FAVORITE_FILE), 224, 224, &px);
    let f = favorite_face(Some(d.path()));
    for p in f.rgba.chunks(4) {
        assert_eq!(
            &p[..3],
            [255, 255, 255],
            "a see-through pixel darkened the edge"
        );
        assert!((120..=135).contains(&p[3]), "alpha {}", p[3]);
    }
}

#[test]
fn a_wide_image_is_fitted_whole_and_centred() {
    let d = tempfile::tempdir().unwrap();
    let blue: Vec<u8> = [0, 0, 255, 255].repeat(200 * 100);
    write_png(&d.path().join(FAVORITE_FILE), 200, 100, &blue);
    let f = favorite_face(Some(d.path()));
    let at = |x: u32, y: u32| f.rgba[((y * f.w + x) * 4 + 3) as usize];
    assert_eq!(at(FAVORITE_PX / 2, 2), 0, "the top band should be empty");
    assert_eq!(at(FAVORITE_PX / 2, FAVORITE_PX / 2), 255);
    assert_eq!(
        at(1, FAVORITE_PX / 2),
        255,
        "the image was cropped at the sides"
    );
}
