mod common;

use common::{app_playing_in, tmp_root_with_carts};
use slot::app::App;
use slot_input::Action;
use slot_power::{Battery, Charge};
use slot_ui::Toast;

fn draining(percent: u8) -> Battery {
    Battery {
        percent,
        charge: Charge::Discharging,
    }
}

fn charging(percent: u8) -> Battery {
    Battery {
        percent,
        charge: Charge::Charging,
    }
}

fn later(a: &mut App, now: &mut u64) {
    *now += 20_000;
    a.tick_ms(*now);
}

#[test]
fn fifteen_percent_warns_once() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    let mut now = 0;
    a.on_battery(draining(16));
    assert_eq!(a.toast(), None);
    a.on_battery(draining(15));
    assert_eq!(a.toast(), Some(Toast::BatteryLow15));
    later(&mut a, &mut now);
    a.on_battery(draining(14));
    a.on_battery(draining(12));
    assert_eq!(a.toast(), None, "the 15% warning came back");
}

#[test]
fn ten_percent_warns_after_fifteen() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    let mut now = 0;
    a.on_battery(draining(15));
    later(&mut a, &mut now);
    a.on_battery(draining(10));
    assert_eq!(a.toast(), Some(Toast::BatteryLow10));
    later(&mut a, &mut now);
    a.on_battery(draining(8));
    assert_eq!(a.toast(), None);
}

#[test]
fn a_reading_past_both_levels_shows_only_the_lower_one() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    let mut now = 0;
    a.on_battery(draining(9));
    assert_eq!(a.toast(), Some(Toast::BatteryLow10));
    later(&mut a, &mut now);
    a.on_battery(draining(9));
    assert_eq!(a.toast(), None);
}

#[test]
fn charging_never_warns_and_brings_the_warnings_back() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    let mut now = 0;
    a.on_battery(charging(12));
    assert_eq!(a.toast(), None);
    a.on_battery(draining(12));
    assert_eq!(a.toast(), Some(Toast::BatteryLow15));
    later(&mut a, &mut now);
    a.on_battery(charging(13));
    a.on_battery(draining(14));
    assert_eq!(a.toast(), Some(Toast::BatteryLow15));
}

#[test]
fn a_shut_lid_holds_the_warning_until_it_opens() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    a.apply(Action::LidClose);
    a.on_battery(draining(14));
    assert_eq!(a.toast(), None);
    a.apply(Action::LidOpen);
    a.on_battery(draining(14));
    assert_eq!(a.toast(), Some(Toast::BatteryLow15));
}

#[test]
fn the_power_off_level_shows_no_warning() {
    let d = tmp_root_with_carts(&["Emerald"]);
    let mut a = app_playing_in(d.path(), "Emerald");
    a.on_battery(draining(4));
    assert_eq!(a.toast(), None);
    assert!(a.powering_off());
}
