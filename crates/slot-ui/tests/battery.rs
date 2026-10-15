use slot_power::{Battery, Charge};
use slot_ui::{
    battery_badge_face, draw_gauge, level_colour, Draw, Printed, TexId, GAUGE_W, HUD_INK, WALL,
};

fn percent_face() -> Printed {
    Printed { face: None, w: 30 }
}

fn quads(out: &[Draw]) -> Vec<(f32, f32, f32, f32)> {
    out.iter()
        .filter_map(|d| match *d {
            Draw::Rect { x, y, w, h, .. } | Draw::Tex { x, y, w, h, .. } => Some((x, y, w, h)),
            _ => None,
        })
        .collect()
}

fn at(percent: u8, charge: Charge) -> Option<Battery> {
    Some(Battery { percent, charge })
}

#[test]
fn nothing_moves_when_the_charge_state_changes() {
    let mut idle = Vec::new();
    let mut charging = Vec::new();
    draw_gauge(
        24.0,
        400.0,
        at(68, Charge::Discharging),
        percent_face(),
        None,
        HUD_INK,
        &mut idle,
    );
    draw_gauge(
        24.0,
        400.0,
        at(68, Charge::Charging),
        percent_face(),
        Some(TexId::from_raw(7)),
        HUD_INK,
        &mut charging,
    );
    let idle = quads(&idle);
    for q in idle.iter() {
        assert!(
            quads(&charging).contains(q),
            "{q:?} moved or vanished when charging started"
        );
    }
}

#[test]
fn the_bolt_never_reaches_the_capsule() {
    let mut out = Vec::new();
    draw_gauge(
        24.0,
        400.0,
        at(68, Charge::Charging),
        percent_face(),
        Some(TexId::from_raw(7)),
        HUD_INK,
        &mut out,
    );
    let bolt_right = out
        .iter()
        .find_map(|d| match *d {
            Draw::Tex { x, w, tex, .. } if tex == TexId::from_raw(7) => Some(x + w),
            _ => None,
        })
        .expect("the bolt did not draw while charging");
    let capsule_left = out
        .iter()
        .filter_map(|d| match *d {
            Draw::Rect { x, .. } => Some(x),
            _ => None,
        })
        .fold(f32::MAX, f32::min);
    assert!(
        bolt_right <= capsule_left,
        "the bolt's right edge ({bolt_right}) reaches past the capsule's left wall ({capsule_left})"
    );
}

#[test]
fn the_bolt_is_only_drawn_while_charging() {
    let bolt = |charge| {
        let mut out = Vec::new();
        draw_gauge(
            24.0,
            400.0,
            at(68, charge),
            percent_face(),
            Some(TexId::from_raw(7)),
            HUD_INK,
            &mut out,
        );
        out.iter()
            .any(|d| matches!(d, Draw::Tex { tex: t, .. } if *t == TexId::from_raw(7)))
    };
    assert!(bolt(Charge::Charging));
    assert!(!bolt(Charge::Discharging));
    assert!(!bolt(Charge::Full));
    assert!(!bolt(Charge::Unknown));
}

#[test]
fn the_fill_tracks_the_percent_and_never_leaves_the_capsule() {
    let mut widths = Vec::new();
    for percent in [0u8, 1, 50, 99, 100] {
        let mut out = Vec::new();
        draw_gauge(
            24.0,
            400.0,
            at(percent, Charge::Discharging),
            percent_face(),
            None,
            HUD_INK,
            &mut out,
        );
        let capsule_left = out
            .iter()
            .filter_map(|d| match *d {
                Draw::Rect { x, .. } => Some(x),
                _ => None,
            })
            .fold(f32::MAX, f32::min);
        let inner_right = capsule_left + GAUGE_W - 2.0 * WALL;
        let fill = out.iter().find_map(|d| match *d {
            Draw::Rect { x, w, .. } if (x - (capsule_left + 2.0 * WALL)).abs() < 0.01 => Some(w),
            _ => None,
        });
        if let Some(w) = fill {
            assert!(
                capsule_left + 2.0 * WALL + w <= inner_right + 0.01,
                "a {percent}% fill burst through the capsule's own wall"
            );
        }
        widths.push(fill.unwrap_or(0.0));
    }
    for pair in widths.windows(2) {
        assert!(
            pair[1] > pair[0],
            "the fill must grow strictly with the percent, got {widths:?}"
        );
    }
}

#[test]
fn no_reading_draws_nothing() {
    let mut out = Vec::new();
    draw_gauge(24.0, 400.0, None, percent_face(), None, HUD_INK, &mut out);
    assert!(out.is_empty());
}

fn inked(rgba: &[u8]) -> usize {
    rgba.chunks_exact(4).filter(|p| p[3] > 0).count()
}

#[test]
fn the_badge_keeps_its_size_whatever_the_charge() {
    let idle = battery_badge_face(Battery {
        percent: 68,
        charge: Charge::Discharging,
    });
    let charging = battery_badge_face(Battery {
        percent: 68,
        charge: Charge::Charging,
    });
    assert!(idle.w > 0 && idle.h > 0);
    assert_eq!(idle.rgba.len(), (idle.w * idle.h * 4) as usize);
    assert_eq!((idle.w, idle.h), (charging.w, charging.h));
    assert!(
        inked(&charging.rgba) > inked(&idle.rgba),
        "the bolt did not draw while charging"
    );
}

#[test]
fn a_fuller_battery_inks_more_of_the_badge() {
    let face = |percent| {
        battery_badge_face(Battery {
            percent,
            charge: Charge::Discharging,
        })
    };
    let (low, high) = (face(10), face(100));
    assert!(inked(&high.rgba) > inked(&low.rgba));
}

#[test]
fn the_fill_goes_from_green_to_red_as_the_battery_drains() {
    let [green, yellow, orange, red] = [100, 50, 25, 10].map(level_colour);
    assert!(green[1] > green[0], "full is not green: {green:?}");
    assert!(
        red[0] > red[1] && red[0] > red[2],
        "empty is not red: {red:?}"
    );
    let bands = [green, yellow, orange, red];
    for (i, a) in bands.iter().enumerate() {
        for b in &bands[i + 1..] {
            assert_ne!(a, b);
        }
    }
    assert_eq!(level_colour(51), green);
    assert_eq!(level_colour(26), yellow);
    assert_eq!(level_colour(11), orange);
    assert_eq!(level_colour(0), red);
}
