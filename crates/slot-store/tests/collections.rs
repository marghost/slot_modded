use std::path::PathBuf;

use slot_store::{
    import_legacy_recents, remove_legacy_recents, Cart, Collections, Platform, Recent, SlotState,
    RECENTS_MAX,
};

fn cart(platform: Platform, stem: &str) -> Cart {
    Cart {
        platform,
        stem: stem.into(),
        rom: PathBuf::from(format!("{stem}.rom")),
        label: None,
        title: String::new(),
        code: String::new(),
        shell: None,
    }
}

#[test]
fn a_card_without_the_file_has_no_favorites() {
    let d = tempfile::tempdir().unwrap();
    assert_eq!(Collections::read(d.path()).unwrap(), Collections::default());
}

#[test]
fn favorites_toggle_and_survive_a_restart() {
    let d = tempfile::tempdir().unwrap();
    let zelda = cart(Platform::Gba, "Zelda");
    let mut c = Collections::default();
    assert!(c.toggle_favorite(&zelda));
    assert!(c.is_favorite(&zelda));
    c.write_favorites(d.path()).unwrap();
    assert_eq!(
        std::fs::read_to_string(d.path().join("Config/favourites.txt")).unwrap(),
        "gba/Zelda\n"
    );
    let mut back = Collections::read(d.path()).unwrap();
    assert!(back.is_favorite(&zelda));
    assert!(!back.toggle_favorite(&zelda));
    assert!(!back.is_favorite(&zelda));
}

#[test]
fn the_same_name_on_two_platforms_is_two_games() {
    let mut c = Collections::default();
    c.toggle_favorite(&cart(Platform::Gb, "Tetris"));
    assert!(c.is_favorite(&cart(Platform::Gb, "Tetris")));
    assert!(!c.is_favorite(&cart(Platform::Gbc, "Tetris")));
}

#[test]
fn favorite_carts_are_sorted_and_missing_games_are_hidden_but_kept() {
    let mut c = Collections::default();
    for (p, stem) in [
        (Platform::Gba, "Zelda"),
        (Platform::Gb, "Gone"),
        (Platform::Gbc, "Alleyway"),
    ] {
        c.toggle_favorite(&cart(p, stem));
    }
    let shelf = [
        cart(Platform::Gba, "Zelda"),
        cart(Platform::Gbc, "Alleyway"),
        cart(Platform::Gba, "Metroid"),
    ];
    let stems: Vec<String> = c
        .favorite_carts(shelf.iter())
        .into_iter()
        .map(|c| c.stem)
        .collect();
    assert_eq!(stems, ["Alleyway", "Zelda"]);
    assert_eq!(c.favorites.len(), 3);
}

#[test]
fn bad_and_repeated_lines_are_skipped() {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(
        d.path().join("Config/favourites.txt"),
        "gba/Zelda\nnes/Mario\nno slash\ngb/\nGBA/Zelda\ngbc/Pokemon Crystal\r\n",
    )
    .unwrap();
    let c = Collections::read(d.path()).unwrap();
    let stems: Vec<&str> = c.favorites.iter().map(|k| k.stem.as_str()).collect();
    assert_eq!(stems, ["Zelda", "Pokemon Crystal"]);
}

fn old_recents(text: &str) -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(d.path().join("Config")).unwrap();
    std::fs::write(d.path().join("Config/recents.txt"), text).unwrap();
    d
}

#[test]
fn an_old_recents_file_moves_into_the_state_and_shows_the_shelf() {
    let d = old_recents("gbc/Pokemon Crystal\ngba/Zelda\nnes/Mario\n");
    let mut s = SlotState::default();
    assert!(import_legacy_recents(d.path(), &mut s).unwrap());
    assert!(s.show_recents);
    assert_eq!(
        s.recents,
        [
            Recent {
                platform: Platform::Gbc,
                stem: "Pokemon Crystal".into()
            },
            Recent {
                platform: Platform::Gba,
                stem: "Zelda".into()
            }
        ]
    );
    remove_legacy_recents(d.path()).unwrap();
    assert!(!d.path().join("Config/recents.txt").exists());
    assert!(!import_legacy_recents(d.path(), &mut s).unwrap());
}

#[test]
fn an_old_recents_file_keeps_only_the_last_ten() {
    let text: String = (0..15).map(|i| format!("gba/Game {i}\n")).collect();
    let d = old_recents(&text);
    let mut s = SlotState::default();
    import_legacy_recents(d.path(), &mut s).unwrap();
    assert_eq!(s.recents.len(), RECENTS_MAX);
    assert_eq!(s.recents[0].stem, "Game 0");
}

#[test]
fn an_old_recents_file_never_replaces_recents_already_in_the_state() {
    let d = old_recents("gba/Zelda\n");
    let mut s = SlotState::default();
    s.played(Platform::Gb, "Tetris");
    assert!(import_legacy_recents(d.path(), &mut s).unwrap());
    assert!(!s.show_recents);
    assert_eq!(s.recents.len(), 1);
    assert_eq!(s.recents[0].stem, "Tetris");
}

#[test]
fn an_empty_old_recents_file_leaves_the_shelf_hidden() {
    let d = old_recents("");
    let mut s = SlotState::default();
    assert!(import_legacy_recents(d.path(), &mut s).unwrap());
    assert!(!s.show_recents);
    assert!(s.recents.is_empty());
}
