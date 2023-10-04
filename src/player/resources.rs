use bevy::prelude::*;

use super::{
	PLAYER_ATTACK_COOLDOWN,
	PLAYER_INVULNERABLE_COOLDOWN,
	PLAYER_RESPAWN_COOLDOWN,
};

#[derive(Resource)]
pub struct PlayerLives {
    pub value: i32,
}

impl Default for PlayerLives {
    fn default() -> PlayerLives {
        PlayerLives {
            value: 3,
        }
    }
}

#[derive(Resource)]
pub struct PlayerAttackTimer {
    pub timer: Timer,
}

impl Default for PlayerAttackTimer {
    fn default() -> PlayerAttackTimer {
        PlayerAttackTimer {
            timer: Timer::from_seconds(PLAYER_ATTACK_COOLDOWN, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct PlayerRespawnTimer {
    pub timer: Timer,
}

impl Default for PlayerRespawnTimer {
    fn default() -> PlayerRespawnTimer {
        PlayerRespawnTimer {
            timer: Timer::from_seconds(PLAYER_RESPAWN_COOLDOWN, TimerMode::Once),
        }
    }
}

#[derive(Resource)]
pub struct PlayerInvulnerableTimer {
    pub timer: Timer,
}

impl Default for PlayerInvulnerableTimer {
    fn default() -> PlayerInvulnerableTimer {
        PlayerInvulnerableTimer {
            timer: Timer::from_seconds(PLAYER_INVULNERABLE_COOLDOWN, TimerMode::Once),
        }
    }
}