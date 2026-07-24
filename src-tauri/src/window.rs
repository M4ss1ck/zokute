use gtk::prelude::*;
use tauri::WebviewWindow;

pub fn configure(window: &WebviewWindow) {
  let _ = window.set_always_on_bottom(true);
  let _ = window.set_ignore_cursor_events(true);
  #[cfg(target_os = "linux")]
  if let Ok(gtk_window) = window.gtk_window() {
    gtk_window.stick();
  }
  // _NET_WM_WINDOW_TYPE_DESKTOP is the fallback if Cinnamon stacking misbehaves.
}
