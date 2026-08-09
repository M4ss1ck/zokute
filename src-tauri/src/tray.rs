use crate::{platform::Session, process_resources, session_guard, settings, window};
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle,
};

#[derive(Debug, PartialEq)]
pub enum MenuAction {
    OpenSettings,
    EditLayout,
    ToggleWidgets,
    CycleDisplays,
    Quit,
}

pub fn action_for(id: &str) -> Option<MenuAction> {
    match id {
        "settings" => Some(MenuAction::OpenSettings),
        "layout" => Some(MenuAction::EditLayout),
        "toggle" => Some(MenuAction::ToggleWidgets),
        "displays" => Some(MenuAction::CycleDisplays),
        "quit" => Some(MenuAction::Quit),
        _ => None,
    }
}

pub fn init(app: &AppHandle, session: &Session) -> tauri::Result<()> {
    let cpu = MenuItem::with_id(app, "cpu", "CPU: --", false, None::<&str>)?;
    let ram = MenuItem::with_id(app, "ram", "RAM: --", false, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let open = MenuItem::with_id(app, "settings", "Settings…", true, None::<&str>)?;
    let layout = MenuItem::with_id(app, "layout", "Edit Layout", true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "Show/Hide Widgets", true, None::<&str>)?;
    let multi_display = app
        .available_monitors()
        .map(|list| list.len() > 1)
        .unwrap_or(false);
    let displays = MenuItem::with_id(
        app,
        "displays",
        "Move Widgets to Next Display",
        multi_display,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(
        app,
        &[&cpu, &ram, &sep, &open, &layout, &toggle, &displays, &quit],
    )?;
    let icon = app.default_window_icon().cloned().expect("bundled icon");
    let tooltip = session_guard::tray_tooltip(session);
    TrayIconBuilder::with_id("zokute")
        .icon(icon)
        .tooltip(&tooltip)
        .menu(&menu)
        .on_menu_event(|app, event| match action_for(event.id.as_ref()) {
            Some(MenuAction::OpenSettings) => settings::open(app),
            Some(MenuAction::EditLayout) => {
                crate::settings_session::open_settings_arranging(app.clone())
            }
            Some(MenuAction::ToggleWidgets) => window::toggle_visibility(app),
            Some(MenuAction::CycleDisplays) => crate::monitor_cycle::run(app),
            Some(MenuAction::Quit) => app.exit(0),
            None => {}
        })
        .build(app)?;

    let cpu_item = cpu.clone();
    let ram_item = ram.clone();
    tauri::async_runtime::spawn(async move {
        let mut reader = process_resources::ResourceReader::new();
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            interval.tick().await;
            if let Some((cpu_pct, ram_mib)) = reader.sample() {
                let _ = cpu_item.set_text(format!("CPU: {:.1}%", cpu_pct));
                let _ = ram_item.set_text(format!("RAM: {:.1} MiB", ram_mib));
            }
        }
    });

    Ok(())
}
