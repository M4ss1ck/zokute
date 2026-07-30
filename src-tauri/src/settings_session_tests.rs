use crate::settings_session::arrange_transition;
use crate::{
    autostart,
    config::{Config, Profile},
    config_error::ConfigError,
    config_write,
};
use tauri::AppHandle;

#[test]
fn turning_arrange_off_merges_geometry_before_the_flag_drops() {
    // merge_live_geometry early-returns unless EditMode is still set, so the
    // merge must be ordered before the flag clears or a drag is lost.
    assert_eq!(arrange_transition(false), (true, false));
}

#[test]
fn turning_arrange_on_has_nothing_to_merge() {
    assert_eq!(arrange_transition(true), (false, true));
}

#[test]
fn session_commands_expose_authoritative_results() {
    let _: fn(&AppHandle, Config, Profile) -> Result<(), ConfigError> = config_write::apply;
    let _: fn(AppHandle, bool) -> Result<Profile, String> = crate::settings_session::set_arrange;
    let _: fn(AppHandle, bool) -> Result<(), String> = autostart::set_autostart;
}
