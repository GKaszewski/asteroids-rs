use bevy::prelude::*;

use super::ENEMY_ATTACK_COOLDOWN;

#[derive(Resource)]
pub struct SpawnEnemyTimer {
    pub timer: Timer,
}

impl Default for SpawnEnemyTimer {
    fn default() -> SpawnEnemyTimer {
        SpawnEnemyTimer {
            timer: Timer::from_seconds(ENEMY_ATTACK_COOLDOWN, TimerMode::Repeating),
        }
    }
}


#[derive(Resource)]
pub struct AttackEnemyTimer {
    pub timer: Timer,
}

impl Default for AttackEnemyTimer {
    fn default() -> AttackEnemyTimer {
        AttackEnemyTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}