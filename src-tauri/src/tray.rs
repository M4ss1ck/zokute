use crate::{settings, window};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

#[derive(Debug, PartialEq)]
pub enum MenuAction {
    OpenSettings,
    ToggleWidgets,
    Quit,
}

pub fn action_for(id: &str) -> Option<MenuAction> {
    match id {
        "settings" => Some(MenuAction::OpenSettings),
        "toggle" => Some(MenuAction::ToggleWidgets),
        "quit" => Some(MenuAction::Quit),
        _ => None,
    }
}

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "settings", "Settings…", true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "Show/Hide Widgets", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &toggle, &quit])?;
    let icon = app.default_window_icon().cloned().expect("bundled icon");
    // The returned handle is intentionally dropped: `build` registers the icon
    // in the app's resource table, which owns it for the process lifetime.
    TrayIconBuilder::with_id("zokute")
        .icon(icon)
        .tooltip("Zokute")
        .menu(&menu)
        .on_menu_event(|app, event| match action_for(event.id.as_ref()) {
            Some(MenuAction::OpenSettings) => settings::open(app),
            Some(MenuAction::ToggleWidgets) => window::toggle_visibility(app),
            Some(MenuAction::Quit) => app.exit(0),
            None => {}
        })
        .build(app)?;
    Ok(())
}
