use slot_store::{
    import_legacy_favorites, read_slot_state, remove_legacy_favorites, write_slot_state, Platform,
    Recent, SlotState,
};

fn card(file: &str, text: &str) -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config").join(file), text).unwrap();
    d
}

fn game(platform: Platform, stem: &str) -> Recent {
    Recent {
        platform,
        stem: stem.into(),
    }
}

#[test]
fn a_card_without_the_old_file_imports_nothing() {
    let d = tempfile::tempdir().unwrap();
    let mut s = SlotState::default();
    assert!(!import_legacy_favorites(d.path(), &mut s).unwrap());
    assert!(s.favorites.is_empty());
}

#[test]
fn old_favourites_move_into_the_state_and_the_file_goes() {
    let d = card("favourites.txt", "gba/Zelda\ngbc/Pokemon Crystal\n");
    let mut s = SlotState::default();
    assert!(import_legacy_favorites(d.path(), &mut s).unwrap());
    assert_eq!(
        s.favorites,
        [
            game(Platform::Gba, "Zelda"),
            game(Platform::Gbc, "Pokemon Crystal")
        ]
    );
    write_slot_state(d.path(), &s).unwrap();
    remove_legacy_favorites(d.path()).unwrap();
    assert!(!d.path().join("Config/favourites.txt").exists());
    assert_eq!(read_slot_state(d.path()).favorites, s.favorites);
}

#[test]
fn old_favourites_join_the_ones_already_starred() {
    let d = card("favourites.txt", "gba/Zelda\ngb/Tetris\n");
    let mut s = SlotState::default();
    s.toggle_favorite(Platform::Gb, "Tetris");
    import_legacy_favorites(d.path(), &mut s).unwrap();
    assert_eq!(
        s.favorites,
        [game(Platform::Gb, "Tetris"), game(Platform::Gba, "Zelda")]
    );
}

#[test]
fn bad_and_repeated_old_lines_are_skipped() {
    let d = card(
        "favourites.txt",
        "gba/Zelda\nnes/Mario\nno slash\ngb/\nGBA/Zelda\ngbc/Pokemon Crystal\r\n",
    );
    let mut s = SlotState::default();
    import_legacy_favorites(d.path(), &mut s).unwrap();
    let stems: Vec<&str> = s.favorites.iter().map(|f| f.stem.as_str()).collect();
    assert_eq!(stems, ["Zelda", "Pokemon Crystal"]);
}

#[test]
fn recents_kept_in_slot_state_by_the_last_release_survive_and_move_to_their_file() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    write_slot_state(d.path(), &SlotState::default()).unwrap();
    let path = d.path().join("Config/slot.state");
    let old = std::fs::read_to_string(&path).unwrap();
    std::fs::write(&path, old + "recent=gba/Zelda\nrecent=gb/Tetris\n").unwrap();
    let s = read_slot_state(d.path());
    assert_eq!(
        s.recents,
        [game(Platform::Gba, "Zelda"), game(Platform::Gb, "Tetris")]
    );
    write_slot_state(d.path(), &s).unwrap();
    let state = std::fs::read_to_string(d.path().join("Config/slot.state")).unwrap();
    assert!(!state.contains("recent="), "{state}");
    assert_eq!(
        std::fs::read_to_string(d.path().join("Config/recents.txt")).unwrap(),
        "gba/Zelda\ngb/Tetris\n"
    );
    assert_eq!(read_slot_state(d.path()).recents, s.recents);
}

#[test]
fn the_recents_file_wins_over_old_lines_in_slot_state() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    write_slot_state(d.path(), &SlotState::default()).unwrap();
    let path = d.path().join("Config/slot.state");
    let old = std::fs::read_to_string(&path).unwrap();
    std::fs::write(&path, old + "recent=gba/Zelda\n").unwrap();
    std::fs::write(d.path().join("Config/recents.txt"), "gbc/Pokemon Crystal\n").unwrap();
    assert_eq!(
        read_slot_state(d.path()).recents,
        [game(Platform::Gbc, "Pokemon Crystal")]
    );
}
