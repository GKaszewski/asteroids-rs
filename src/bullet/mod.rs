use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod utils;
mod systems;

use systems::*;

use crate::{AppState, game::SimulationState};

pub const BULLET_SPEED: f32 = 800.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (bullet_movement, bullet_cleanup)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
        );
        app.add_system(
            despawn_bullets.in_schedule(OnExit(AppState::Game))
        );
    }
}
