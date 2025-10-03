pub const VOXEL_SIZE: f32 = 1.0;

// Voxel Flags
pub const VOXEL_GRASS: u8 = 0b10000000;
pub const VOXEL_DIRT: u8 = 0b10000001;

#[derive(Debug, PartialEq, Eq)]
pub enum VoxelType {
    Empty,
    Grass,
    Dirt,
}

impl From<u8> for VoxelType {
    fn from(value: u8) -> Self {
        if !(value >> 7 & 1 == 1) {
            VoxelType::Empty
        } else if value == VOXEL_GRASS {
            VoxelType::Grass
        } else if value == VOXEL_DIRT {
            VoxelType::Dirt
        } else {
            VoxelType::Empty
        }
    }
}

impl From<VoxelType> for u8 {
    fn from(value: VoxelType) -> Self {
        match value {
            VoxelType::Empty => 0,
            VoxelType::Grass => VOXEL_GRASS,
            VoxelType::Dirt => VOXEL_DIRT,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Voxel(pub u8);

impl Default for Voxel {
    fn default() -> Self {
        Voxel(0)
    }
}
