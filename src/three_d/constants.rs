//! Physical and grid constants (SI units where applicable).

// Physical constants
pub const G: f64 = 6.67430e-11;
pub const C: f64 = 299_792_458.0;

// Black hole mass — Sagittarius A* (kg)
pub const BH_MASS: f64 = 8.54e36;

// Grid layout
pub const GRID_SIZE: i32 = 25;
pub const SPACING: f32 = 1e10;

// Vertical offsets for Flamm paraboloid warp
// Far-field reference: ≈ 2*sqrt(r_s*(17.7e10 - r_s)) ≈ 9.1e10 m
pub const WARP_OFFSET: f32 = -9.1e10;
pub const GRID_Y_SHIFT: f32 = -3e10;
