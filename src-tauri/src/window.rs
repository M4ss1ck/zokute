use gtk::prelude::*;
use tauri::WebviewWindow;

pub fn configure(window: &WebviewWindow) {
  let _ = window.set_always_on_bottom(true);
  let _ = window.set_ignore_cursor_events(true);
  #[cfg(target_os = "linux")]
  if let Ok(gtk_window) = window.gtk_window() {
    gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop);
    gtk_window.stick();
  }
  // Cinnamon can hide normal sticky windows during Show Desktop, so Desktop hint is the fallback.
}
