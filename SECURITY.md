# Security policy

## Supported versions

Security fixes are applied to the latest `0.9.x` release. Older releases should
be upgraded before troubleshooting.

## Reporting a vulnerability

Do not include secrets, recordings, transcripts, or exploit details in a public
issue. Contact the repository owner through the
[JagoGG GitHub profile](https://github.com/JagoGG) with a minimal description and
request a private channel. Non-sensitive bugs may use the public issue tracker.

## Security model

Vozora transcribes locally and never executes dictated text. It can place text
into another application using clipboard and keyboard simulation. Terminal
Command mode adds defense in depth: text matching destructive-command patterns
is withheld, expires after 60 seconds, and can only be pasted after Vozora
restores and verifies the original Windows process and window.

The heuristic is not a shell parser. It can produce false positives and false
negatives, so users remain responsible for reviewing commands before execution.

Provider API keys use the operating-system credential store. GitHub updater
artifacts are verified with the public key in `src-tauri/tauri.conf.json`.
Release signing secrets must exist only in GitHub Actions secrets.
