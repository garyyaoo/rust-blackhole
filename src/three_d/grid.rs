//! Warped spacetime grid geometry.

use crate::three_d::constants::WARP_OFFSET;

use super::constants::{GRID_SIZE, SPACING};
use super::scene::{BlackHole, SceneObject};

pub fn generate_grid(bh: &BlackHole, objects: &[SceneObject]) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for z in 0..=GRID_SIZE {
        for x in 0..=GRID_SIZE {
            let world_x = (x - GRID_SIZE / 2) as f32 * SPACING;
            let world_z = (z - GRID_SIZE / 2) as f32 * SPACING;
            let mut world_y = bh.warp_y(world_x, world_z);
            for obj in objects {
                world_y += obj.warp_contribution(world_x, world_z);
            }
            vertices.push([world_x, world_y + WARP_OFFSET, world_z]);
        }
    }

    for z in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let i = (z * (GRID_SIZE + 1) + x) as u32;
            indices.push(i);
            indices.push(i + 1);
            indices.push(i);
            indices.push(i + (GRID_SIZE + 1) as u32);
        }
    }

    (vertices, indices)
}
