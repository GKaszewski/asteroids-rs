use bevy::prelude::*;

use crate::AppState;

use systems::layout::*;
use systems::interactions::*;

mod systems;
mod components;
mod styles;

pub struct GameOverPlugin;


impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)))
            .add_systems(
                    (
                        interact_with_back_button,
                        interact_with_quit_button
                    ).in_set(OnUpdate(AppState::GameOver)
                )
        );
    }
}

