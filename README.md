# Kerynth Rust Launcher (KRL)

A Rust-first launcher project focused on running Minecraft: Java Edition on mobile devices with measurable performance, reliable version management, and per-profile mod management.

## Project status

KRL is at the foundation stage. The repository currently establishes the Rust workspace, domain models, version-manifest parsing, safe JAR mod management, runtime process preparation, performance metrics, and an Android native entry point. A complete launcher UI, Microsoft authentication, verified asset/library downloads, mod-loader installation, and game launch orchestration are not implemented yet.

## Rust-only source policy

All KRL application and launcher logic is written in Rust. The repository must not contain Java or Kotlin source files. Android uses the operating system's `NativeActivity` entry point and a Rust native library.

Minecraft: Java Edition itself is Java bytecode and requires a compatible JVM at runtime. KRL will manage and invoke that external runtime from Rust; this does not make the launcher code Java-based. A JVM is a runtime requirement of the game, not an optional launcher language choice.

## Targets

- Android ARM64 is the first implementation and validation target.
- iOS is a separate feasibility track. Java runtime execution, JIT availability, app distribution rules, and graphics integration must be proven before promising support.

## Workspace

- `krl-core`: shared launcher and profile models.
- `krl-versions`: Mojang version manifest parsing and lookup.
- `krl-mods`: safe import, inspection, listing, and removal of JAR mod files.
- `krl-runtime`: JVM validation and Rust-side process specification.
- `krl-bench`: frame-time and 1% low calculations.
- `krl-android`: Android NativeActivity entry point implemented in Rust.

## Build and test

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Android ARM64 cross-builds are checked by GitHub Actions. A successful native-library build is not the same as a complete installable APK or a successful Minecraft launch.

## Performance policy

No FPS claims are made without repeatable measurements on identified devices, game versions, renderers, settings, and mod sets. Benchmarks must include average FPS, 1% low, frame-time distribution, memory use, and thermal behavior.

## Legal

Minecraft is a trademark of Mojang Studios. KRL is an independent third-party project and is not affiliated with or endorsed by Mojang or Microsoft. Users must obtain game files through legitimate channels. KRL will not distribute proprietary game assets.

## License

Apache-2.0. See [LICENSE](LICENSE).
