use bevy::{
	prelude::*,
	window::PrimaryWindow,
};

use super::components::*;
use super::BULLET_SPEED;

pub fn bullet_movement(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in bullet_query.iter_mut() {
        transform.translation +=
            Vec3::new(bullet.direction.x, bullet.direction.y, 0.0) *
            BULLET_SPEED *
            time.delta_seconds();
    }
}

pub fn bullet_cleanup(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for (entity, transform) in bullet_query.iter() {
        if transform.translation.y > window.height() {
            commands.entity(entity).despawn();
        } else if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_bullets(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, _) in bullet_query.iter() {
        commands.entity(entity).despawn();
    }
}