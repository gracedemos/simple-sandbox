use std::sync::{Arc, Mutex, mpsc::Receiver};

use crate::{
    player::{Player, PlayerPosition},
    voxel::{VOXEL_SIZE, Voxel, VoxelType},
    worldgen::{VoxelWorld, VoxelWorldRes},
};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

pub const CHUNK_WIDTH: usize = 32;
pub const CHUNK_HEIGHT: usize = 64;
pub const CHUNK_SIZE: usize = CHUNK_WIDTH * 2 * CHUNK_HEIGHT;
pub const RENDER_DISTANCE: i32 = 2;

pub struct Chunk {
    pub voxels: [[[Voxel; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            voxels: [[[Voxel::default(); CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
    }
}

impl Chunk {
    pub fn mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut i = 0;
        for z in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for x in 0..CHUNK_WIDTH {
                    let voxel = self.voxels[z][y][x];
                    let voxel_type = VoxelType::from(voxel.0);
                    match voxel_type {
                        VoxelType::Empty => continue,
                        VoxelType::Grass => {}
                        VoxelType::Dirt => {}
                    }

                    let h = VOXEL_SIZE / 2.0;
                    for dz in [-h, h] {
                        for dy in [-h, h] {
                            for dx in [-h, h] {
                                vertices.push([x as f32 + dx, y as f32 + dy, z as f32 + dz]);
                            }
                        }
                    }

                    let mut new_indices = vec![
                        // left
                        0, 2, 1, 1, 2, 3, // right
                        4, 5, 6, 5, 7, 6, // bottom
                        0, 1, 4, 1, 5, 4, // top
                        2, 6, 3, 3, 6, 7, // back
                        0, 4, 2, 2, 4, 6, // front
                        1, 3, 5, 3, 7, 5,
                    ];
                    for index in new_indices.iter_mut() {
                        *index += i;
                    }
                    indices.append(&mut new_indices);
                    i += 8;
                }
            }
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_indices(Indices::U32(indices))
    }
}

pub fn chunk_handler(
    mut commands: Commands,
    world: Res<VoxelWorldRes>,
    player_position_res: Res<PlayerPosition>,
    player_position: Single<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk_x = player_position.translation.x.floor() as i32 % CHUNK_WIDTH as i32;
    let chunk_y = player_position.translation.y.floor() as i32 % CHUNK_WIDTH as i32;

    for y in chunk_y - RENDER_DISTANCE..chunk_y + RENDER_DISTANCE {
        for x in chunk_x - RENDER_DISTANCE..chunk_x + RENDER_DISTANCE {
            let binding = world.0.chunks.lock().unwrap();
            let chunk = binding.get(&[x, y]);
            if let Some(chunk) = chunk {
                let world_x = (x * CHUNK_WIDTH as i32) as f32;
                let world_y = (y * CHUNK_WIDTH as i32) as f32;
                commands.spawn((
                    Mesh3d(meshes.add(chunk.mesh())),
                    MeshMaterial3d(
                        materials.add(StandardMaterial::from(Color::srgb(0.0, 1.0, 0.0))),
                    ),
                    Transform::from_xyz(world_x, 0.0, world_y),
                ));
            } else {
                continue;
            }
        }
    }
}

pub fn chunk_loader(world: Arc<VoxelWorld>, player_position: Arc<Mutex<Vec3>>) {
    for y in -RENDER_DISTANCE..RENDER_DISTANCE {
        for x in -RENDER_DISTANCE..RENDER_DISTANCE {
            world
                .chunks
                .lock()
                .unwrap()
                .insert([x, y], Chunk::default());
            world.gen_chunk([x, y]);
        }
    }
}
