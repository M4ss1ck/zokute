mod window;

use tauri::Manager;

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if let Some(window) = app.get_webview_window("main") {
        window::configure(&window);
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
