use bevy::prelude::*;

use crate::bullet::components::Bullet;
use crate::player::{PLAYER_HEIGHT, PLAYER_WIDTH};

use super::components::*;

pub fn spawn_player (
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	window: &Window,
	is_invulnerable: bool
) {
	commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, 0.0, 0.0),
            texture: asset_server.load("textures/player.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            ..default()
        },
        Player {
            is_invulnerable: is_invulnerable,
        },
	));
}

pub fn handle_player_attack (
	commands: &mut Commands,
    asset_server: Res<AssetServer>,
    player_transform: &Transform,
    audio: Res<Audio>
) {
	commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(player_transform.translation),
                    texture: asset_server.load("textures/bullet.png"),
                    ..default()
                },
                Bullet {
                    direction: Vec2::new(0.0, 1.0),
                    is_enemy: false,
                },
            ));
            let sound_effect = asset_server.load("sounds/sfx_laser1.ogg");
            audio.play(sound_effect);
}