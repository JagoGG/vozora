//! App-profile resolution: picks the effective dictation mode + paste method
//! for the application the user is currently dictating into, based on the
//! title of the OS-reported focused window (see `settings::AppProfile`).
//!
//! Focused-window-title detection is implemented for Windows here (the
//! project already depends on the `windows` crate with
//! `Win32_UI_WindowsAndMessaging`, see `src-tauri/Cargo.toml`). macOS and
//! Linux (X11/Wayland) are NOT implemented in this session — Wayland in
//! particular has no portable window-title API without compositor-specific
//! portals (e.g. the wlr-foreign-toplevel or xdg-desktop-portal protocols),
//! which is real follow-up work, not a quick add. On unsupported platforms
//! `get_focused_window_title` returns `None`, and callers fall back to the
//! user's global `dictation_mode` setting.

use crate::settings::{AppProfile, AppSettings, DictationMode, PasteMethod};

/// Best-effort title of the currently focused (foreground) window.
/// Returns `None` if unsupported on this platform or if the OS call fails.
#[cfg(target_os = "windows")]
pub fn get_focused_window_title() -> Option<String> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0.is_null() {
            return None;
        }
        let mut buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buf);
        if len <= 0 {
            return None;
        }
        Some(String::from_utf16_lossy(&buf[..len as usize]))
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_focused_window_title() -> Option<String> {
    // TODO(macOS): use CGWindowListCopyWindowInfo / NSWorkspace.frontmostApplication.
    // TODO(Linux/X11): use _NET_ACTIVE_WINDOW + _NET_WM_NAME via an X11 connection.
    // TODO(Linux/Wayland): no portable API; would need a compositor-specific
    // protocol (e.g. wlr-foreign-toplevel-management) or an xdg-desktop-portal
    // screencast-adjacent API. Not implemented — untested in this session.
    None
}

/// Find the first `AppProfile` whose `window_title_patterns` contains a
/// case-insensitive substring match against `window_title`.
pub fn match_profile<'a>(
    window_title: &str,
    profiles: &'a [AppProfile],
) -> Option<&'a AppProfile> {
    let title_lower = window_title.to_lowercase();
    profiles.iter().find(|profile| {
        profile
            .window_title_patterns
            .iter()
            .any(|pattern| title_lower.contains(&pattern.to_lowercase()))
    })
}

/// Resolve the effective dictation mode + optional paste-method override for
/// the currently focused window, falling back to the user's global default
/// when nothing matches (or focused-window detection isn't available on this
/// platform).
pub fn resolve_dictation_settings(settings: &AppSettings) -> (DictationMode, Option<PasteMethod>) {
    match get_focused_window_title() {
        Some(title) => match match_profile(&title, &settings.app_profiles) {
            Some(profile) => (profile.dictation_mode, profile.paste_method),
            None => (settings.dictation_mode, None),
        },
        None => (settings.dictation_mode, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::AppProfile;

    fn profiles() -> Vec<AppProfile> {
        vec![
            AppProfile {
                name: "Cursor".to_string(),
                window_title_patterns: vec!["Cursor".to_string()],
                dictation_mode: DictationMode::Code,
                paste_method: None,
            },
            AppProfile {
                name: "kitty".to_string(),
                window_title_patterns: vec!["kitty".to_string()],
                dictation_mode: DictationMode::TerminalCommand,
                paste_method: Some(PasteMethod::Direct),
            },
        ]
    }

    #[test]
    fn matches_case_insensitively() {
        let profiles = profiles();
        let p = match_profile("main.rs - CURSOR", &profiles).unwrap();
        assert_eq!(p.name, "Cursor");
    }

    #[test]
    fn returns_none_when_nothing_matches() {
        assert!(match_profile("Untitled - Notepad", &profiles()).is_none());
    }

    #[test]
    fn first_matching_profile_wins() {
        let profiles = profiles();
        let p = match_profile("~/project - kitty", &profiles).unwrap();
        assert_eq!(p.name, "kitty");
        assert_eq!(p.paste_method, Some(PasteMethod::Direct));
    }
}
