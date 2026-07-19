# Building Vozora

## Requirements

- Bun 1.3.14 or newer
- latest stable Rust toolchain
- platform-native Tauri 2 prerequisites
- Vulkan SDK for GPU-enabled Windows x64 and Linux builds

On Windows, install Visual Studio Build Tools with the Desktop development with
C++ workload. Do not build or run Vozora as administrator.

## Development

```powershell
bun install --frozen-lockfile
bun run tauri dev
```

If the source is on a network share, keep Rust outputs on a local disk:

```powershell
$env:CARGO_TARGET_DIR = "C:\cvzbuild"
bun run tauri build
```

## Verification

```powershell
bun run check:translations
bun run lint
bun run format:check
bun run build
cargo clippy --locked --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path src-tauri/Cargo.toml
```

Production bundles are written below the configured Cargo target directory in
`release/bundle/`. Windows produces NSIS and MSI packages; the maintained Linux
release target is x86_64 AppImage. Release automation creates a draft so assets,
signatures, updater metadata, and smoke-test evidence can be reviewed before
publication.
