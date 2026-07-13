<p align="center">
  <img src="src/assets/vozora-isotipo.png" width="140" alt="Vozora" />
</p>

<h1 align="center">Vozora</h1>

<p align="center">
  <strong>Dictado por voz privado, 100% local, para Windows.</strong><br/>
  Pulsa una tecla, habla, y el texto aparece en la app que tengas delante.
</p>

---

## ¿Qué es Vozora?

Vozora es una aplicación de escritorio de **speech-to-text** que convierte tu voz en texto en **cualquier aplicación de Windows**: editores de código, terminales, navegadores, chats… Funciona con un atajo global **push-to-talk** (por defecto `PageDown`): mantienes la tecla, dictas, sueltas, y Vozora escribe el texto transcrito donde tengas el cursor.

La diferencia clave frente a servicios de dictado en la nube: **todo ocurre en tu máquina**. El audio nunca sale de tu ordenador — los modelos de reconocimiento de voz se descargan una vez y se ejecutan en local.

## Características

- **Push-to-talk global** — dicta en cualquier app sin cambiar de ventana; el texto se escribe donde esté el foco.
- **Transcripción 100% local** — motor `transcribe.cpp` con modelos GGUF (Whisper, Parakeet, Canary, Moonshine, GigaAM y más de 20 opciones descargables desde la propia app).
- **Detección de voz (VAD)** — usa Silero VAD para recortar silencios y no transcribir ruido.
- **Overlay en pantalla** — una píldora flotante muestra la onda de audio mientras grabas y el estado (grabando / transcribiendo), con caret animado en los colores de la marca.
- **Post-procesado opcional con LLM** — puedes configurar un proveedor compatible con la API de OpenAI para pulir la transcripción con prompts propios (por ejemplo: "convierte esto en una instrucción para un agente de código"). Es opcional; sin ello, nada sale de tu equipo.
- **Historial de transcripciones** con búsqueda.
- **22 idiomas de interfaz** y modelos multilingües.
- **Temas claro / oscuro / sistema** con la paleta de marca Vozora (azul `#1e7fd0`, navy `#0d2240`, lima `#b8c50f`).
- **Modo portable** — el instalador ofrece instalación normal o portable (carpeta autocontenida, sin registro ni accesos directos).

## Instalación

Descarga el instalador desde [Releases](../../releases):

- **`Vozora_x.y.z_x64-setup.exe`** (recomendado) — instalador NSIS con opción de modo portable. Instala en `%LOCALAPPDATA%\Vozora` sin pedir permisos de administrador.
- **`Vozora_x.y.z_x64_en-US.msi`** — alternativa MSI.

En el primer arranque, Vozora te guía para descargar un modelo de transcripción (recomendado: *Parakeet* para inglés o *Whisper* multilingüe) y probar el micrófono.

> **Importante:** no ejecutes Vozora como administrador. El atajo global y la escritura de texto funcionan en tu sesión de usuario normal; una instancia elevada bloquea la comunicación entre procesos.

## Cómo funciona por dentro

Vozora es una app **Tauri 2**: backend en **Rust** y frontend en **React + TypeScript** renderizado con WebView2.

```
┌─────────────────────────────────────────────────────────────┐
│                        VOZORA (Tauri 2)                      │
│                                                             │
│  Frontend (React + TS)          Backend (Rust)              │
│  ┌───────────────────┐          ┌────────────────────────┐  │
│  │ Ventana ajustes   │◄─ IPC ──►│ Atajo global (hotkey)  │  │
│  │ Onboarding        │          │ Captura de audio (cpal)│  │
│  │ Historial         │          │ Silero VAD (onnx)      │  │
│  │ Overlay grabación │          │ transcribe.cpp + GGUF  │  │
│  └───────────────────┘          │ Inyección de texto     │  │
│                                 │ Tray + settings store  │  │
│                                 └────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

El flujo de un dictado:

1. **Atajo** — el backend registra un hotkey global a nivel de sistema. Al pulsarlo, empieza la captura del micrófono y aparece el overlay.
2. **Grabación** — el audio se captura con `cpal` y pasa por **Silero VAD** (ONNX) para descartar silencio.
3. **Transcripción** — al soltar la tecla, el audio va al motor **transcribe.cpp**, que ejecuta el modelo GGUF elegido en CPU/GPU local.
4. **(Opcional) Post-proceso** — si has configurado un LLM, el texto pasa por tu prompt antes de escribirse.
5. **Escritura** — el resultado se inyecta como texto en la aplicación con foco (simulando teclado/portapapeles) y se guarda en el historial.

### Dónde guarda las cosas

| Qué | Ruta |
|---|---|
| Ajustes | `%APPDATA%\com.vozora.desktop\settings_store.json` |
| Modelos descargados | `%APPDATA%\com.vozora.desktop\models\` |
| Logs | `%LOCALAPPDATA%\com.vozora.desktop\logs\vozora.log` |
| Ejecutable (instalación normal) | `%LOCALAPPDATA%\Vozora\vozora.exe` |

### CLI para diagnóstico

El ejecutable acepta modo headless, útil para probar el backend sin GUI:

```
vozora.exe --list-models --json                 # lista modelos registrados
vozora.exe --transcribe-file audio.wav --json   # transcribe un WAV completo
```

## Estructura del repositorio

```
├── src/                    # Frontend React + TypeScript
│   ├── components/         #   UI (ajustes, onboarding, sidebar, iconos de marca)
│   ├── overlay/            #   Overlay flotante de grabación
│   ├── assets/             #   Logo y isotipo oficiales
│   ├── styles/theme.css    #   Paleta de marca (única fuente de verdad de colores)
│   └── i18n/               #   Traducciones (22 idiomas)
├── src-tauri/              # Backend Rust (Tauri 2)
│   ├── src/                #   Audio, VAD, transcripción, hotkeys, tray, settings
│   ├── icons/              #   Iconos de app (generados desde el isotipo)
│   ├── resources/          #   Iconos de tray, sonidos, VAD onnx
│   ├── nsis/installer.nsi  #   Plantilla del instalador (con modo portable)
│   └── tauri.conf.json     #   Configuración de app y bundle
├── public/                 # Estáticos (release notes in-app)
└── tests/                  # Tests Playwright
```

## Compilar desde el código

Requisitos: **Rust** (stable), **Bun**, y las build tools de Visual Studio (C++).

```powershell
bun install          # dependencias del frontend
bun x tauri dev      # desarrollo con hot-reload de la app completa
bun x tauri build    # build de producción → instaladores NSIS y MSI
```

Los instaladores quedan en `<target>/release/bundle/{nsis,msi}/`. Si el código vive en una unidad de red, define `CARGO_TARGET_DIR` hacia un disco local para acelerar la compilación de Rust.

Para regenerar los iconos de la app tras cambiar el isotipo:

```powershell
bun x tauri icon ruta/al/master-1024.png
```

## Privacidad

- El audio y las transcripciones **no salen de tu equipo**.
- No hay telemetría ni analítica.
- La única conexión de red por defecto es la **descarga de modelos** (una vez por modelo).
- Si activas el post-procesado con un LLM externo, ese texto sí viaja al proveedor que tú configures — es opt-in y configurable por completo.

## Licencia

MIT — ver [LICENSE](LICENSE). Este proyecto incorpora trabajo de terceros detallado en [ATTRIBUTION.md](ATTRIBUTION.md).
