# Changelog

All notable Vozora changes are documented here.

## [0.9.3] - 2026-07-18

### Security

- Moved cloud-provider API keys from settings JSON to the operating-system credential store.
- Removed transcript contents from persistent logs.
- Bound destructive-command confirmation to the original Windows process/window and added a 60-second expiry.
- Added a restrictive Content Security Policy and limited the asset protocol to the recordings directory.
- Pinned Git dependencies and GitHub Actions to immutable commits.

### Changed

- Updated supported frontend, Tauri, Rust, SQLite, transcription, and build dependencies.
- Restored the Windows x64 Vulkan backend declared by the product documentation.
- Added continuous integration, RustSec audit, Dependabot, and a reviewed draft-release workflow.
- Published current security, privacy, build, contribution, and agent guidance.

### Fixed

- Committed the updater dependency and version changes that were missing from `Cargo.lock` in v0.9.2.
- Rejected recording file names containing traversal components before joining internal paths.

## [0.9.2] - 2026-07-12

- Added signed in-app updates for Windows and a Linux AppImage release.
- Added Vozora release branding and updater feed configuration.

## [0.9.1] - 2026-07-12

- Initial public Vozora release based on Handy under the MIT license.
