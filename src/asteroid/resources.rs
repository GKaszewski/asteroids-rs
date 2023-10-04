use bevy::prelude::*;

use super::ASTEROID_SPAWN_COOLDOWN;

#[derive(Resource)]
pub struct SpawnAsteroidTimer {
    pub timer: Timer,
}

impl Default for SpawnAsteroidTimer {
    fn default() -> SpawnAsteroidTimer {
        SpawnAsteroidTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWN_COOLDOWN, TimerMode::Repeating),
        }
    }
}