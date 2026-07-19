//! App-profile resolution: picks the effective dictation mode + paste method
//! for the application the user is currently dictating into, based on the
//! identity of the OS-reported focused window (see `settings::AppProfile`).
//!
//! Focused-window-title detection is implemented for Windows here (the
//! project already depends on the `windows` crate with
//! `Win32_UI_WindowsAndMessaging`, see `src-tauri/Cargo.toml`). macOS and
//! Linux (X11/Wayland) are not currently implemented. Wayland in
//! particular has no portable window-title API without compositor-specific
//! portals (e.g. the wlr-foreign-toplevel or xdg-desktop-portal protocols),
//! which is real follow-up work, not a quick add. On unsupported platforms
//! `get_focused_window_target` returns `None`, and callers fall back to the
//! user's global `dictation_mode` setting.

use crate::settings::{AppProfile, AppSettings, DictationMode, PasteMethod};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusedWindowTarget {
    pub title: String,
    #[cfg(target_os = "windows")]
    pub hwnd: isize,
    #[cfg(target_os = "windows")]
    pub process_id: u32,
}

/// Best-effort identity of the currently focused (foreground) window.
/// Returns `None` if unsupported on this platform or if the OS call fails.
#[cfg(target_os = "windows")]
pub fn get_focused_window_target() -> Option<FocusedWindowTarget> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
    };

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
        let mut process_id = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        if process_id == 0 {
            return None;
        }
        Some(FocusedWindowTarget {
            title: String::from_utf16_lossy(&buf[..len as usize]),
            hwnd: hwnd.0 as isize,
            process_id,
        })
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_focused_window_target() -> Option<FocusedWindowTarget> {
    // TODO(macOS): use CGWindowListCopyWindowInfo / NSWorkspace.frontmostApplication.
    // TODO(Linux/X11): use _NET_ACTIVE_WINDOW + _NET_WM_NAME via an X11 connection.
    // TODO(Linux/Wayland): no portable API; would need a compositor-specific
    // protocol (e.g. wlr-foreign-toplevel-management) or an xdg-desktop-portal
    // screencast-adjacent API. Not implemented — untested in this session.
    None
}

#[cfg(target_os = "windows")]
pub fn restore_focused_window(target: &FocusedWindowTarget) -> Result<(), String> {
    use std::ffi::c_void;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetWindowTextW, GetWindowThreadProcessId, IsWindow, SetForegroundWindow, ShowWindow,
        SW_RESTORE,
    };

    unsafe {
        let hwnd = HWND(target.hwnd as *mut c_void);
        if !IsWindow(Some(hwnd)).as_bool() {
            return Err("The original target window is no longer available".to_string());
        }

        let mut process_id = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));
        let mut title_buf = [0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buf);
        let current_title = if title_len > 0 {
            String::from_utf16_lossy(&title_buf[..title_len as usize])
        } else {
            String::new()
        };

        if process_id != target.process_id || current_title != target.title {
            return Err("The original target window changed; paste was cancelled".to_string());
        }

        let _ = ShowWindow(hwnd, SW_RESTORE);
        if !SetForegroundWindow(hwnd).as_bool() {
            return Err("Vozora could not safely restore the original target window".to_string());
        }
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn restore_focused_window(_target: &FocusedWindowTarget) -> Result<(), String> {
    Err("Safe target restoration is not supported on this platform".to_string())
}

/// Find the first `AppProfile` whose `window_title_patterns` contains a
/// case-insensitive substring match against `window_title`.
pub fn match_profile<'a>(window_title: &str, profiles: &'a [AppProfile]) -> Option<&'a AppProfile> {
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
pub fn resolve_dictation_settings(
    settings: &AppSettings,
) -> (
    DictationMode,
    Option<PasteMethod>,
    Option<FocusedWindowTarget>,
) {
    let target = get_focused_window_target();
    let (mode, paste_method) = match target.as_ref() {
        Some(target) => match match_profile(&target.title, &settings.app_profiles) {
            Some(profile) => (profile.dictation_mode, profile.paste_method),
            None => (settings.dictation_mode, None),
        },
        None => (settings.dictation_mode, None),
    };
    (mode, paste_method, target)
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
