use std::process::Command;

const ALLOWED_SCHEMES: [&str; 3] = ["https", "http", "file"];

#[cfg(test)]
pub enum Action {
    OpenUri(String),
    CopyVisibleField(String),
    Refresh,
}

#[cfg(test)]
pub fn parse_action(action: &str, uri: Option<&str>) -> Option<Action> {
    match action {
        "open-uri" => uri.and_then(|u| validate_uri(u)).map(|v| Action::OpenUri(v.to_string())),
        "copy-visible-field" => uri.map(|v| Action::CopyVisibleField(v.to_string())),
        "refresh" => Some(Action::Refresh),
        _ => None,
    }
}

fn validate_uri(uri: &str) -> Option<&str> {
    let scheme_end = uri.find("://")?;
    let scheme = &uri[..scheme_end];
    if ALLOWED_SCHEMES.contains(&scheme) {
        Some(uri)
    } else {
        None
    }
}

#[tauri::command]
pub fn open_uri(uri: String) -> Result<(), String> {
    let uri = validate_uri(&uri).ok_or_else(|| "unsupported URI scheme".to_string())?;
    Command::new("xdg-open").arg(uri).spawn().map_err(|e| format!("open: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn copy_text(text: String) {
    let _ = Command::new("wl-copy").arg(&text).spawn();
    let _ = Command::new("xclip").args(["-selection", "clipboard"]).arg(&text).spawn();
}
