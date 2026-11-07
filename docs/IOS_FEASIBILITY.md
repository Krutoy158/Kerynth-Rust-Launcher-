# iOS feasibility track

iOS is an intended platform, not a currently supported target.

The application and launcher logic must remain Rust. Before committing to an iOS release, the project must prove all of the following on a physical iPhone or iPad:

- A Rust-native application lifecycle and installable package.
- File access through supported iOS document and application-container APIs.
- A compatible graphics path for the game renderer.
- A viable way to execute the Minecraft Java Edition runtime within iOS platform and distribution rules.
- Stable memory behavior and thermal performance during sustained play.
- A workable sign-in flow that does not embed launcher logic in Swift or Objective-C source files.

The largest unknown is not compiling Rust for iOS: it is whether the game runtime, dynamic code execution requirements, and graphics stack can operate acceptably under iOS restrictions. No iOS game-launch claim should be made until a physical-device prototype proves it.
