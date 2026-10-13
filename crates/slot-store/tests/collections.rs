use std::path::PathBuf;

use slot_store::{Cart, Collections, Platform, RECENTS_MAX};

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
fn a_card_without_the_files_has_no_favorites_or_recents() {
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
fn recents_put_the_last_game_first_without_repeats() {
    let mut c = Collections::default();
    for stem in ["A", "B", "A", "C"] {
        c.played(&cart(Platform::Gba, stem));
    }
    let stems: Vec<&str> = c.recents.iter().map(|k| k.stem.as_str()).collect();
    assert_eq!(stems, ["C", "A", "B"]);
}

#[test]
fn recents_keep_only_the_last_ten() {
    let mut c = Collections::default();
    for i in 0..15 {
        c.played(&cart(Platform::Gba, &format!("Game {i}")));
    }
    assert_eq!(c.recents.len(), RECENTS_MAX);
    assert_eq!(c.recents[0].stem, "Game 14");
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
fn recent_carts_follow_the_play_order() {
    let mut c = Collections::default();
    c.played(&cart(Platform::Gba, "Zelda"));
    c.played(&cart(Platform::Gba, "Metroid"));
    let shelf = [cart(Platform::Gba, "Metroid"), cart(Platform::Gba, "Zelda")];
    let stems: Vec<String> = c
        .recent_carts(shelf.iter())
        .into_iter()
        .map(|c| c.stem)
        .collect();
    assert_eq!(stems, ["Metroid", "Zelda"]);
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
