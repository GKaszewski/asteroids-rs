use bevy::prelude::*;

#[derive(Component)]
pub struct Asteroid {
    pub direction: Vec2,
    pub phase: f32,
}
