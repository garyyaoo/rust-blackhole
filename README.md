# Black Hole Simulation

Realtime black hole simulation implemented in Rust.

![Black Hole Simulation](assets/output-stars.gif)

## Features

- **Spacetime curvature grid** — the background grid is warped by the Schwarzschild embedding formula, visualising the spacetime curvature from the blackhole.
- **Geodesic ray tracer accretion disk** — light geodesics integrated per-pixel on the GPU via Euler. This creates the black hole's accretion disk from a light source stemming from the camera.
- **Orbital camera** — drag to orbit perspective, scroll to zoom, implemented using the perspective of an orbital camera.
- **Stars emulation** - simulate the effect on light from two neighboring stars with blue light and yellow light.

Inspired by youtube video [Simulating Blackholes in C++](https://www.youtube.com/watch?v=8-B6ryuBkCM)

## Running locally

```
git clone https://github.com/garyyaoo/rust-blackhole.git
cargo run
```

## Physics

All distances are in SI metres. The black hole uses [Sagittarius A*'s](https://en.wikipedia.org/wiki/Sagittarius_A*) parameters:

| Quantity | Value |
|----------|-------|
| Mass | 8.54 × 10³⁶ kg |
| Event horizon r_s | ≈ 1.27 × 10¹⁰ m |


## Module layout

| File | Purpose |
|------|---------|
| `constants.rs` | Physical and grid constants |
| `scene.rs` | `BlackHole` and `GridObject` structs, and grid warping |
| `grid.rs` | Warped grid vertex/index generation |
| `camera.rs` | Orbital camera (azimuth, elevation, radius) |
| `math.rs` | Matrix math for camera perspective |
| `shaders.rs` | GLSL source strings (grid + geodesic quad) |
| `gl_utils.rs` | Shader compilation and program linking |
| `mod.rs` | Entry point: window, GPU setup, render loop |
