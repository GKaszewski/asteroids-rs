use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod utils;
mod systems;

use systems::*;
use resources::*;

use crate::{ AppState, game::SimulationState };

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_WIDTH: f32 = 98.0;
pub const PLAYER_HEIGHT: f32 = 75.0;
pub const PLAYER_ATTACK_COOLDOWN: f32 = 0.5;
pub const PLAYER_RESPAWN_COOLDOWN: f32 = 1.5;
pub const PLAYER_INVULNERABLE_COOLDOWN: f32 = 6.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLives>()
            .init_resource::<PlayerAttackTimer>()
            .init_resource::<PlayerRespawnTimer>()
            .init_resource::<PlayerInvulnerableTimer>()
            .add_system(
                setup_player.in_schedule(OnEnter(AppState::Game))
            )
            .add_systems(
                (
                    player_movement,
                    confine_player_movement,
                    player_attack,
                    respawn_player,
                    player_respawn_timer_tick,
                    player_invulnerable_timer_tick,
                    player_invulnerable,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
            )
            .add_system(
                despawn_player.in_schedule(OnExit(AppState::Game))
            );
    }
}
