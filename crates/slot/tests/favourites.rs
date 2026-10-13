mod common;

use common::{boot, tmp_root_with_carts, write_gb_cart, write_retail_header};
use slot::app::{App, Phase, ShelfKind};
use slot::link_kind::LinkKind;
use slot_input::{Action, Btn};
use slot_store::{read_slot_state, write_slot_state, Platform, StartupScreen};
use slot_ui::{QuickRow, QuickValue, Toast};

fn press(a: &mut App, btn: Btn) {
    a.apply(Action::GbaDown(btn));
    a.apply(Action::GbaUp(btn));
}

fn selected(a: &App) -> (Platform, String) {
    let c = a.selected_cart().expect("a cart is selected");
    (c.platform, c.stem.clone())
}

fn file(d: &tempfile::TempDir, name: &str) -> String {
    std::fs::read_to_string(d.path().join("Config").join(name)).unwrap_or_default()
}

fn start_selected(a: &mut App) {
    a.apply(Action::Insert);
    a.on_core_ready();
    for _ in 0..120 {
        a.update(1.0 / 60.0);
    }
    assert!(
        matches!(a.phase(), Phase::Playing { .. }),
        "{:?}",
        a.phase()
    );
}

#[test]
fn y_stars_and_unstars_the_selected_cart() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    let mut a = boot(d.path());
    press(&mut a, Btn::Y);
    assert_eq!(a.toast(), Some(Toast::FavoriteAdded));
    assert_eq!(file(&d, "favourites.txt"), "gba/Alpha\n");
    assert!(a.is_favorite(a.selected_cart().unwrap()));
    press(&mut a, Btn::Y);
    assert_eq!(a.toast(), Some(Toast::FavoriteRemoved));
    assert_eq!(file(&d, "favourites.txt"), "");
}

#[test]
fn the_new_shelves_come_after_the_platforms_and_empty_ones_are_skipped() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    let mut a = boot(d.path());
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
    press(&mut a, Btn::R1);
    assert_eq!(
        a.shelf_kind(),
        ShelfKind::Platform(Platform::Gba),
        "an empty shelf showed"
    );
    press(&mut a, Btn::Right);
    press(&mut a, Btn::Y);
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    assert_eq!(selected(&a), (Platform::Gba, "Beta".into()));
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
}

#[test]
fn a_game_started_from_favorites_plays_and_lands_in_recents() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    let mut a = boot(d.path());
    press(&mut a, Btn::Right);
    press(&mut a, Btn::Y);
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    start_selected(&mut a);
    assert_eq!(a.seated_cart().map(|c| c.stem.as_str()), Some("Beta"));
    assert_eq!(file(&d, "recents.txt"), "gba/Beta\n");
}

#[test]
fn recents_put_the_last_game_first() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    let mut a = boot(d.path());
    start_selected(&mut a);
    a.apply(Action::Eject);
    for _ in 0..120 {
        a.update(1.0 / 60.0);
    }
    assert!(matches!(a.phase(), Phase::Shelf), "{:?}", a.phase());
    press(&mut a, Btn::Right);
    start_selected(&mut a);
    assert_eq!(file(&d, "recents.txt"), "gba/Beta\ngba/Alpha\n");
}

#[test]
fn recents_ignore_the_letter_jump() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config/recents.txt"), "gba/Alpha\ngba/Beta\n").unwrap();
    let mut a = boot(d.path());
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Recents);
    assert_eq!(selected(&a).1, "Alpha");
    press(&mut a, Btn::Down);
    assert_eq!(selected(&a).1, "Alpha");
}

#[test]
fn unstarring_the_last_favorite_leaves_the_empty_shelf() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    let mut a = boot(d.path());
    press(&mut a, Btn::Y);
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    press(&mut a, Btn::Y);
    assert_ne!(a.shelf_kind(), ShelfKind::Favorites);
    assert!(a.selected_cart().is_some());
}

#[test]
fn the_same_name_on_two_platforms_is_a_favorite_once() {
    let d = tmp_root_with_carts(&["Tetris"]);
    write_gb_cart(&d, "Tetris", "TETRIS");
    let mut a = boot(d.path());
    press(&mut a, Btn::R1);
    assert_eq!(selected(&a), (Platform::Gb, "Tetris".into()));
    press(&mut a, Btn::Y);
    assert_eq!(file(&d, "favourites.txt"), "gb/Tetris\n");
    press(&mut a, Btn::L1);
    assert_eq!(selected(&a), (Platform::Gba, "Tetris".into()));
    assert!(!a.is_favorite(a.selected_cart().unwrap()));
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    start_selected(&mut a);
    assert_eq!(
        a.seated_cart().map(|c| c.platform),
        Some(Platform::Gb),
        "the GBA game of the same name was started"
    );
}

#[test]
fn favorites_and_recents_come_back_after_a_restart() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    {
        let mut a = boot(d.path());
        press(&mut a, Btn::Y);
        start_selected(&mut a);
    }
    let mut a = boot(d.path());
    for _ in 0..120 {
        a.update(1.0 / 60.0);
    }
    a.apply(Action::Eject);
    for _ in 0..120 {
        a.update(1.0 / 60.0);
    }
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Recents);
    assert_eq!(selected(&a).1, "Alpha");
}

fn boot_with_startup(d: &tempfile::TempDir, start: StartupScreen) -> App {
    let mut s = read_slot_state(d.path());
    s.startup_screen = start;
    s.cart = None;
    write_slot_state(d.path(), &s).unwrap();
    boot(d.path())
}

#[test]
fn the_shelf_opens_on_gba_by_default() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    write_gb_cart(&d, "Tetris", "TETRIS");
    let a = boot(d.path());
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
}

#[test]
fn the_shelf_opens_on_the_chosen_startup_screen() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    write_gb_cart(&d, "Tetris", "TETRIS");
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config/favourites.txt"), "gba/Beta\n").unwrap();
    std::fs::write(d.path().join("Config/recents.txt"), "gba/Alpha\n").unwrap();
    for (start, kind) in [
        (StartupScreen::Gb, ShelfKind::Platform(Platform::Gb)),
        (StartupScreen::Favorites, ShelfKind::Favorites),
        (StartupScreen::Recents, ShelfKind::Recents),
    ] {
        let a = boot_with_startup(&d, start);
        assert_eq!(a.shelf_kind(), kind, "{start:?}");
    }
}

#[test]
fn an_empty_startup_screen_opens_the_next_shelf_with_games() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config/recents.txt"), "gba/Alpha\n").unwrap();
    let a = boot_with_startup(&d, StartupScreen::Favorites);
    assert_eq!(a.shelf_kind(), ShelfKind::Recents);
    let a = boot_with_startup(&d, StartupScreen::Gbc);
    assert_eq!(a.shelf_kind(), ShelfKind::Recents);
}

#[test]
fn a_game_left_in_the_slot_still_resumes_over_the_startup_screen() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    let mut s = read_slot_state(d.path());
    s.startup_screen = StartupScreen::Recents;
    s.cart = Some("Beta".into());
    write_slot_state(d.path(), &s).unwrap();
    let a = boot(d.path());
    assert!(
        matches!(a.phase(), Phase::Inserting { .. }),
        "{:?}",
        a.phase()
    );
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
}

#[test]
fn the_startup_screen_row_cycles_and_saves() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    let mut a = boot(d.path());
    a.apply(Action::QuickMenu);
    press(&mut a, Btn::Down);
    press(&mut a, Btn::Down);
    assert_eq!(a.quick_menu(), Some(QuickRow::StartupScreen));
    assert_eq!(
        a.quick_value(QuickRow::StartupScreen),
        Some(QuickValue::Gba)
    );
    press(&mut a, Btn::Right);
    assert_eq!(read_slot_state(d.path()).startup_screen, StartupScreen::Gb);
    press(&mut a, Btn::Left);
    press(&mut a, Btn::Left);
    assert_eq!(
        a.quick_value(QuickRow::StartupScreen),
        Some(QuickValue::Recents)
    );
    assert_eq!(
        read_slot_state(d.path()).startup_screen,
        StartupScreen::Recents
    );
}

fn resume_with_startup(d: &tempfile::TempDir, start: StartupScreen, cart: &str) -> App {
    let mut s = read_slot_state(d.path());
    s.startup_screen = start;
    s.cart = Some(cart.into());
    s.cart_platform = Some(Platform::Gba);
    write_slot_state(d.path(), &s).unwrap();
    let mut a = boot(d.path());
    a.on_core_ready();
    for _ in 0..120 {
        a.update(1.0 / 60.0);
    }
    assert!(
        matches!(a.phase(), Phase::Playing { .. }),
        "{:?}",
        a.phase()
    );
    a.apply(Action::Eject);
    for _ in 0..120 {
        a.update(1.0 / 60.0);
    }
    assert!(matches!(a.phase(), Phase::Shelf), "{:?}", a.phase());
    a
}

#[test]
fn ejecting_a_resumed_game_on_the_startup_screen_stays_there() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config/favourites.txt"), "gba/Beta\n").unwrap();
    let a = resume_with_startup(&d, StartupScreen::Favorites, "Beta");
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    assert_eq!(selected(&a).1, "Beta");
}

#[test]
fn ejecting_a_resumed_game_not_on_the_startup_screen_goes_to_its_platform() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config/favourites.txt"), "gba/Beta\n").unwrap();
    let a = resume_with_startup(&d, StartupScreen::Favorites, "Alpha");
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
    assert_eq!(selected(&a).1, "Alpha");
}

#[test]
fn a_game_started_from_favorites_keeps_its_header_for_link_play() {
    let d = tmp_root_with_carts(&["Alpha"]);
    write_retail_header(&d, "Emerald", "POKEMON EMER", "BPEE");
    let mut a = boot(d.path());
    press(&mut a, Btn::Right);
    assert_eq!(selected(&a).1, "Emerald");
    press(&mut a, Btn::Y);
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    start_selected(&mut a);
    assert_eq!(a.link_mode("Emerald").0, LinkKind::Wireless);
}

#[test]
fn a_game_started_from_recents_keeps_its_header_for_link_play() {
    let d = tmp_root_with_carts(&["Alpha"]);
    write_retail_header(&d, "Emerald", "POKEMON EMER", "BPEE");
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(
        d.path().join("Config/recents.txt"),
        "gba/Alpha\ngba/Emerald\n",
    )
    .unwrap();
    let mut a = boot(d.path());
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Recents);
    press(&mut a, Btn::Right);
    assert_eq!(selected(&a).1, "Emerald");
    start_selected(&mut a);
    assert_eq!(file(&d, "recents.txt"), "gba/Emerald\ngba/Alpha\n");
    assert_eq!(a.link_mode("Emerald").0, LinkKind::Wireless);
}

#[test]
fn recents_open_on_the_game_just_played() {
    let d = tmp_root_with_carts(&["Alpha", "Beta", "Gamma"]);
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config/recents.txt"), "gba/Alpha\ngba/Beta\n").unwrap();
    let mut a = boot(d.path());
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Recents);
    press(&mut a, Btn::Right);
    assert_eq!(selected(&a).1, "Beta");
    press(&mut a, Btn::R1);
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
    press(&mut a, Btn::Right);
    press(&mut a, Btn::Right);
    assert_eq!(selected(&a).1, "Gamma");
    start_selected(&mut a);
    a.apply(Action::Eject);
    for _ in 0..120 {
        a.update(1.0 / 60.0);
    }
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Recents);
    assert_eq!(selected(&a).1, "Gamma");
}

#[test]
fn an_unreadable_favorites_file_is_never_overwritten() {
    let d = tmp_root_with_carts(&["Alpha", "Beta"]);
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    let unreadable = b"gba/Alpha\n\xff\xfe\n".to_vec();
    std::fs::write(d.path().join("Config/favourites.txt"), &unreadable).unwrap();
    let mut a = boot(d.path());
    press(&mut a, Btn::Y);
    assert!(a.refusal_active(a.now()), "the shelf did not refuse");
    assert_ne!(a.toast(), Some(Toast::FavoriteAdded));
    assert_eq!(
        std::fs::read(d.path().join("Config/favourites.txt")).unwrap(),
        unreadable
    );
}

#[test]
fn a_card_without_games_keeps_the_first_shelf_whatever_the_startup_screen() {
    let d = tmp_root_with_carts(&[]);
    let a = boot_with_startup(&d, StartupScreen::Favorites);
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
}

#[test]
fn y_on_the_favorites_shelf_removes_the_game_and_keeps_the_others() {
    let d = tmp_root_with_carts(&["Alpha", "Beta", "Gamma"]);
    let mut a = boot(d.path());
    for _ in 0..3 {
        press(&mut a, Btn::Y);
        press(&mut a, Btn::Right);
    }
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    press(&mut a, Btn::Right);
    assert_eq!(selected(&a).1, "Beta");
    press(&mut a, Btn::Y);
    assert_eq!(a.toast(), Some(Toast::FavoriteRemoved));
    assert_eq!(a.shelf_kind(), ShelfKind::Favorites);
    assert_eq!(file(&d, "favourites.txt"), "gba/Alpha\ngba/Gamma\n");
    assert_eq!(
        selected(&a).1,
        "Gamma",
        "the shelf did not move to the next favorite"
    );
    press(&mut a, Btn::L1);
    assert_eq!(a.shelf_kind(), ShelfKind::Platform(Platform::Gba));
    while selected(&a).1 != "Beta" {
        press(&mut a, Btn::Right);
    }
    assert!(!a.is_favorite(a.selected_cart().unwrap()));
}
