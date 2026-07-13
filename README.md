<p align="center">
  <img src="src/assets/vozora-isotipo.png" width="140" alt="Vozora" />
</p>

<h1 align="center">Vozora</h1>

<p align="center">
  <strong>Private, 100% local voice dictation for Windows — built for coding by voice.</strong><br/>
  Press a key, speak, and your words appear in whatever app you're using.
</p>

<p align="center">
  <a href="README.es.md">Léeme en español</a>
</p>

---

## What is Vozora?

Vozora is a desktop **speech-to-text** app that turns your voice into text in **any Windows application**: code editors, terminals, browsers, chat apps… It works with a global **push-to-talk** shortcut (default: `PageDown`): hold the key, dictate, release, and Vozora types the transcribed text wherever your cursor is.

The key difference from cloud dictation services: **everything happens on your machine**. Your audio never leaves your computer — the speech-recognition models are downloaded once and run locally.

## Built on Handy 🤝

Vozora is a **fork of [Handy](https://github.com/cjpais/Handy)**, the excellent open source (MIT) local speech-to-text app created by **CJ Pais**. Handy provides the foundation that makes this possible: the `transcribe.cpp` transcription pipeline, the settings system, and the Tauri scaffolding. Full credit for that foundation goes to CJ Pais and Handy's contributors — if you're looking for the original app, [it's here](https://github.com/cjpais/Handy).

Vozora takes that foundation in its own direction: **dictating to programming tools** (Claude Code, Cursor, terminals) safely and comfortably. It is not affiliated with or endorsed by CJ Pais. Full attribution details are in [ATTRIBUTION.md](ATTRIBUTION.md), and the original MIT license is preserved unmodified in [LICENSE](LICENSE).

### What Vozora adds on top of Handy

Functional improvements:

- **🎯 Coding Mode** — a dictation mode with a code-oriented phrase table: you speak in natural language and phrases are translated into programming symbols and constructs before being typed.
- **🛡️ Destructive-command confirmation** — if what you're about to dictate into a terminal looks like a dangerous command (delete, force, overwrite), Vozora holds it and asks for confirmation in a dialog before pasting. Dictating into a shell stops being scary.
- **👤 Per-application dictation profiles** — Vozora detects the focused window and can apply a different dictation mode depending on the target app (e.g., plain text in the browser, Coding Mode in the terminal).
- **📦 Portable mode in the installer** — choose between a normal install or a self-contained folder with no registry changes or shortcuts, right from the setup wizard.

Robustness and privacy fixes to the inherited engine:

- Fixed a **deadlock** in model loading: a failure during load left the `is_loading` state stuck and blocked later activations; loading now uses a guard that always releases.
- **Model-loading errors now surface in the UI** (they used to fail silently, making clicks look dead).
- **In-memory settings cache** (settings reads no longer hit disk on every access, with write-through consistency).
- **API keys are masked in debug logs** — a log dump can no longer leak your post-processing keys.
- A clean standalone identity: full rebrand, new palette and iconography, and its own HTTP headers.

## Features

- **Global push-to-talk** — dictate into any app without switching windows; text is typed wherever the focus is.
- **100% local transcription** — `transcribe.cpp` engine with GGUF models (Whisper, Parakeet, Canary, Moonshine, GigaAM and 20+ options downloadable from within the app).
- **Voice activity detection (VAD)** — uses Silero VAD to trim silence and avoid transcribing noise.
- **On-screen overlay** — a floating pill shows the audio waveform while you record, plus the current state (recording / transcribing).
- **Optional LLM post-processing** — configure any OpenAI-compatible provider to polish transcriptions with your own prompts. Optional; without it, nothing leaves your machine.
- **Transcription history** with search.
- **22 interface languages** (including English and Spanish) and multilingual models.
- **Light / dark / system themes**.

## Installation

Download the installer from [Releases](../../releases):

- **`Vozora_x.y.z_x64-setup.exe`** (recommended) — NSIS installer with a portable-mode option. Installs to `%LOCALAPPDATA%\Vozora` without requiring administrator rights.
- **`Vozora_x.y.z_x64_en-US.msi`** — MSI alternative for corporate deployments (Intune/SCCM).

On first launch, Vozora walks you through downloading a transcription model and testing your microphone.

**Recommended model: Nemotron Streaming 3.5** — live (streaming) multilingual transcription across **28 languages** (English, Spanish, French, German, Portuguese, Japanese, Chinese…), with a great balance of speed and accuracy. It's the model Vozora is developed and tested with daily.

> **Note:** the installer is not yet code-signed, so Windows SmartScreen may show a warning. That's the normal behavior for binaries without a signing certificate; you can build from this source yourself if you prefer to verify it.

> **Important:** don't run Vozora as administrator. The global shortcut and text injection work in your normal user session; an elevated instance breaks inter-process communication.

## How it works under the hood

Vozora is a **Tauri 2** app: a **Rust** backend and a **React + TypeScript** frontend rendered with WebView2.

```
┌─────────────────────────────────────────────────────────────┐
│                        VOZORA (Tauri 2)                      │
│                                                             │
│  Frontend (React + TS)          Backend (Rust)              │
│  ┌───────────────────┐          ┌────────────────────────┐  │
│  │ Settings window   │◄─ IPC ──►│ Global hotkey          │  │
│  │ Onboarding        │          │ Audio capture (cpal)   │  │
│  │ History           │          │ Silero VAD (onnx)      │  │
│  │ Recording overlay │          │ transcribe.cpp + GGUF  │  │
│  │ Confirm dialog    │          │ Coding Mode + profiles │  │
│  └───────────────────┘          │ Text injection         │  │
│                                 │ Tray + settings store  │  │
│                                 └────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

The flow of a dictation:

1. **Hotkey** — the backend registers a system-wide global hotkey. Pressing it starts microphone capture and shows the overlay.
2. **Recording** — audio is captured with `cpal` and passed through **Silero VAD** (ONNX) to discard silence.
3. **Transcription** — on key release, the audio goes to the **transcribe.cpp** engine, which runs your chosen GGUF model on local CPU/GPU.
4. **Dictation mode** — depending on the focused app's profile, the text may pass through **Coding Mode** (phrases → code) and the **destructive-command gate** if the target is a terminal.
5. **(Optional) Post-processing** — if you configured an LLM, the text runs through your prompt before being typed.
6. **Typing** — the result is injected as text into the focused application and saved to history.

### Where things are stored

| What | Path |
|---|---|
| Settings | `%APPDATA%\com.vozora.desktop\settings_store.json` |
| Downloaded models | `%APPDATA%\com.vozora.desktop\models\` |
| Logs | `%LOCALAPPDATA%\com.vozora.desktop\logs\vozora.log` |
| Executable (normal install) | `%LOCALAPPDATA%\Vozora\vozora.exe` |

### Diagnostic CLI

The executable supports a headless mode, useful for testing the backend without the GUI:

```
vozora.exe --list-models --json                 # list registered models
vozora.exe --transcribe-file audio.wav --json   # transcribe a full WAV file
```

## Repository structure

```
├── src/                    # React + TypeScript frontend
│   ├── components/         #   UI (settings, onboarding, sidebar, dictation dialogs)
│   ├── overlay/            #   Floating recording overlay
│   ├── assets/             #   Logo and mark
│   ├── styles/theme.css    #   Palette (single source of truth for colors)
│   └── i18n/               #   Translations (22 languages)
├── src-tauri/              # Rust backend (Tauri 2)
│   ├── src/                #   Audio, VAD, transcription, hotkeys, coding_mode,
│   │                       #   app_profile, tray, settings
│   ├── icons/              #   App icons
│   ├── resources/          #   Tray icons, sounds, VAD onnx
│   ├── nsis/installer.nsi  #   Installer template (with portable mode)
│   └── tauri.conf.json     #   App and bundle configuration
├── public/                 # Static files (in-app release notes)
└── tests/                  # Playwright tests
```

## Building from source

Requirements: **Rust** (stable), **Bun**, and the Visual Studio C++ build tools.

```powershell
bun install          # frontend dependencies
bun x tauri dev      # full-app development with hot reload
bun x tauri build    # production build → NSIS and MSI installers
```

Installers land in `<target>/release/bundle/{nsis,msi}/`. If the code lives on a network drive, point `CARGO_TARGET_DIR` at a local disk to speed up the Rust build.

## Privacy

- Audio and transcriptions **never leave your machine**.
- No telemetry, no analytics.
- The only network connection by default is **model downloads** (once per model).
- If you enable post-processing with an external LLM, that text does travel to the provider you configure — it's fully opt-in and configurable.

## Project status

Vozora is under active development (v0.9.x). Windows is the primary and only thoroughly tested platform; the Linux/macOS code is inherited from the base and not yet verified in this fork. Issues and suggestions are welcome.

## License

MIT — see [LICENSE](LICENSE). Based on [Handy](https://github.com/cjpais/Handy) by CJ Pais; full attribution in [ATTRIBUTION.md](ATTRIBUTION.md).
