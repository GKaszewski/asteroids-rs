use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;
use super::resources::*;
use super::utils::*;

use super::{
	PLAYER_HEIGHT,
	PLAYER_SPEED,
	PLAYER_WIDTH,
};

pub fn setup_player(
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    spawn_player(commands, asset_server, window, false);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        direction = direction.normalize_or_zero();
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_width = PLAYER_WIDTH / 2.0;
        let half_player_height = PLAYER_HEIGHT / 2.0;
        let x_min = half_player_width;
        let x_max = window.width() - half_player_width;
        let y_min = half_player_height;
        let y_max = window.height() - half_player_height;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        } else if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn player_attack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    audio: Res<Audio>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            handle_player_attack(&mut commands, asset_server, player_transform, audio)
        }
    }
}

pub fn respawn_player(
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    lives: Res<PlayerLives>,
    player_respawn_timer: ResMut<PlayerRespawnTimer>,
    mut player_query: Query<&mut Player>
) {
    if player_query.iter_mut().count() > 0 {
        return;
    }

    let window = window_query.get_single().unwrap();

    if player_respawn_timer.timer.just_finished() && lives.value > 0 {
        spawn_player(commands, asset_server, window, true);
    }
}

pub fn player_respawn_timer_tick(
    mut player_respawn_timer: ResMut<PlayerRespawnTimer>,
    time: Res<Time>,
    lives: Res<PlayerLives>
) {
    if lives.value > 0 {
        player_respawn_timer.timer.tick(time.delta());
    }
}

pub fn player_invulnerable_timer_tick(
    mut player_invulnerable_timer: ResMut<PlayerInvulnerableTimer>,
    time: Res<Time>,
    lives: Res<PlayerLives>
) {
    if lives.value > 0 {
        player_invulnerable_timer.timer.tick(time.delta());
    }
}

pub fn player_invulnerable(
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    mut sprite_query: Query<&mut Sprite, With<Player>>,
    player_invulnerable_timer: Res<PlayerInvulnerableTimer>
) {
    if let Ok((_, mut player)) = player_query.get_single_mut() {
        if let Ok(mut sprite) = sprite_query.get_single_mut() {
            if player.is_invulnerable {
                sprite.color = Color::rgb(1.0, 0.0, 0.0);
            } else {
                sprite.color = Color::rgb(1.0, 1.0, 1.0);
            }
        }

        if player_invulnerable_timer.timer.finished() {
            player.is_invulnerable = false;
        }
    }
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    for (entity, _) in player_query.iter() {
        commands.entity(entity).despawn();
    }
}