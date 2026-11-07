# Contributing to KRL

## Source policy

- Application and launcher logic must be Rust.
- Do not add Java or Kotlin source files.
- Do not add desktop application targets.
- Keep Android as the first validated platform and keep iOS work isolated until its feasibility gates pass.

## Quality requirements

- Keep modules focused and avoid speculative abstractions.
- Add tests for parsing, validation, filesystem changes, and performance calculations.
- Never swallow errors that affect user data or launch correctness.
- Do not claim a feature works unless it has been exercised.
- Do not claim FPS improvements without a repeatable device benchmark.
- Keep game assets and proprietary runtime payloads out of the source repository.

## Checks

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Pull requests should state what was tested, on which target, and which limitations remain.
