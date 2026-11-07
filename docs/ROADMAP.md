# KRL roadmap

## Platform policy

KRL is a mobile launcher. Android is the first implementation target. iOS is a separate feasibility and implementation track. Desktop operating systems are explicitly out of scope.

All application and launcher source code must be Rust. Android uses the built-in NativeActivity entry point through the Rust android-activity crate. No Java or Kotlin source files are part of the launcher.

Minecraft: Java Edition still needs a compatible virtual machine to execute the game bytecode. KRL may integrate with a separately packaged runtime from Rust, but that does not permit Java/Kotlin launcher source code.

## Original implementation queue

Work through these milestones in order. An unfinished earlier milestone always has priority over later milestones and newly requested features.

### M0 — Repository foundation

- [x] Rust workspace and crate boundaries.
- [x] Core profile validation.
- [x] Mojang version manifest data model and channel filtering.
- [x] Safe JAR import, archive inspection, duplicate detection, and removal.
- [x] Runtime launch-plan validation.
- [x] FPS and frame-time metric calculations.
- [x] Persistent KRL storage layout, format marker, and per-instance directory model.
- [x] Android NativeActivity entry point.
- [x] CI checks and platform policy.

### M1 — Android application shell

- [x] Native Rust UI shell with Play, Versions, Mods, Accounts, and Settings navigation.
- [x] Android NativeActivity event loop and eframe window integration.
- [ ] Touch and text-entry behavior verified on physical Android devices.
- [ ] Microsoft account sign-in and secure session storage.
- [ ] Application lifecycle and persistent settings.
- [ ] Installable debug APK packaging.

### M2 — Version and asset pipeline

- [ ] Download and verify the official version manifest.
- [ ] Parse per-version metadata.
- [ ] Resolve libraries, assets, and client files.
- [ ] Reuse shared libraries and assets across instances.
- [ ] Resume interrupted downloads and verify checksums before reusing files.
- [ ] Support releases, snapshots, old beta, and old alpha where metadata remains available.

### M3 — Runtime integration

- [ ] Runtime discovery and compatibility checks.
- [ ] ARM64 runtime packaging strategy.
- [ ] Launch Minecraft in a controlled mobile process.
- [ ] Collect crash reports and structured launch diagnostics.

### M4 — Mod and mod-loader support

- [ ] Per-instance mod directory in shared KRL storage.
- [ ] User-selected JAR import.
- [ ] Compatibility metadata and conflict warnings.
- [ ] Fabric first, then Quilt, Forge, and NeoForge after installation flows are implemented.
- [ ] Enable, disable, update, and remove mods without deleting user data.

### M5 — Performance engineering

- [ ] Repeatable device test matrix.
- [ ] Average FPS, 1% low, frame-time percentiles, memory, and thermal tracking.
- [ ] Renderer and runtime comparisons.
- [ ] Presets that are device-aware and reversible.
- [ ] No performance claims without measured results.

### M6 — iOS feasibility

- [ ] Prove Rust-native application lifecycle and packaging.
- [ ] Evaluate runtime execution restrictions and graphics integration.
- [ ] Confirm that a supported Java Edition runtime can operate under iOS platform rules.
- [ ] Do not promise game launch until a real device test succeeds.

## Appended user-request backlog

These requests were added after the original milestones. They must stay behind all unfinished work above, and must be completed in the order shown. Any future request is appended after the last item here; it must not interrupt an earlier task.

### R1 — Persistent shared KRL folder and reinstall recovery

- [ ] Connect the Rust storage layout to Android Storage Access Framework.
- [ ] Let the user choose a shared Documents directory and create the KRL folder there.
- [ ] Store versions, libraries, assets, runtimes, instances, mods, and worlds outside app-specific storage.
- [ ] After reinstall, let the user reselect the existing KRL folder if Android no longer grants the old URI permission.
- [ ] Validate storage metadata and scan existing versions and instances.
- [ ] Reuse complete files only after verifying required metadata and checksums; do not redownload valid existing files.
- [ ] Never overwrite or delete an unrecognized folder just because it is named KRL.

### R2 — Offline launch after valid account authorization

- [ ] Allow previously installed game files to launch without a network connection after a valid online account sign-in.
- [ ] Store and refresh authorized session data securely.
- [ ] Do not fabricate an authenticated identity or represent a nickname-only profile as a licensed account.

## Release gate

A feature is not complete because it compiles. It must have tests, device-level validation where applicable, documented limitations, and no misleading UI state.
