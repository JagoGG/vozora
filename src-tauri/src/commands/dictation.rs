//! Commands backing the "terminal-command" dictation mode's confirmation
//! gate. See `settings::DictationMode::TerminalCommand` and
//! `coding_mode::looks_destructive`: when a transcription in that mode looks
//! like a destructive shell command, `actions.rs` stores it here instead of
//! pasting immediately and emits a `pending-destructive-paste` event; the
//! frontend shows a confirmation dialog that calls one of these two
//! commands. Vozora never executes shell commands itself in either case —
//! this only gates whether the text gets inserted via the paste pipeline.

use crate::clipboard;
use crate::settings::{get_settings, write_settings, AppProfile, DictationMode, PasteMethod};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

const PENDING_PASTE_TTL: Duration = Duration::from_secs(60);

#[tauri::command]
#[specta::specta]
pub fn change_dictation_mode_setting(app: AppHandle, mode: DictationMode) {
    let mut settings = get_settings(&app);
    settings.dictation_mode = mode;
    write_settings(&app, settings);
}

#[tauri::command]
#[specta::specta]
pub fn change_app_profiles_setting(app: AppHandle, profiles: Vec<AppProfile>) {
    let mut settings = get_settings(&app);
    settings.app_profiles = profiles;
    write_settings(&app, settings);
}

#[derive(Default)]
pub struct PendingPasteState(pub Mutex<Option<PendingPaste>>);

pub struct PendingPaste {
    pub text: String,
    pub paste_method_override: Option<PasteMethod>,
    pub target: Option<crate::app_profile::FocusedWindowTarget>,
    pub created_at: Instant,
}

impl PendingPaste {
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > PENDING_PASTE_TTL
    }
}

#[tauri::command]
#[specta::specta]
pub fn confirm_pending_paste(app: AppHandle) -> Result<(), String> {
    let pending = {
        let state = app.state::<PendingPasteState>();
        let mut guard = state.0.lock().map_err(|e| e.to_string())?;
        guard.take()
    };

    match pending {
        Some(pending) => {
            if pending.is_expired() {
                return Err("The confirmation expired; paste was cancelled".to_string());
            }
            let target = pending
                .target
                .as_ref()
                .ok_or_else(|| "The original target window could not be identified".to_string())?;
            crate::app_profile::restore_focused_window(target)?;
            clipboard::paste_with_method_override(pending.text, app, pending.paste_method_override)
        }
        None => Err("No pending paste to confirm".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_paste_expires_after_ttl() {
        let pending = PendingPaste {
            text: "rm -rf ./build".to_string(),
            paste_method_override: None,
            target: None,
            created_at: Instant::now() - PENDING_PASTE_TTL - Duration::from_millis(1),
        };

        assert!(pending.is_expired());
    }
}

#[tauri::command]
#[specta::specta]
pub fn cancel_pending_paste(app: AppHandle) -> Result<(), String> {
    let state = app.state::<PendingPasteState>();
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    *guard = None;
    Ok(())
}
