use bevy::{
	prelude::*,
	window::PrimaryWindow,
};

use super::components::*;
use super::resources::*;

use super::{
	ENEMY_HEIGHT,
	ENEMY_SPEED,
	ENEMY_WIDTH,
	MAX_ENEMIES,
};

use crate::game::resources::*;
use crate::player::components::*;
use crate::player::resources::*;
use crate::player::{
    PLAYER_HEIGHT,
    PLAYER_WIDTH,
};
use crate::score::resources::*;
use crate::bullet::components::*;


pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_state: Res<GameState>
) {
    if game_state.is_game_over {
        return;
    }

    let window = window_query.get_single().unwrap();

    let rand_x = rand::random::<f32>() * window.width();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(rand_x, window.height(), 0.0),
            texture: asset_server.load("textures/spaceship.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
                ..default()
            },
            ..default()
        },
        Enemy {
            direction: Vec2::new(0.0, -1.0),
            phase: 0.0,
        },
    ));
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        let amplitude: f32 = 3.0;
        let frequency: f32 = 0.05;
        let x: f32 = amplitude * f32::cos(frequency * (transform.translation.y + enemy.phase));
        enemy.direction = Vec2::new(x, -1.0);
        transform.translation +=
            Vec3::new(enemy.direction.x, enemy.direction.y, 0.0) *
            ENEMY_SPEED *
            time.delta_seconds();
    }
}

pub fn tick_enemy_spawn_timer(mut timer: ResMut<SpawnEnemyTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<SpawnEnemyTimer>,
    enemy_query: Query<&Enemy>,
    game_state: Res<GameState>
) {
    if game_state.is_game_over {
        return;
    }
    let window = window_query.get_single().unwrap();

    if timer.timer.finished() && enemy_query.iter().count() < (MAX_ENEMIES as usize) {
        let rand_x = rand::random::<f32>() * window.width();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, window.height(), 0.0),
                texture: asset_server.load("textures/spaceship.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(0.0, -1.0),
                phase: 0.0,
            },
        ));

        timer.timer.reset();
    }
}

pub fn bullet_hit_enemy(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut bullet_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut() {
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            if bullet.is_enemy {
                continue;
            }
            let bullet_x = bullet_transform.translation.x;
            let bullet_y = bullet_transform.translation.y;
            let enemy_x = enemy_transform.translation.x;
            let enemy_y = enemy_transform.translation.y;

            if
                bullet_x > enemy_x - ENEMY_WIDTH / 2.0 &&
                bullet_x < enemy_x + ENEMY_WIDTH / 2.0 &&
                bullet_y > enemy_y - ENEMY_HEIGHT / 2.0 &&
                bullet_y < enemy_y + ENEMY_HEIGHT / 2.0
            {
                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();
                score.value += 1;
                let sound_effect = asset_server.load("sounds/sfx_zap.ogg");
                audio.play(sound_effect);
            }
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
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

        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            let enemy_x = enemy_transform.translation.x;
            let enemy_y = enemy_transform.translation.y;

            if
                player_x > enemy_x - ENEMY_WIDTH / 2.0 &&
                player_x < enemy_x + ENEMY_WIDTH / 2.0 &&
                player_y > enemy_y - ENEMY_HEIGHT / 2.0 &&
                player_y < enemy_y + ENEMY_HEIGHT / 2.0
            {
                commands.entity(player_entity).despawn();
                commands.entity(enemy_entity).despawn();
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

pub fn enemy_bullet_hit_player(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    mut bullet_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
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
        for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut() {
            if !bullet.is_enemy {
                continue;
            }

            let bullet_x = bullet_transform.translation.x;
            let bullet_y = bullet_transform.translation.y;
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;

            if
                bullet_x > player_x - PLAYER_WIDTH / 2.0 &&
                bullet_x < player_x + PLAYER_WIDTH / 2.0 &&
                bullet_y > player_y - PLAYER_HEIGHT / 2.0 &&
                bullet_y < player_y + PLAYER_HEIGHT / 2.0
            {
                commands.entity(player_entity).despawn();
                commands.entity(bullet_entity).despawn();
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

pub fn enemy_attack_timer_tick(mut attack_timer: ResMut<AttackEnemyTimer>, time: Res<Time>) {
    attack_timer.timer.tick(time.delta());
}

pub fn enemy_attack(
    mut commands: Commands,
    mut attack_timer: ResMut<AttackEnemyTimer>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>
) {
    if attack_timer.timer.finished() {
        for (_, enemy_transform) in enemy_query.iter_mut() {
            let random = rand::random::<f32>();
            if random < 0.5 {
                continue;
            }

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(enemy_transform.translation),
                    texture: asset_server.load("textures/bullet.png"),
                    ..default()
                },
                Bullet {
                    direction: Vec2::new(0.0, -1.0),
                    is_enemy: true,
                },
            ));
            let sound_effect = asset_server.load("sounds/sfx_laser2.ogg");
            audio.play(sound_effect);
        }
        attack_timer.timer.reset();
    }
}

pub fn enemy_cleanup(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for (entity, transform) in enemy_query.iter() {
        if transform.translation.y < 0.0 {
            commands.entity(entity).despawn();
        } else if transform.translation.y > window.height() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, _) in enemy_query.iter() {
        commands.entity(entity).despawn();
    }
}