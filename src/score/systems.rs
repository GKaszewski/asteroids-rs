use bevy::prelude::*;
use super::resources::*;

use crate::AppState;
use crate::game::resources::*;
use crate::game::utils::handle_transition_to_state;
use crate::player::resources::*;
use crate::enemy::components::*;
use crate::bullet::components::*;

pub fn check_highscore(mut score: ResMut<Score>) {
    if score.value > score.high_score {
        score.high_score = score.value;
    }
}


pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    lives: Res<PlayerLives>,
    mut enemy_query: Query<Entity, With<Enemy>>,
    mut bullet_query: Query<Entity, With<Bullet>>,
    audio: Res<Audio>,
    mut game_state: ResMut<GameState>,
    app_state: Res<State<AppState>>,
    next_state: ResMut<NextState<AppState>>,
) {
    if lives.is_changed() && lives.value == 0 {
        for entity in enemy_query.iter_mut() {
            commands.entity(entity).despawn();
        }

        for entity in bullet_query.iter_mut() {
            commands.entity(entity).despawn();
        }

        println!("Game Over! Score: {}", score.value);
        let game_over_sound = asset_server.load("sounds/sfx_twoTone.ogg");
        audio.play(game_over_sound);
        game_state.is_game_over = true;
        handle_transition_to_state(app_state, next_state, AppState::GameOver);
    }
}