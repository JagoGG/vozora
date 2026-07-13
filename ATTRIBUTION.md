# Attribution

Vozora is a fork of [Handy](https://github.com/cjpais/Handy) by CJ Pais,
licensed under the MIT License. This project is **not affiliated with or
endorsed by** the original author or the Handy project.

The original MIT copyright notice is preserved unmodified in [LICENSE](LICENSE).

## What Vozora changed from Handy

- **Rebrand**: product name, app identifier (`com.vozora.desktop`), window
  titles, tray text/icon, package/crate names (`vozora_app_lib`), and the
  `handy-keys` shortcut backend's in-repo wrapper (`vozora_keys.rs`) — see
  `SESSION_REPORT.md` for the exact file list.
- **Coding Mode dictation** (`src-tauri/src/coding_mode.rs`): a data-driven
  phrase table converting spoken punctuation/formatting commands, in English
  and Spanish, into literal characters.
- **Dictation modes and app profiles** (`src-tauri/src/settings.rs`,
  `src-tauri/src/app_profile.rs`): Literal / Natural-corrected / AI Prompt /
  Code / Terminal Command modes, plus per-application defaults matched
  against the focused window's title.
- **Destructive-command confirmation gate**
  (`coding_mode::looks_destructive`, `src-tauri/src/commands/dictation.rs`,
  `src/components/dictation/DestructiveCommandDialog.tsx`): Terminal Command
  mode withholds the auto-paste and asks for explicit confirmation when the
  transcription looks like a destructive shell command.
- New/updated docs: this file, `CONTRIBUTING.md`, `SECURITY.md`,
  `PRIVACY.md`, `CHANGELOG.md`, `docs/ARCHITECTURE.md`,
  `docs/PACKAGING.md`, and `README.md`.

## What Vozora deliberately left untouched

- All of Handy's core transcription/audio/ASR-model machinery,
  post-processing pipeline, overlay system, history, and settings
  infrastructure not listed above.
- Functional URLs pointing at Handy's real hosting infrastructure — model
  downloads still come from `blob.handy.computer` and the `handy-computer`
  Hugging Face org, since Vozora has no model-hosting of its own. Only
  cosmetic/branding strings were renamed; these URLs were left untouched so
  models keep downloading correctly.
- The upstream repository links used for HTTP `User-Agent`/`Referer` headers
  in `src-tauri/src/llm_client.rs` (`github.com/cjpais/Handy`) — changing
  these wasn't necessary for correctness and they identify the true origin
  of that inherited code path.
- `LICENSE`'s original copyright text.

## Thanks

Thanks to CJ Pais and the Handy contributors for building and open-sourcing
a genuinely forkable local speech-to-text app — Vozora would not exist
without that foundation.
