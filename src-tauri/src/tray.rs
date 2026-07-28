use crate::{settings, window};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle,
};

#[derive(Debug, PartialEq)]
pub enum MenuAction {
    OpenSettings,
    EditLayout,
    ToggleWidgets,
    Quit,
}

pub fn action_for(id: &str) -> Option<MenuAction> {
    match id {
        "settings" => Some(MenuAction::OpenSettings),
        "layout" => Some(MenuAction::EditLayout),
        "toggle" => Some(MenuAction::ToggleWidgets),
        "quit" => Some(MenuAction::Quit),
        _ => None,
    }
}

pub fn init(app: &AppHandle) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "settings", "Settings…", true, None::<&str>)?;
    let layout = MenuItem::with_id(app, "layout", "Edit Layout", true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "Show/Hide Widgets", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &layout, &toggle, &quit])?;
    let icon = app.default_window_icon().cloned().expect("bundled icon");
    TrayIconBuilder::with_id("zokute")
        .icon(icon)
        .tooltip("Zokute")
        .menu(&menu)
        .on_menu_event(|app, event| match action_for(event.id.as_ref()) {
            Some(MenuAction::OpenSettings) => settings::open(app),
            Some(MenuAction::EditLayout) => crate::layout_commands::enter_edit_layout(app.clone()),
            Some(MenuAction::ToggleWidgets) => window::toggle_visibility(app),
            Some(MenuAction::Quit) => app.exit(0),
            None => {}
        })
        .build(app)?;
    Ok(())
}
