# AGENTS.md — Vozora

## Product boundary

Vozora is a Tauri 2 desktop dictation app. The React/TypeScript frontend lives
in `src/`; the Rust backend lives in `src-tauri/src/`. Speech recognition is
local by default. Optional cloud post-processing is explicit and user-owned.

Never log transcription text, audio contents, API keys, clipboard contents, or
personal paths. Never execute transcribed commands. Text that looks destructive
in Terminal Command mode must remain behind the target-bound confirmation gate.

## Commands

```bash
bun install --frozen-lockfile
bun run check:translations
bun run lint
bun run format:check
bun run build
bun run test:playwright
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path src-tauri/Cargo.toml
bun run tauri build
```

Use a local `CARGO_TARGET_DIR` when the source is on a network share. Never run
Vozora elevated: global input and the mapped drive belong to the normal user
session.

## Engineering rules

- Preserve local-first behavior and make every network path visible in privacy docs.
- Store provider credentials in the operating-system credential store, never JSON or logs.
- Pin Git dependencies with `rev` and GitHub Actions with full commit SHAs.
- Validate frontend-provided file names before joining them to internal paths.
- Add user-facing strings to i18n; do not hardcode JSX copy.
- Keep `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, and `CHANGELOG.md` on the same release version.
- Use conventional commits. Do not publish releases or push without explicit authorization.

## Before handoff

Run the smallest relevant checks, state anything not run, review `git diff`, and
leave the repository buildable without discarding pre-existing work.
