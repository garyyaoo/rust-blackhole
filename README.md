# Black Hole Simulation

Realtime black hole simulation, highlight accretion disk and spacetime curvature.

![Black Hole Simulation](assets/output-stars.gif)

Full demos in assets.

## Features

- **Spacetime curvature grid** — the background grid is warped by the Schwarzschild embedding formula, visualising spacetime curvature, coloured amber at the inner edge to yellow at the outer
- **Geodesic ray tracer accretion disk** — null geodesics integrated per-pixel on the GPU via Euler steps with proximity-based adaptive step size (smaller near the event horizon)
- **Orbital camera** — drag to orbit, scroll to zoom, implementing perspective matrix

## Physics

All distances are in SI metres. The black hole uses Sagittarius A* parameters:

| Quantity | Value |
|----------|-------|
| Mass | 8.54 × 10³⁶ kg |
| Schwarzschild radius r_s | ≈ 1.27 × 10¹⁰ m |
| Default camera radius | 2 × 10¹¹ m (~16 r_s) |

## Module layout

| File | Purpose |
|------|---------|
| `constants.rs` | Physical and grid constants |
| `scene.rs` | `BlackHole` and `GridObject` structs, and grid warping |
| `grid.rs` | Warped grid vertex/index generation |
| `camera.rs` | Orbital camera (azimuth, elevation, radius) |
| `math.rs` | Matrix for camera perspective |
| `shaders.rs` | GLSL source strings (grid + geodesic quad) |
| `gl_utils.rs` | Shader compilation and program linking |
| `mod.rs` | Entry point: window, GPU setup, render loop |