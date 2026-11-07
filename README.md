# Kerynth Rust Launcher (KRL)

A Rust-only mobile launcher project for Minecraft: Java Edition. KRL is focused on measurable performance, reliable game-version management, and per-profile mod management.

## Project status

KRL is at the foundation stage. The repository currently contains a Rust workspace, profile validation, version-manifest parsing and channel filtering, safe JAR import and inspection, runtime-plan validation, frame-time metrics, and an Android native entry point.

The Rust core now includes offline player identity creation from a username, including Minecraft-compatible offline UUID generation. The Android sign-in screen and actual offline game launch are not implemented yet. Microsoft sign-in, verified downloads, runtime packaging, mod-loader installation, and a real game launch also remain incomplete.

## Platform scope

- **Android ARM64:** first implementation and validation target.
- **iOS:** planned feasibility track; support is not claimed until a physical-device prototype proves the runtime and graphics path.
- **Desktop:** explicitly out of scope. KRL is a phone/tablet launcher.

## Rust-only source policy

All KRL application and launcher logic is written in Rust. Do not add Java, Kotlin, Swift, or Objective-C application source files. Android currently uses the built-in `NativeActivity` entry point through the Rust `android-activity` crate.

Minecraft: Java Edition itself consists of Java bytecode and requires a compatible virtual machine to run. KRL will integrate with that external runtime from Rust; this does not make the launcher source code Java-based. The runtime is a game execution requirement, not a launcher implementation language.

The Android NativeActivity route has limited integration with the system soft keyboard. The mobile UI and sign-in input flow must be designed around the Rust-only constraint rather than silently adding platform-language source.

## Workspace

- `krl-core`: shared profile and loader models plus local offline-player identity.
- `krl-versions`: official version-manifest parsing and channel filtering.
- `krl-mods`: JAR import, archive inspection, duplicate detection, listing, and removal.
- `krl-runtime`: runtime compatibility checks and launch-plan validation.
- `krl-bench`: average FPS, 1% low, and frame-time calculations.
- `krl-android`: Rust NativeActivity entry point.

## Build and test

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

GitHub Actions runs Rust checks and an Android ARM64 compile check. A successful native-library compile is not the same as a complete installable APK or a successful Minecraft launch.

## Account modes

KRL is planned to support both Microsoft sign-in and a local Offline Account mode. Offline mode asks only for a username and creates a deterministic offline UUID locally; it does not contact Microsoft or claim that the player owns or has authenticated a Minecraft account. Offline identities are for local/offline play and servers configured to accept offline identities. Servers requiring official account authentication will reject them. The current Rust core contains the identity model only; the Android UI and end-to-end game launch are still pending.

## Performance policy

Rust alone does not make Minecraft faster. Performance depends on the game runtime, renderer, graphics drivers, version, settings, mods, device memory, and thermal state. No FPS claims will be made without repeatable physical-device benchmarks.

See [docs/PERFORMANCE.md](docs/PERFORMANCE.md) and [docs/ROADMAP.md](docs/ROADMAP.md).

## Legal

Minecraft is a trademark of Mojang Studios. KRL is an independent third-party project and is not affiliated with or endorsed by Mojang or Microsoft. Users must obtain game files through legitimate channels. KRL will not distribute proprietary game assets.

## License

Apache-2.0. See [LICENSE](LICENSE).
