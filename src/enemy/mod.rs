use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod utils;
mod systems;

use systems::*;
use resources::*;

use crate::{ AppState, game::SimulationState };

pub const ENEMY_SPEED: f32 = 100.0;
pub const ENEMY_WIDTH: f32 = 91.0;
pub const ENEMY_HEIGHT: f32 = 91.0;
pub const ENEMY_ATTACK_COOLDOWN: f32 = 1.0;
pub const MAX_ENEMIES: u32 = 10;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnEnemyTimer>()
            .init_resource::<AttackEnemyTimer>()
            .add_system(
                spawn_enemy.in_schedule(OnEnter(AppState::Game)),
            )
            .add_system(
                despawn_enemies.in_schedule(OnExit(AppState::Game)),
            )
            .add_startup_system(
                spawn_enemy
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                (
                    enemy_movement,
                    enemy_attack_timer_tick,
                    enemy_attack,
                    enemy_cleanup,
                    enemy_hit_player,
                    enemy_bullet_hit_player,
                    bullet_hit_enemy,
                    spawn_enemies_over_time,
                    tick_enemy_spawn_timer,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
            );
    }
}
