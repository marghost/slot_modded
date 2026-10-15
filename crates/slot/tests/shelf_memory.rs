mod common;

use common::{boot, tmp_root_with_carts, write_gb_cart};
use slot::app::{App, Rack};
use slot_input::{Action, Btn};
use slot_store::{read_slot_state, write_slot_state, Platform, StartupScreen};

fn press(a: &mut App, btn: Btn) {
    a.apply(Action::GbaDown(btn));
    a.apply(Action::GbaUp(btn));
}

fn selected(a: &App) -> String {
    a.selected_cart().expect("a cart is selected").stem.clone()
}

fn state_text(d: &tempfile::TempDir) -> String {
    std::fs::read_to_string(d.path().join("Config/slot.state")).unwrap_or_default()
}

fn card() -> tempfile::TempDir {
    let d = tmp_root_with_carts(&["Alpha", "Beta", "Gamma"]);
    write_gb_cart(&d, "Kirby", "KIRBY");
    write_gb_cart(&d, "Tetris", "TETRIS");
    d
}

#[test]
fn each_shelf_opens_where_it_was_left_after_a_restart() {
    let d = card();
    {
        let mut a = boot(d.path());
        press(&mut a, Btn::Right);
        assert_eq!(selected(&a), "Beta");
        press(&mut a, Btn::R1);
        assert_eq!(a.shelf_kind(), Rack::Of(Platform::Gb));
        press(&mut a, Btn::Right);
        assert_eq!(selected(&a), "Tetris");
        press(&mut a, Btn::L1);
    }
    let mut a = boot(d.path());
    assert_eq!(a.shelf_kind(), Rack::Of(Platform::Gba));
    assert_eq!(selected(&a), "Beta");
    press(&mut a, Btn::R1);
    assert_eq!(selected(&a), "Tetris");
}

#[test]
fn the_favorites_shelf_remembers_its_place_too() {
    let d = card();
    {
        let mut a = boot(d.path());
        press(&mut a, Btn::Y);
        press(&mut a, Btn::Right);
        press(&mut a, Btn::Y);
        press(&mut a, Btn::L1);
        assert_eq!(a.shelf_kind(), Rack::Favorites);
        press(&mut a, Btn::Right);
        assert_eq!(selected(&a), "Beta");
        a.apply(Action::QuickMenu);
    }
    let mut s = read_slot_state(d.path());
    s.startup_screen = StartupScreen::Favorites;
    write_slot_state(d.path(), &s).unwrap();
    let a = boot(d.path());
    assert_eq!(a.shelf_kind(), Rack::Favorites);
    assert_eq!(selected(&a), "Beta");
}

#[test]
fn browsing_alone_writes_nothing_to_the_card() {
    let d = card();
    let mut a = boot(d.path());
    let before = state_text(&d);
    for _ in 0..5 {
        press(&mut a, Btn::Right);
    }
    assert_eq!(
        state_text(&d),
        before,
        "a step along the shelf wrote the card"
    );
    press(&mut a, Btn::R1);
    assert_ne!(
        state_text(&d),
        before,
        "leaving the shelf did not remember it"
    );
}

#[test]
fn shutting_the_lid_or_opening_the_power_menu_remembers_the_shelf() {
    for close in [Action::LidClose, Action::PowerHold] {
        let d = card();
        let mut a = boot(d.path());
        press(&mut a, Btn::Right);
        press(&mut a, Btn::Right);
        a.apply(close);
        assert_eq!(
            read_slot_state(d.path()).shelf_marks[0].as_deref(),
            Some("gba/Gamma"),
            "{close:?}"
        );
    }
}

#[test]
fn a_remembered_game_that_is_gone_opens_the_first_game() {
    let d = card();
    let mut s = read_slot_state(d.path());
    s.shelf_marks[0] = Some("gba/Missing".into());
    write_slot_state(d.path(), &s).unwrap();
    let a = boot(d.path());
    assert_eq!(selected(&a), "Alpha");
}
