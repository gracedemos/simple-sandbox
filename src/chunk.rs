use crate::{
    player::Player,
    voxel::{VOXEL_SIZE, Voxel, VoxelType},
    worldgen::VoxelWorldRes,
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

#[derive(Resource)]
pub struct LastChunkPos(pub IVec2);

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
                        0, 2, 1, 1, 2, 3, // right
                        4, 5, 6, 5, 7, 6, // left
                        0, 1, 4, 1, 5, 4, // bottom
                        2, 6, 3, 3, 6, 7, // top
                        0, 4, 2, 2, 4, 6, // back
                        1, 3, 5, 3, 7, 5, // front
                    ];
                    for index in new_indices.iter_mut() {
                        *index += i;
                    }
                    for (i, neighbor) in self.get_neighbors(x, y, z).iter().enumerate() {
                        if !neighbor {
                            for j in 0..6 {
                                indices.push(new_indices[i * 6 + j]);
                            }
                        }
                    }
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

    fn get_neighbors(&self, x: usize, y: usize, z: usize) -> [bool; 6] {
        let mut neighbors = [false; 6];
        if z > 0 && z < CHUNK_WIDTH - 1 {
            match VoxelType::from(self.voxels[z - 1][y][x].0) {
                VoxelType::Empty => {}
                _ => neighbors[0] = true,
            }
            match VoxelType::from(self.voxels[z + 1][y][x].0) {
                VoxelType::Empty => {}
                _ => neighbors[1] = true,
            }
        }
        if y > 0 && y < CHUNK_HEIGHT - 1 {
            match VoxelType::from(self.voxels[z][y - 1][x].0) {
                VoxelType::Empty => {}
                _ => neighbors[2] = true,
            }
            match VoxelType::from(self.voxels[z][y + 1][x].0) {
                VoxelType::Empty => {}
                _ => neighbors[3] = true,
            }
        }
        if x > 0 && x < CHUNK_WIDTH - 1 {
            match VoxelType::from(self.voxels[z][y][x - 1].0) {
                VoxelType::Empty => {}
                _ => neighbors[4] = true,
            }
            match VoxelType::from(self.voxels[z][y][x + 1].0) {
                VoxelType::Empty => {}
                _ => neighbors[5] = true,
            }
        }

        neighbors
    }
}

pub fn chunk_spawner(
    mut commands: Commands,
    mut world: ResMut<VoxelWorldRes>,
    player_position: Single<&Transform, With<Player>>,
    mut last_chunk_pos: ResMut<LastChunkPos>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chunk_x = (player_position.translation.x / CHUNK_WIDTH as f32).floor() as i32;
    let chunk_y = (player_position.translation.z / CHUNK_WIDTH as f32).floor() as i32;
    if chunk_x == last_chunk_pos.0.x && chunk_y == last_chunk_pos.0.y {
        return;
    }
    last_chunk_pos.0.x = chunk_x;
    last_chunk_pos.0.y = chunk_y;

    for y in chunk_y - RENDER_DISTANCE..chunk_y + RENDER_DISTANCE {
        for x in chunk_x - RENDER_DISTANCE..chunk_x + RENDER_DISTANCE {
            let chunk = world.0.chunks.get(&[x, y]);
            if let None = chunk {
                world.0.chunks.insert([x, y], Chunk::default());
                world.0.gen_chunk([x, y]);
                let chunk = world.0.chunks.get(&[x, y]).unwrap();
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
