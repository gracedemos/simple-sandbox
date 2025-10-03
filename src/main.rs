use std::sync::{Arc, Mutex, mpsc};

use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    window::WindowMode,
};

use crate::{
    player::PlayerPosition,
    worldgen::{VoxelWorld, VoxelWorldRes},
};

mod chunk;
mod physics;
mod player;
mod voxel;
mod worldgen;

fn main() {
    let world = VoxelWorldRes(Arc::new(VoxelWorld::new(0)));
    let player_position = PlayerPosition(Arc::new(Mutex::new(player::SPAWN_POSITION)));
    let world_1 = world.0.clone();
    let player_position_1 = player_position.0.clone();
    let _ = std::thread::spawn(move || chunk::chunk_loader(world_1, player_position_1));

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
        .insert_resource(player_position)
        .add_systems(Startup, player::spawn_player)
        .add_systems(FixedUpdate, physics::physics)
        .add_systems(
            Update,
            (
                player::update_player_direction,
                player::update_player_velocity,
                chunk::chunk_handler,
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
