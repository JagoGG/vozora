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
use tauri::{AppHandle, Manager};

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
            clipboard::paste_with_method_override(pending.text, app, pending.paste_method_override)
        }
        None => Err("No pending paste to confirm".to_string()),
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
