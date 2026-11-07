# Performance policy

KRL's own code is written in Rust and is kept small, asynchronous where I/O benefits, and measurable. Rust alone does not make Minecraft render faster. Game performance depends on the virtual machine, game version, renderer, graphics drivers, device thermal state, and installed mods.

## Required measurements

- Average FPS over a fixed capture window.
- 1% low FPS derived from the slowest 1% of frame-time samples.
- Median, p95, and p99 frame time.
- Peak and steady-state memory use.
- CPU and GPU utilization where the platform exposes it.
- Thermal state and throttling.
- Startup time and time to first rendered frame.

## Test protocol

1. Record device model, OS version, ABI, game version, runtime version, renderer, and mod set.
2. Use the same world, camera path, render distance, graphics settings, and capture duration.
3. Allow the device to reach a comparable thermal starting state.
4. Run a baseline and at least three repeated runs per configuration.
5. Keep raw frame-time samples and summarize median results across runs.
6. Do not compare results collected under different conditions as if they were equivalent.

The benchmark crate reports whether a sample set is large enough for percentile results to be treated as reliable. A small sample is useful for tests but not for public 1% low claims.
