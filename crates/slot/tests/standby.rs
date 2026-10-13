mod common;

use common::{app_playing_in, tmp_root_with_carts};
use slot::app::Phase;
use slot::standby::{Standby, Step, DARK_FRAMES};
use slot_input::Action;
use slot_store::{read_slot_state, Core, Platform, StateRing};

fn ring(d: &tempfile::TempDir) -> StateRing {
    StateRing::new(d.path(), Platform::Gba, Core::Mgba, "Emerald")
}

#[test]
fn awake_always_draws() {
    let mut s = Standby::new();
    for _ in 0..100 {
        assert_eq!(s.step(false), Step::Draw);
    }
    assert!(!s.resting());
}

#[test]
fn a_doze_draws_its_dark_frames_then_rests() {
    let mut s = Standby::new();
    for i in 0..DARK_FRAMES {
        assert_eq!(s.step(true), Step::Draw, "dark frame {i} was skipped");
    }
    for _ in 0..100 {
        assert_eq!(s.step(true), Step::Rest);
    }
    assert!(s.resting());
}

#[test]
fn waking_draws_straight_away_and_the_next_doze_starts_dark_again() {
    let mut s = Standby::new();
    for _ in 0..10 {
        s.step(true);
    }
    assert_eq!(
        s.step(false),
        Step::Draw,
        "the opened lid waited for a frame"
    );
    assert_eq!(
        s.step(true),
        Step::Draw,
        "the next doze skipped its dark frames"
    );
}

#[test]
fn quick_lid_cycles_never_rest_while_open() {
    let mut s = Standby::new();
    for cycle in 0..50 {
        for _ in 0..cycle % 5 {
            s.step(true);
        }
        assert_eq!(s.step(false), Step::Draw, "cycle {cycle} rested while open");
    }
}

#[test]
fn the_lid_and_power_both_put_the_app_in_standby() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    assert!(!a.standing_by());
    a.apply(Action::LidClose);
    assert!(a.standing_by(), "a shut lid did not stand by");
    a.apply(Action::LidOpen);
    assert!(!a.standing_by());
    assert!(matches!(a.phase(), Phase::Playing { .. }));

    a.apply(Action::PowerTap);
    assert!(a.standing_by(), "a tap of POWER did not stand by");
    a.apply(Action::PowerTap);
    assert!(!a.standing_by());
    assert!(matches!(a.phase(), Phase::Playing { .. }));
}

#[test]
fn the_doze_timeout_still_powers_off_and_is_not_standby() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    a.apply(Action::LidClose);
    a.on_doze_timeout();
    assert!(
        a.shutting_down(),
        "the 3-minute timeout no longer powers off"
    );
    assert!(!a.standing_by(), "standby would hide the shutdown");
    assert_eq!(read_slot_state(d.path()).cart, Some("Emerald".into()));
}

#[test]
fn fifty_quick_lid_cycles_keep_the_game_and_its_resume() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    let mut now = 1_000;
    for cycle in 0..50 {
        a.apply(Action::LidClose);
        assert!(matches!(a.phase(), Phase::Doze { .. }), "cycle {cycle}");
        now += 150 * (cycle % 4);
        a.tick_ms(now);
        a.apply(Action::LidOpen);
        assert!(
            matches!(a.phase(), Phase::Playing { .. }),
            "cycle {cycle}: {:?}",
            a.phase()
        );
        now += 16 * (cycle % 3);
        a.tick_ms(now);
    }
    a.settle_saves();
    assert!(ring(&d).read_resume().unwrap().is_some(), "no resume state");
    assert!(
        ring(&d).list().unwrap().is_empty(),
        "lid cycles made save states"
    );
    assert!(!a.shutting_down());
}
