use crate::tray::{action_for, MenuAction};

#[test]
fn maps_every_menu_id_to_its_action() {
    assert_eq!(action_for("settings"), Some(MenuAction::OpenSettings));
    assert_eq!(action_for("layout"), Some(MenuAction::EditLayout));
    assert_eq!(action_for("toggle"), Some(MenuAction::ToggleWidgets));
    assert_eq!(action_for("quit"), Some(MenuAction::Quit));
}

#[test]
fn ignores_unknown_menu_ids() {
    assert_eq!(action_for("bogus"), None);
    assert_eq!(action_for(""), None);
}
