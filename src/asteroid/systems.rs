use bevy::{
	prelude::*,
	window::PrimaryWindow
};

use super::resources::*;
use super::components::*;

use crate::game::resources::*;
use crate::player::components::*;
use crate::player::resources::*;
use crate::score::resources::*;
use crate::bullet::components::*;


use super::{
	ASTEROID_HEIGHT,
	ASTEROID_WIDTH,
	ASTEROID_SPEED,
	MAX_ASTEROIDS
};

pub fn tick_asteroid_spawn_timer(mut timer: ResMut<SpawnAsteroidTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn spawn_asteroids_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<SpawnAsteroidTimer>,
    asteroid_query: Query<&Asteroid>,
    game_state: Res<GameState>
) {
    if game_state.is_game_over {
        return;
    }
    let window = window_query.get_single().unwrap();

    if timer.timer.finished() && asteroid_query.iter().count() < (MAX_ASTEROIDS as usize) {
        let rand_x = rand::random::<f32>() * window.width();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, window.height(), 0.0),
                texture: asset_server.load("textures/asteroid.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(ASTEROID_WIDTH, ASTEROID_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            Asteroid {
                direction: Vec2::new(0.0, -1.0),
                phase: 0.0,
            },
        ));

        timer.timer.reset();
    }
}

pub fn asteroid_movement(mut asteroid_query: Query<(&mut Transform, &mut Asteroid)>, time: Res<Time>) {
    for (mut transform, mut asteroid) in asteroid_query.iter_mut() {
        let amplitude: f32 = 1.0;
        let frequency: f32 = 0.02;
        let x: f32 = amplitude * f32::cos(frequency * (transform.translation.y + asteroid.phase));
        asteroid.direction = Vec2::new(x, -1.0);
        transform.translation +=
            Vec3::new(asteroid.direction.x, asteroid.direction.y, 0.0) *
            ASTEROID_SPEED *
            time.delta_seconds();
    }
}

pub fn bullet_hit_asteroid(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut bullet_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
    mut asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut() {
        for (asteroid_entity, asteroid_transform) in asteroid_query.iter_mut() {
            if bullet.is_enemy {
                continue;
            }
            let bullet_x = bullet_transform.translation.x;
            let bullet_y = bullet_transform.translation.y;
            let asteroid_x = asteroid_transform.translation.x;
            let asteroid_y = asteroid_transform.translation.y;

            if
                bullet_x > asteroid_x - ASTEROID_WIDTH / 2.0 &&
                bullet_x < asteroid_x + ASTEROID_WIDTH / 2.0 &&
                bullet_y > asteroid_y - ASTEROID_HEIGHT / 2.0 &&
                bullet_y < asteroid_y + ASTEROID_HEIGHT / 2.0
            {
                commands.entity(bullet_entity).despawn();
                commands.entity(asteroid_entity).despawn();
                score.value += 1;
                let sound_effect = asset_server.load("sounds/sfx_zap.ogg");
                audio.play(sound_effect);
            }
        }
    }
}

pub fn asteroid_hit_player(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    mut asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    mut lives: ResMut<PlayerLives>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut player_invulnerable_timer: ResMut<PlayerInvulnerableTimer>,
    mut player_respawn_timer: ResMut<PlayerRespawnTimer>
) {
    if let Ok((player_entity, player_transform, player)) = player_query.get_single_mut() {
        if player.is_invulnerable {
            return;
        }

        for (asteroid_entity, asteroid_transform) in asteroid_query.iter_mut() {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            let asteroid_x = asteroid_transform.translation.x;
            let asteroid_y = asteroid_transform.translation.y;

            if
                player_x > asteroid_x - ASTEROID_WIDTH / 2.0 &&
                player_x < asteroid_x + ASTEROID_WIDTH / 2.0 &&
                player_y > asteroid_y - ASTEROID_HEIGHT / 2.0 &&
                player_y < asteroid_y + ASTEROID_HEIGHT / 2.0
            {
                commands.entity(player_entity).despawn();
                commands.entity(asteroid_entity).despawn();
                score.value -= 10;
                let sound_effect = asset_server.load("sounds/sfx_lose.ogg");
                audio.play(sound_effect);
                lives.value -= 1;
                player_respawn_timer.timer.reset();
                player_invulnerable_timer.timer.reset();
            }
        }
    }
}

pub fn asteroid_cleanup(
    mut commands: Commands,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for (entity, transform) in asteroid_query.iter() {
        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        } else if transform.translation.y > window.height() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_asteroids(mut commands: Commands, asteroid_query: Query<Entity, With<Asteroid>>) {
    for entity in asteroid_query.iter() {
        commands.entity(entity).despawn();
    }
}