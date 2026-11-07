# Kerynth Rust Launcher (KRL)

A Rust-only mobile launcher project for Minecraft: Java Edition. KRL is focused on measurable performance, reliable game-version management, persistent shared game storage, and per-instance mod management.

## Project status

KRL is at the foundation stage. The repository contains a Rust workspace, profile validation, version-manifest parsing and channel filtering, safe JAR import and inspection, runtime-plan validation, frame-time metrics, a filesystem storage-layout crate, and an Android-native Rust UI shell.

The Android UI currently provides navigation for Play, Versions, Mods, Accounts, and Settings. Actions that depend on unfinished account, download, runtime, or storage integrations remain disabled and are identified as unfinished. This is an interface shell, not a complete launcher.

The storage crate defines the shared KRL directory, a versioned storage marker, shared game-data directories, and isolated per-instance directories. The Android Storage Access Framework adapter, automatic version indexing, verified reuse of existing downloads, installable APK packaging, Microsoft sign-in, runtime packaging, mod-loader installation, and a real game launch are not implemented yet.

## Platform scope

- **Android ARM64:** first implementation and validation target.
- **iOS:** planned feasibility track; support is not claimed until a physical-device prototype proves the runtime and graphics path.
- **Desktop:** explicitly out of scope. KRL is a phone/tablet launcher.

## Rust-only source policy

All KRL application and launcher logic is written in Rust. Do not add Java, Kotlin, Swift, or Objective-C application source files. Android uses the built-in NativeActivity entry point through the Rust android-activity crate and the Rust eframe UI.

Minecraft: Java Edition itself consists of Java bytecode and requires a compatible virtual machine to run. KRL will integrate with that external runtime from Rust; this does not make the launcher source code Java-based. The runtime is a game execution requirement, not a launcher implementation language.

## Workspace

- krl-core: shared profile and loader models.
- krl-versions: official version manifest parsing and channel filtering.
- krl-mods: JAR import, archive inspection, duplicate detection, listing, and removal.
- krl-runtime: runtime compatibility checks and launch-plan validation.
- krl-bench: average FPS, 1% low, and frame-time calculations.
- krl-storage: persistent shared-directory layout, storage marker, instance paths, and path validation.
- krl-android: Android NativeActivity entry point and Rust UI shell.

## Persistent KRL folder

KRL stores game data in a shared folder named KRL, intended to be created inside a user-selected Documents directory. Installed versions, libraries, assets, and runtimes are shared. Each instance has its own mods, configuration, saves, resource packs, and logs.

This location is deliberately outside Android app-specific storage so game data can remain after the launcher is uninstalled. After reinstalling, Android may require the user to select the existing KRL folder again. KRL then validates its marker and reuses complete, verified game files instead of downloading them again. The SAF picker and this end-to-end restore flow are planned, not yet implemented.

See docs/STORAGE.md for the directory contract and Android storage requirements.

## Build and test

Run cargo fmt --all -- --check, cargo test --workspace --all-targets, and cargo clippy --workspace --all-targets -- -D warnings.

GitHub Actions runs Rust checks and an Android ARM64 compile check. A successful native-library compile is not the same as a complete installable APK or a successful Minecraft launch.

## Account modes

KRL is planned to support Microsoft sign-in and offline play after a successful account sign-in. The launcher should securely preserve the authorized local session and allow previously installed game versions to start without a network connection when the session and required files remain valid. Offline play must never fabricate an authenticated identity or bypass account ownership checks. Microsoft sign-in, secure session persistence, and end-to-end offline launch are not implemented yet.

## Performance policy

Rust alone does not make Minecraft faster. Performance depends on the game runtime, renderer, graphics drivers, version, settings, mods, device memory, and thermal state. No FPS claims will be made without repeatable physical-device benchmarks.

See docs/PERFORMANCE.md, docs/ROADMAP.md, and docs/STORAGE.md.

## Legal

Minecraft is a trademark of Mojang Studios. KRL is an independent third-party project and is not affiliated with or endorsed by Mojang or Microsoft. Users must obtain game files through legitimate channels. KRL will not distribute proprietary game assets.

## License

Apache-2.0. See LICENSE.
