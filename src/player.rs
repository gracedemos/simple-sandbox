use std::sync::{Arc, Mutex, mpsc::Sender};

use bevy::{input::mouse::AccumulatedMouseMotion, math::VectorSpace, prelude::*};

use crate::physics::Physics;

pub const MOVE_SPEED: f32 = 0.1;
pub const ACCELERATION: f32 = 20.0;
pub const LOOK_SENSITIVITY: f32 = 0.005;
pub const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2;
pub const SPAWN_POSITION: Vec3 = Vec3 {
    x: 0.0,
    y: 65.0,
    z: 0.0,
};

#[derive(Resource)]
pub struct PlayerPosition(pub Arc<Mutex<Vec3>>);

#[derive(Component)]
pub struct Player {
    look_sense: f32,
    move_speed: f32,
    acceleration: f32,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player {
            look_sense: LOOK_SENSITIVITY,
            move_speed: MOVE_SPEED,
            acceleration: ACCELERATION,
        },
        Physics {
            velocity: Vec3::ZERO,
        },
        Transform::from_xyz(SPAWN_POSITION.x, SPAWN_POSITION.y, SPAWN_POSITION.z),
        Projection::from(PerspectiveProjection {
            fov: 75.0_f32.to_radians(),
            ..Default::default()
        }),
        Visibility::Visible,
        children![(
            Camera3d::default(),
            Transform::from_xyz(0.0, 2.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y)
        )],
    ));
}

pub fn update_player_direction(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut param_set: ParamSet<(
        Single<(&Player, &mut Transform)>,
        Single<&mut Transform, With<Camera3d>>,
    )>,
) {
    let delta = mouse_motion.delta;
    let look_sense = {
        let (player, mut transform) = param_set.p0().into_inner();
        let look_sense = player.look_sense;

        transform.rotate_y(-delta.x * look_sense);
        look_sense
    };

    let mut camera = param_set.p1();
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch - delta.y * look_sense).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}

pub fn update_player_velocity(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&Player, &Transform, &mut Physics)>,
) {
    let (player, transform, mut physics) = player.into_inner();

    let mut direction = Vec3::ZERO;
    let mut vertical = 0.0;
    for key in input.get_pressed() {
        match key {
            KeyCode::KeyW => direction.x = -1.0,
            KeyCode::KeyS => direction.x = 1.0,
            KeyCode::KeyD => direction.z = -1.0,
            KeyCode::KeyA => direction.z = 1.0,
            KeyCode::Space => direction.y = 1.0,
            KeyCode::ShiftLeft => direction.y = -1.0,
            _ => {}
        }
    }

    if direction == Vec3::ZERO {
        let target_velocity = Vec3::ZERO;
        physics.velocity = physics
            .velocity
            .move_towards(target_velocity, time.delta_secs() * player.acceleration);
    } else {
        let basis = Mat3::from_cols(
            transform.local_x().as_vec3(),
            transform.local_y().as_vec3(),
            transform.local_z().as_vec3(),
        );
        let target_velocity = basis * Vec3::new(direction.x, direction.y, direction.z).normalize();
        physics.velocity = physics.velocity.move_towards(
            target_velocity * player.move_speed,
            time.delta_secs() * player.acceleration,
        );
    }
}
