use bevy::prelude::*;

#[derive(Component)]
pub struct Physics {
    pub velocity: Vec3,
}

pub fn physics(mut query: Query<(&Physics, &mut Transform)>) {
    for (physics, mut transform) in query.iter_mut() {
        transform.translation += physics.velocity;
    }
}
