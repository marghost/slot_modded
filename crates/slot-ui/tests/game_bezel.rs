use std::path::Path;

use slot_ui::{game_bezel_face, OUT_H, OUT_W};

const PANEL: (f32, f32) = (640.0, 480.0);
const GB_SCALE: f32 = 3.0;

fn picture_on_panel() -> [f32; 4] {
    let (w, h) = (160.0 * GB_SCALE, 144.0 * GB_SCALE);
    let sx = PANEL.0 / OUT_W as f32;
    let sy = (PANEL.0 * OUT_H as f32 / OUT_W as f32).round() / OUT_H as f32;
    let x = (OUT_W as f32 - w) / 2.0 * sx;
    let y = (OUT_H as f32 - h) / 2.0 * sy;
    [x, y, x + w * sx, y + h * sy]
}

#[test]
fn the_rg35xxsp_bezels_open_exactly_on_the_game_picture() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let [px0, py0, px1, py1] = picture_on_panel();
    for name in [
        "bezels 640/GB/bezel_gb_rg35xxsp.png",
        "bezels 640/GBC/bezel_gbc_rg35xxsp.png",
    ] {
        let face = game_bezel_face(&root.join(name)).expect(name);
        assert_eq!((face.w, face.h), (640, 480), "{name}");
        let [hx, hy, hw, hh] = face.hole;
        let (x0, y0) = (hx * PANEL.0, hy * PANEL.1);
        let (x1, y1) = (x0 + hw * PANEL.0, y0 + hh * PANEL.1);
        assert!(
            x0 >= px0 && y0 >= py0 && x1 <= px1 && y1 <= py1,
            "{name}: window {x0},{y0}-{x1},{y1} shows black around the picture {px0},{py0}-{px1},{py1}"
        );
        assert!(
            px0 > x0 - 1.0 && py0 > y0 - 1.0 && px1 < x1 + 1.0 && py1 < y1 + 1.0,
            "{name}: the frame hides more than a pixel of the picture"
        );
    }
}
