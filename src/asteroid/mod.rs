use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod utils;
mod systems;

pub const ASTEROID_SPEED: f32 = 50.0;
pub const MAX_ASTEROIDS: u32 = 3;
pub const ASTEROID_WIDTH: f32 = 70.0;
pub const ASTEROID_HEIGHT: f32 = 70.0;
pub const ASTEROID_SPAWN_COOLDOWN: f32 = 5.0;

use resources::*;
use systems::*;

use crate::{ game::SimulationState, AppState };

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnAsteroidTimer>()
            .add_systems(
                (
                    tick_asteroid_spawn_timer,
                    spawn_asteroids_over_time,
                    asteroid_movement,
                    bullet_hit_asteroid,
                    asteroid_hit_player,
                    asteroid_cleanup,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
            )
            .add_system(
                despawn_asteroids.in_schedule(OnExit(AppState::Game))
            );
    }
}
