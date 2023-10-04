use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod utils;
mod systems;

use systems::*;
use resources::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().add_systems(
            (update_score, check_highscore, game_over)
                .in_set(OnUpdate(AppState::Game))
        );
    }
}
