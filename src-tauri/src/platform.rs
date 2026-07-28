use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum Session {
    MintCinnamonX11,
    Unsupported(String),
}

const SUPPORTED_ARCH: &str = "x86_64";

pub fn detect_arch() -> String {
    std::env::consts::ARCH.to_string()
}

pub fn is_supported_arch() -> bool {
    detect_arch() == SUPPORTED_ARCH
}

pub fn detect() -> Session {
    let arch = detect_arch();
    if arch != SUPPORTED_ARCH {
        return Session::Unsupported(format!("architecture {} is not supported (requires x86_64)", arch));
    }
    let os_release = read_os_release();
    if !is_mint(&os_release) {
        return Session::Unsupported(os_unsupported_message(&os_release));
    }
    if !is_cinnamon_desktop() {
        return Session::Unsupported("Cinnamon desktop not detected".into());
    }
    if !is_x11() {
        return Session::Unsupported("Wayland session detected — Zokute requires X11".into());
    }
    Session::MintCinnamonX11
}

pub fn is_supported() -> bool {
    detect() == Session::MintCinnamonX11
}

pub fn unsupported_reason() -> Option<String> {
    match detect() {
        Session::MintCinnamonX11 => None,
        Session::Unsupported(reason) => Some(reason),
    }
}

pub fn compatibility_message(session: &Session) -> String {
    match session {
        Session::MintCinnamonX11 => String::new(),
        Session::Unsupported(reason) => {
            format!(
                "Zokute supports Linux Mint Cinnamon on X11 (x86_64).\n\
                 {reason}\n\n\
                 The tray icon and settings will remain available.",
            )
        }
    }
}

fn read_os_release() -> Vec<(String, String)> {
    let content = fs::read_to_string("/etc/os-release").unwrap_or_default();
    content.lines().filter_map(|line| {
        let mut parts = line.splitn(2, '=');
        let key = parts.next()?.to_string();
        let value = parts.next().unwrap_or("").trim_matches('"').to_string();
        Some((key, value))
    }).collect()
}

fn value_from<'a>(fields: &'a [(String, String)], key: &'a str) -> Option<&'a str> {
    fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
}

fn is_mint(os_release: &[(String, String)]) -> bool {
    let id = value_from(os_release, "ID").unwrap_or("");
    let id_like = value_from(os_release, "ID_LIKE").unwrap_or("");
    id == "linuxmint" || id_like.split_whitespace().any(|part| part == "ubuntu" || part == "debian")
}

fn os_unsupported_message(os_release: &[(String, String)]) -> String {
    let id = value_from(os_release, "ID").unwrap_or("unknown");
    let version = value_from(os_release, "VERSION_ID").unwrap_or("unknown");
    format!("OS {id} {version} is not supported (requires Linux Mint)")
}

fn is_cinnamon_desktop() -> bool {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let gdesktop = std::env::var("GDMSESSION").unwrap_or_default();
    desktop == "X-Cinnamon" || desktop.contains("Cinnamon") || gdesktop == "cinnamon" || gdesktop == "cinnamon2d"
}

fn is_x11() -> bool {
    let wayland = std::env::var("WAYLAND_DISPLAY");
    let xdg_session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    wayland.is_err() && xdg_session_type != "wayland"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_mint_from_os_release_id() {
        let fields = vec![("ID".into(), "linuxmint".into()), ("VERSION_ID".into(), "22".into())];
        assert!(is_mint(&fields));
    }

    #[test]
    fn does_not_detect_ubuntu_as_mint() {
        let fields = vec![("ID".into(), "ubuntu".into()), ("VERSION_ID".into(), "24.04".into())];
        assert!(!is_mint(&fields));
    }

    #[test]
    fn generic_debian_is_not_mint() {
        let fields = vec![("ID".into(), "debian".into())];
        assert!(!is_mint(&fields));
    }

    #[test]
    fn arch_detection_returns_current() {
        let arch = detect_arch();
        assert!(!arch.is_empty());
    }

    #[test]
    fn compatibility_message_is_empty_for_mint() {
        let msg = compatibility_message(&Session::MintCinnamonX11);
        assert!(msg.is_empty());
    }

    #[test]
    fn compatibility_message_includes_reason() {
        let msg = compatibility_message(&Session::Unsupported("Wayland detected".into()));
        assert!(msg.contains("Wayland"));
    }
}
