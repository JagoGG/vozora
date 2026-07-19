# Privacy

Vozora is local-first and has no telemetry or analytics.

## Data kept locally

- Microphone audio is processed by the selected local model.
- Saved recordings and transcription history remain in the app data directory.
- Settings remain in `settings_store.json`.
- Provider API keys are stored in the operating-system credential store, not in
  the settings JSON. Portable installations therefore do not carry API keys.
- Logs contain operational metadata and error messages, not transcript contents.

## Network access

Vozora may connect to the network for:

1. model downloads initiated by the user;
2. signed update checks against this repository when update checks are enabled;
3. optional post-processing sent to the provider and endpoint selected by the user.

Without cloud post-processing, recorded audio and transcript text are not sent to
an AI provider. Model hosts and GitHub can still observe ordinary download/update
metadata such as IP address and request time.

When upgrading from an older release, Vozora migrates plaintext provider keys to
the secure store. If that store is unavailable, it preserves the legacy value to
avoid silent data loss and reports the migration failure in the log; new keys are
not accepted until secure storage is available.

## Deletion

Users can delete individual history entries and recordings from the app. Removing
the app data directory deletes local settings, models, history, recordings, and
logs. API keys must be removed in Vozora before uninstalling, or from the
operating-system credential manager afterward.
