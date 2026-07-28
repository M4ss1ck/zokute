use crate::platform::Session;

/// Whether to skip creating HUD windows.
pub fn skip_hud_windows(session: &Session) -> bool {
    match session {
        Session::MintCinnamonX11 => false,
        Session::Unsupported(_) => true,
    }
}

/// Return a user-visible compatibility notice for the tray tooltip.
pub fn tray_tooltip(session: &Session) -> String {
    match session {
        Session::MintCinnamonX11 => "Zokute".into(),
        Session::Unsupported(reason) => format!("Zokute — {reason}"),
    }
}

/// Determine whether collection and audio should be suppressed.
pub fn suppress_background_work(session: &Session) -> bool {
    skip_hud_windows(session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::Session;

    #[test]
    fn mint_allows_windows() {
        assert!(!skip_hud_windows(&Session::MintCinnamonX11));
    }

    #[test]
    fn unsupported_skips_windows() {
        assert!(skip_hud_windows(&Session::Unsupported("test".into())));
    }

    #[test]
    fn mint_tooltip_is_simple() {
        assert_eq!(tray_tooltip(&Session::MintCinnamonX11), "Zokute");
    }

    #[test]
    fn unsupported_tooltip_includes_reason() {
        let tip = tray_tooltip(&Session::Unsupported("Wayland".into()));
        assert!(tip.contains("Wayland"));
    }
}
