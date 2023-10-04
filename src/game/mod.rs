use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod utils;
mod systems;

use systems::*;
use resources::*;

use crate::{asteroid, enemy, player, score, bullet, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugin(asteroid::AsteroidPlugin)
            .add_plugin(enemy::EnemyPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(score::ScorePlugin)
            .add_plugin(bullet::BulletPlugin)
            .init_resource::<GameState>()
            .add_startup_system(setup_camera)
            .add_system(gamepad_connections)
            .add_system(gamepad_input)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(transition_to_game_state)
            .add_system(transition_to_menu_state);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
