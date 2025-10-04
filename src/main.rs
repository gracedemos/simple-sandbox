use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    window::WindowMode,
};

use crate::{
    chunk::LastChunkPos,
    worldgen::{VoxelWorld, VoxelWorldRes},
};

mod chunk;
mod physics;
mod player;
mod voxel;
mod worldgen;

fn main() {
    let world = VoxelWorldRes(VoxelWorld::new(0));
    let last_chunk_pos = LastChunkPos(IVec2::new(-1, -1));

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            WireframePlugin::default(),
        ))
        .insert_resource(world)
        .insert_resource(last_chunk_pos)
        .add_systems(Startup, player::spawn_player)
        .add_systems(FixedUpdate, physics::physics)
        .add_systems(
            Update,
            (
                player::update_player_direction,
                player::update_player_velocity,
                chunk::chunk_spawner,
                toggle_wireframe,
            ),
        )
        .run();
}

fn toggle_wireframe(
    input: Res<ButtonInput<KeyCode>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    if input.just_pressed(KeyCode::Tab) {
        wireframe_config.global = !wireframe_config.global;
    }
}
