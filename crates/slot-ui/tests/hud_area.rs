use slot_ui::{Canvas, Draw, FfState, Hud, HudKind, Icon, TexId, Toast};

const AREA: [f32; 4] = [120.0, 25.0, 480.0, 430.0];

fn inside(out: &[Draw]) {
    assert!(!out.is_empty(), "nothing was drawn");
    for d in out {
        let (x, y, w, h) = match *d {
            Draw::Rect { x, y, w, h, .. } | Draw::Tex { x, y, w, h, .. } => (x, y, w, h),
            _ => continue,
        };
        assert!(
            x >= AREA[0]
                && y >= AREA[1]
                && x + w <= AREA[0] + AREA[2]
                && y + h <= AREA[1] + AREA[3],
            "{d:?} leaves the bezel window {AREA:?}"
        );
    }
}

fn hud() -> Hud {
    let mut h = Hud::new();
    h.set_icons((0..Icon::ALL.len()).map(TexId::from_raw).collect());
    h.set_toasts((100..200).map(TexId::from_raw).collect());
    h
}

#[test]
fn the_volume_bar_and_fast_forward_badge_stay_in_the_window() {
    let mut h = hud();
    h.show(HudKind::Volume, 7, false, 0);
    h.set_ff(FfState::Latched);
    let mut out = Vec::new();
    h.draw_within(Canvas::FRAMED, 10, AREA, &mut out);
    inside(&out);
}

#[test]
fn the_rewind_bar_stays_in_the_window() {
    let mut h = hud();
    h.show(HudKind::Rewind, 50, false, 0);
    let mut out = Vec::new();
    h.draw_within(Canvas::FRAMED, 10, AREA, &mut out);
    inside(&out);
}

#[test]
fn a_banner_stays_in_the_window() {
    let mut h = hud();
    h.toast(Toast::StateSaved, 0);
    let mut out = Vec::new();
    h.draw_within(Canvas::FRAMED, 10, AREA, &mut out);
    inside(&out);
}

#[test]
fn without_a_window_the_hud_keeps_its_place() {
    let mut a = hud();
    a.show(HudKind::Volume, 7, false, 0);
    let (mut full, mut plain) = (Vec::new(), Vec::new());
    a.draw(Canvas::FRAMED, 10, &mut plain);
    a.draw_within(Canvas::FRAMED, 10, [0.0, 0.0, 720.0, 480.0], &mut full);
    assert_eq!(full, plain);
}
