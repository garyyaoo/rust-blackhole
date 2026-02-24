//! Black hole and grid-warping objects.

use super::constants::{BH_MASS, C, G, GRID_Y_SHIFT, WARP_OFFSET};

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
        GRID_Y_SHIFT + delta_y + WARP_OFFSET
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
        delta_y + WARP_OFFSET
    }
}

/// Default list of extra objects that warp the grid (e.g. stars).
pub fn grid_objects() -> Vec<GridObject> {
    vec![
        // GridObject { x: 4e11, z: 0.0, mass: 1.98892e30 },
        // GridObject { x: 0.0, z: 4e11, mass: 1.98892e30 },
    ]
}
