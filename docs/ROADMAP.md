# KRL roadmap

## Platform policy

KRL is a mobile launcher. Android is the first implementation target. iOS is a separate feasibility and implementation track. Desktop operating systems are explicitly out of scope.

All application and launcher source code must be Rust. Android uses the built-in NativeActivity entry point through the Rust android-activity crate. No Java or Kotlin source files are part of the launcher.

Minecraft: Java Edition still needs a compatible virtual machine to execute the game bytecode. KRL may integrate with a separately packaged runtime from Rust, but that does not permit Java/Kotlin launcher source code.

## Milestones

### M0 — Repository foundation

- Rust workspace and crate boundaries.
- Core profile validation.
- Mojang version manifest data model and channel filtering.
- Safe JAR import, archive inspection, duplicate detection, and removal.
- Runtime launch-plan validation.
- FPS and frame-time metric calculations.
- Android NativeActivity entry point.
- CI checks and platform policy.

### M1 — Android application shell

- Native Rust UI and touch input.
- Application lifecycle and persistent settings.
- Android Storage Access Framework integration for user-selected files.
- Internal per-profile directory layout.
- Installable debug APK packaging.

### M2 — Version and asset pipeline

- Download and verify the official version manifest.
- Parse per-version metadata.
- Resolve libraries, assets, and client files.
- Resume interrupted downloads and verify checksums.
- Support releases, snapshots, old beta, and old alpha where metadata remains available.

### M3 — Runtime integration

- Runtime discovery and compatibility checks.
- ARM64 runtime packaging strategy.
- Launch Minecraft in a controlled mobile process.
- Collect crash reports and structured launch diagnostics.

### M4 — Mod and mod-loader support

- Per-profile mod directory.
- User-selected JAR import.
- Compatibility metadata and conflict warnings.
- Fabric first, then Quilt, Forge, and NeoForge after installation flows are implemented.
- Enable, disable, update, and remove mods without deleting user data.

### M5 — Performance engineering

- Repeatable device test matrix.
- Average FPS, 1% low, frame-time percentiles, memory, and thermal tracking.
- Renderer and runtime comparisons.
- Presets that are device-aware and reversible.
- No performance claims without measured results.

### M6 — iOS feasibility

- Prove Rust-native application lifecycle and packaging.
- Evaluate runtime execution restrictions and graphics integration.
- Confirm that a supported Java Edition runtime can operate under iOS platform rules.
- Do not promise game launch until a real device test succeeds.

## Release gate

A feature is not complete because it compiles. It must have tests, device-level validation where applicable, documented limitations, and no misleading UI state.
