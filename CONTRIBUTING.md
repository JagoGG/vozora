# Contributing to Vozora

Open an issue before large behavior or architecture changes. Small bug fixes,
tests, documentation, and translation corrections can go directly to a pull
request.

## Local setup

Install current stable Rust and Bun 1.3.14 or newer, then run:

```bash
bun install --frozen-lockfile
bun run build
cargo test --locked --manifest-path src-tauri/Cargo.toml
```

Before submitting, run the lint, format, translation, frontend build, Rust test,
and Clippy commands listed in `AGENTS.md`. Explain any check that cannot run on
your platform.

Keep user-facing text translatable. Do not add telemetry, persist secrets, log
transcripts, or weaken the destructive-command confirmation path. Preserve the
MIT attribution in `LICENSE` and `ATTRIBUTION.md`.

Use conventional commit prefixes such as `fix:`, `feat:`, `docs:`, `test:`, and
`chore:`.
