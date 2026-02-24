//! Black hole and grid-warping objects.

use super::constants::{BH_MASS, C, G, GRID_Y_SHIFT};

pub struct BlackHole {
    pub x: f32,
    pub z: f32,
    pub mass: f64,
    pub r_s: f32,
}

impl BlackHole {
    pub fn new() -> Self {
        let r_s = (2.0 * G * BH_MASS / (C * C)) as f32;
        Self {
            x: 0.0,
            z: 0.0,
            mass: BH_MASS,
            r_s,
        }
    }

    // Grid warping at (x, y) due to this blackhole
    pub fn warp_y(&self, x: f32, z: f32) -> f32 {
        let dx = x - self.x;
        let dy = GRID_Y_SHIFT;
        let dz = z - self.z;
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        let delta_y = 2.0 * (self.r_s * (dist - self.r_s)).sqrt();
        GRID_Y_SHIFT + delta_y
    }
}

/// Extra massive object that warps the grid (e.g. star). Position in x-z plane.
pub struct GridObject {
    pub x: f32,
    pub z: f32,
    pub mass: f64,
}

impl GridObject {
    fn r_s(&self) -> f32 {
        (2.0 * G * self.mass / (C * C)) as f32
    }

    /// Flamm paraboloid warp contribution at (world_x, world_z), added to grid y.
    pub fn warp_contribution(&self, world_x: f32, world_z: f32) -> f32 {
        let r_s = self.r_s();
        let dx = world_x - self.x;
        let dz = world_z - self.z;
        let dy = GRID_Y_SHIFT;
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        let delta_y = 2.0 * (r_s * (dist - r_s)).sqrt();
        delta_y
    }
}

/// Sphere visible in the geodesic ray tracer (position, visual radius, mass, RGB colour).
/// `radius` is the visual sphere size passed to the shader.
/// `mass` is used for gravitational effects (grid warp) via `r_s()`.
pub struct SceneObject {
    pub position: [f32; 3],
    pub radius:   f32,   // visual radius (metres) — NOT the Schwarzschild radius
    pub mass:     f64,
    pub color:    [f32; 3],
}

impl SceneObject {
    /// Schwarzschild radius of this object, for gravitational calculations.
    pub fn r_s(&self) -> f32 {
        (2.0 * G * self.mass / (C * C)) as f32
    }

    pub fn new(position: [f32; 3], visual_radius: f32, mass: f64, color: [f32; 3]) -> Self {
        Self { position, radius: visual_radius, mass, color }
    }

    /// Flamm paraboloid warp contribution at (world_x, world_z), added to grid y.
    pub fn warp_contribution(&self, world_x: f32, world_z: f32) -> f32 {
        let r_s = self.r_s();
        let dx = world_x - self.position[0];
        let dz = world_z - self.position[2];
        let dy = GRID_Y_SHIFT;
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        let delta_y = 2.0 * (r_s * (dist - r_s)).sqrt();
        delta_y
    }
}

pub fn scene_objects() -> Vec<SceneObject> {
    vec![
        SceneObject::new([-3e11, 1e11,  2e11], 4e10, 2e30, [1.0, 1.0, 0.0]), // yellow star
        SceneObject::new([-3e11, 0.0, -1e11], 4e10, 2e30, [0.0, 0.5, 1.0]), // blue star
    ]
}
