use std::collections::HashMap;

use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use crate::{
    chunk::{CHUNK_HEIGHT, CHUNK_WIDTH, Chunk},
    voxel::{self, Voxel},
};

pub const WORLD_SCALE: f64 = 0.5;
pub const WORLD_FREQUENCY: f64 = 0.01;

pub struct VoxelWorld {
    pub chunks: HashMap<[i32; 2], Chunk>,
    pub noise: Fbm<Perlin>,
}

#[derive(Resource)]
pub struct VoxelWorldRes(pub VoxelWorld);

impl VoxelWorld {
    pub fn new(seed: u32) -> Self {
        let chunks = HashMap::new();
        let noise = Fbm::new(seed).set_frequency(WORLD_FREQUENCY);

        VoxelWorld { chunks, noise }
    }

    pub fn gen_chunk(&mut self, position: [i32; 2]) {
        let chunk = self.chunks.get_mut(&position).unwrap();
        for z in 0..CHUNK_WIDTH {
            for x in 0..CHUNK_WIDTH {
                let position_f64 = [
                    (position[0] * 32 + x as i32) as f64,
                    (position[1] * 32 + z as i32) as f64,
                ];
                let mut height = self.noise.get(position_f64) * WORLD_SCALE;
                height = (height + 1.0) / 2.0;
                height = (height * CHUNK_HEIGHT as f64).floor();
                height = height.clamp(0.0, CHUNK_HEIGHT as f64);
                let height = height as u32;

                for y in 0..height {
                    chunk.voxels[z][y as usize][x] = Voxel(voxel::VOXEL_GRASS);
                }
            }
        }
    }
}
