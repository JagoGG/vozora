<p align="center">
  <img src="src/assets/vozora-isotipo.png" width="140" alt="Vozora" />
</p>

<h1 align="center">Vozora</h1>

<p align="center">
  <strong>Dictado por voz privado, 100% local, para Windows — pensado para programar con la voz.</strong><br/>
  Pulsa una tecla, habla, y el texto aparece en la app que tengas delante.
</p>

<p align="center">
  <a href="README.md">Read me in English</a>
</p>

---

## ¿Qué es Vozora?

Vozora es una aplicación de escritorio de **speech-to-text** que convierte tu voz en texto en **cualquier aplicación de Windows**: editores de código, terminales, navegadores, chats… Funciona con un atajo global **push-to-talk** (por defecto `PageDown`): mantienes la tecla, dictas, sueltas, y Vozora escribe el texto transcrito donde tengas el cursor.

La diferencia clave frente a servicios de dictado en la nube: **todo ocurre en tu máquina**. El audio nunca sale de tu ordenador — los modelos de reconocimiento de voz se descargan una vez y se ejecutan en local.

## Basado en Handy 🤝

Vozora es un **fork de [Handy](https://github.com/cjpais/Handy)**, la estupenda app open source (MIT) de speech-to-text local creada por **CJ Pais**. Handy aporta la base sobre la que esto es posible: el pipeline de transcripción con `transcribe.cpp`, el sistema de ajustes y el andamiaje Tauri. Todo el crédito de esa base es suyo y de sus contribuidores — si buscas la app original, [está aquí](https://github.com/cjpais/Handy).

Vozora toma esa base y la lleva en una dirección propia: **dictar a herramientas de programación** (Claude Code, Cursor, terminales) de forma segura y cómoda. No está afiliado ni respaldado por CJ Pais. El detalle completo de la atribución está en [ATTRIBUTION.md](ATTRIBUTION.md) y la licencia MIT original se conserva intacta en [LICENSE](LICENSE).

### Qué aporta Vozora sobre Handy

Mejoras funcionales:

- **🎯 Coding Mode** — un modo de dictado con tabla de frases orientada a código: hablas en lenguaje natural y las frases se traducen a símbolos y construcciones de programación antes de escribirse.
- **🛡️ Confirmación de comandos destructivos** — si lo que vas a dictar a una terminal parece un comando peligroso (borrar, forzar, sobreescribir), Vozora lo retiene y te pide confirmación en un diálogo antes de pegarlo. Dictar a una shell deja de dar miedo.
- **👤 Perfiles de dictado por aplicación** — Vozora detecta la ventana con foco y puede aplicar un modo de dictado distinto según la app de destino (p. ej., texto normal en el navegador, Coding Mode en la terminal).
- **📦 Modo portable en el instalador** — instalación normal o carpeta autocontenida sin registro ni accesos directos, a elegir en el propio setup.

Robustez y privacidad sobre el motor heredado:

- Arreglado un **deadlock** en la carga de modelos: un fallo durante la carga dejaba el estado `is_loading` atascado y bloqueaba activaciones posteriores; ahora la carga usa un guard que siempre libera.
- Los **errores de carga de modelos se muestran en la UI** (antes fallaban en silencio y parecía que el clic no hacía nada).
- **Caché de ajustes en memoria** (lectura de settings sin tocar disco en cada acceso, con coherencia garantizada en escritura).
- **Las claves API se enmascaran en los logs** de depuración — un volcado de log ya no puede filtrar tus claves de post-procesado.
- Identidad propia limpia: rebrand completo, nueva paleta e iconografía, cabeceras HTTP propias.

## Características

- **Push-to-talk global** — dicta en cualquier app sin cambiar de ventana; el texto se escribe donde esté el foco.
- **Transcripción 100% local** — motor `transcribe.cpp` con modelos GGUF (Whisper, Parakeet, Canary, Moonshine, GigaAM y más de 20 opciones descargables desde la propia app).
- **Detección de voz (VAD)** — usa Silero VAD para recortar silencios y no transcribir ruido.
- **Overlay en pantalla** — una píldora flotante muestra la onda de audio mientras grabas y el estado (grabando / transcribiendo).
- **Post-procesado opcional con LLM** — configura cualquier proveedor compatible con la API de OpenAI para pulir la transcripción con tus propios prompts. Opcional; sin ello, nada sale de tu equipo.
- **Historial de transcripciones** con búsqueda.
- **22 idiomas de interfaz** (incluidos inglés y español) y modelos multilingües.
- **Temas claro / oscuro / sistema**.

## Instalación

Descarga el instalador desde [Releases](../../releases):

- **`Vozora_x.y.z_x64-setup.exe`** (recomendado) — instalador NSIS con opción de modo portable. Instala en `%LOCALAPPDATA%\Vozora` sin pedir permisos de administrador.
- **`Vozora_x.y.z_x64_en-US.msi`** — alternativa MSI para despliegues corporativos (Intune/SCCM).

En el primer arranque, Vozora te guía para descargar un modelo de transcripción y probar el micrófono.

**Modelo recomendado: Nemotron Streaming 3.5** — transcripción en vivo (streaming) multilingüe en **28 idiomas** (español, inglés, francés, alemán, portugués, japonés, chino…), con muy buen equilibrio entre velocidad y precisión. Es el modelo con el que se desarrolla y prueba Vozora a diario.

> **Nota:** el instalador aún no está firmado digitalmente, así que Windows SmartScreen puede mostrar un aviso. Es el comportamiento normal para binarios sin certificado de firma; puedes compilarlo tú mismo desde este código si prefieres verificarlo.

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
│  │ Diálogo confirmar │          │ Coding Mode + perfiles │  │
│  └───────────────────┘          │ Inyección de texto     │  │
│                                 │ Tray + settings store  │  │
│                                 └────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

El flujo de un dictado:

1. **Atajo** — el backend registra un hotkey global a nivel de sistema. Al pulsarlo, empieza la captura del micrófono y aparece el overlay.
2. **Grabación** — el audio se captura con `cpal` y pasa por **Silero VAD** (ONNX) para descartar silencio.
3. **Transcripción** — al soltar la tecla, el audio va al motor **transcribe.cpp**, que ejecuta el modelo GGUF elegido en CPU/GPU local.
4. **Modo de dictado** — según el perfil de la app con foco, el texto puede pasar por **Coding Mode** (frases → código) y por la **puerta de comandos destructivos** si el destino es una terminal.
5. **(Opcional) Post-proceso** — si has configurado un LLM, el texto pasa por tu prompt antes de escribirse.
6. **Escritura** — el resultado se inyecta como texto en la aplicación con foco y se guarda en el historial.

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
│   ├── components/         #   UI (ajustes, onboarding, sidebar, diálogos de dictado)
│   ├── overlay/            #   Overlay flotante de grabación
│   ├── assets/             #   Logo y isotipo
│   ├── styles/theme.css    #   Paleta (única fuente de verdad de colores)
│   └── i18n/               #   Traducciones (22 idiomas)
├── src-tauri/              # Backend Rust (Tauri 2)
│   ├── src/                #   Audio, VAD, transcripción, hotkeys, coding_mode,
│   │                       #   app_profile, tray, settings
│   ├── icons/              #   Iconos de app
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

## Privacidad

- El audio y las transcripciones **no salen de tu equipo**.
- No hay telemetría ni analítica.
- La única conexión de red por defecto es la **descarga de modelos** (una vez por modelo).
- Si activas el post-procesado con un LLM externo, ese texto sí viaja al proveedor que tú configures — es opt-in y configurable por completo.

## 🚀 Visión y Roadmap

Lo que ves hoy es la **fase uno** de una idea mucho más grande: **convertir tu voz en una forma de primera clase de manejar un ordenador.**

Ahora mismo, Vozora escribe lo que dices. El destino es un **asistente completo por voz para tu máquina**: hablas con tu ordenador y tu ordenador hace el trabajo — abre aplicaciones, organiza ventanas, ejecuta comandos, navega por tus archivos, automatiza lo repetitivo de tu día. Todo diseñado local-first y privado-first, para que la inteligencia te sirva sin que tu vida salga de tu equipo.

- ✅ **Fase 1 — Dictado por voz (estás aquí).** Dictado push-to-talk en cualquier app, transcripción 100% local, Coding Mode, perfiles por aplicación y puerta de seguridad para comandos peligrosos. Sólido, útil por sí solo, y la base sobre la que se construye todo lo demás.
- 🔜 **Fase 2 — Comandos de voz.** Más allá de escribir texto: "abre el navegador", "cambia a la terminal", "cierra esta ventana". Control directo de aplicaciones y ventanas por voz, con la misma filosofía de confirmación para todo lo arriesgado.
- 🔭 **Fase 3 — Control conversacional del ordenador.** Encadenar trabajo real en lenguaje natural: "abre mi proyecto, corre los tests y dime qué falló". Conciencia del contexto en pantalla, habilidades por aplicación y acciones multi-paso — como tener unas manos capaces que escuchan.
- 🌍 **Fase 4 — Una plataforma local de agente por voz, extensible.** La meta a largo plazo: una plataforma que cualquiera pueda extender con sus propias habilidades y automatizaciones, en cualquier idioma, ayudando a personas de todo el mundo a procesar su día a día de forma más simple, más rápida y mejor — sin renunciar a su privacidad.

### Cómo puedes ayudar

Este proyecto no persigue dinero — persigue **personas**: conexiones, colaboradores y usuarios a los que de verdad les ayude. Si la visión resuena contigo:

- ⭐ **Dale una estrella al repo** — es la forma más simple de decir "sigue adelante" y ayuda a que otros lo encuentren.
- 🐛 **Pruébalo y abre issues** — cada reporte desde hardware real lo hace mejor.
- 🔧 **Contribuye** — código, traducciones, documentación, ideas. El roadmap de arriba tiene sitio para muchas manos.
- 📣 **Compártelo** — con cualquiera que programe con el teclado todo el día y nunca haya probado hablarle a su máquina.

## Estado del proyecto

Vozora está en desarrollo activo (v0.9.x, fase 1 del roadmap de arriba) y saca actualizaciones con regularidad — la app comprueba nuevas versiones y se actualiza sola desde las releases de este repositorio. Windows es la plataforma prioritaria y la única probada a fondo; el código de Linux/macOS se hereda de la base y aún no está verificado en este fork. Los issues y sugerencias son bienvenidos.

## Licencia

MIT — ver [LICENSE](LICENSE). Basado en [Handy](https://github.com/cjpais/Handy) de CJ Pais; atribución completa en [ATTRIBUTION.md](ATTRIBUTION.md).
