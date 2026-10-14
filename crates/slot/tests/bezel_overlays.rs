mod common;

use common::{app_playing_in, tmp_root_with_carts};
use slot::app::{App, Phase};
use slot::video_mode::VideoMode;
use slot_gfx::TexId;
use slot_input::{Action, Btn};
use slot_store::Platform;

const RG35XXSP_HOLE: [f32; 4] = [107.0 / 640.0, 22.0 / 480.0, 426.0 / 640.0, 383.0 / 480.0];
const CANVAS_HOLE: [f32; 4] = [127.0 / 720.0, 33.0 / 480.0, 466.0 / 720.0, 418.0 / 480.0];

fn gb_with_bezel(d: &tempfile::TempDir, size: (u32, u32), hole: [f32; 4]) -> App {
    let mut a = app_playing_in(d.path(), "Emerald");
    a.set_platform(Platform::Gb);
    a.set_panel(size);
    a.set_game_bezel(Some(TexId::from_raw(9)), size, Some(hole));
    a
}

fn near(a: [f32; 4], b: [f32; 4]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1.0)
}

#[test]
fn the_save_state_menu_takes_the_bezel_away_and_gives_it_back() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = gb_with_bezel(&d, (640, 480), RG35XXSP_HOLE);
    assert!(a.shown_game_bezel().is_some());
    a.apply(Action::SaveState);
    a.settle_saves();
    a.apply(Action::Polaroids);
    assert!(
        matches!(a.phase(), Phase::Polaroids { .. }),
        "{:?}",
        a.phase()
    );
    assert!(
        a.shown_game_bezel().is_none(),
        "the bezel hides the save states"
    );
    assert!(a.hud_area().is_none());
    a.apply(Action::GbaDown(Btn::B));
    assert!(
        a.shown_game_bezel().is_some(),
        "the bezel did not come back"
    );
}

#[test]
fn on_a_640_screen_the_hud_fits_the_rg35xxsp_window() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let a = gb_with_bezel(&d, (640, 480), RG35XXSP_HOLE);
    let area = a.hud_area().expect("no hud area with a bezel up");
    assert!(near(area, [120.4, 24.7, 479.3, 430.5]), "{area:?}");
}

#[test]
fn on_the_canvas_the_hud_fits_the_canvas_bezel_window() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let a = gb_with_bezel(&d, (720, 480), CANVAS_HOLE);
    let area = a.hud_area().expect("no hud area with a bezel up");
    assert!(near(area, [127.0, 33.0, 466.0, 418.0]), "{area:?}");
}

#[test]
fn a_gba_game_keeps_the_hud_across_the_screen() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = gb_with_bezel(&d, (640, 480), RG35XXSP_HOLE);
    a.set_platform(Platform::Gba);
    assert!(a.hud_area().is_none());
}

#[test]
fn stretched_the_bezel_is_off_and_the_hud_runs_across_the_game() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = gb_with_bezel(&d, (640, 480), RG35XXSP_HOLE);
    a.set_video_mode(VideoMode::Stretch);
    assert!(a.shown_game_bezel().is_none());
    assert!(
        a.hud_area().is_none(),
        "the hud kept the bezel window in stretch"
    );
    a.set_video_mode(VideoMode::Actual);
    assert!(
        a.hud_area().is_some(),
        "the hud lost the bezel window back at 1:1"
    );
}

#[test]
fn stretched_the_save_state_menu_opens_with_nothing_over_it() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = gb_with_bezel(&d, (640, 480), RG35XXSP_HOLE);
    a.set_video_mode(VideoMode::Stretch);
    a.apply(Action::SaveState);
    a.settle_saves();
    a.apply(Action::Polaroids);
    assert!(
        matches!(a.phase(), Phase::Polaroids { .. }),
        "{:?}",
        a.phase()
    );
    assert!(a.shown_game_bezel().is_none());
    a.apply(Action::GbaDown(Btn::B));
    assert!(
        a.shown_game_bezel().is_none(),
        "a bezel came back over a stretched game"
    );
}
