use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec2,
    pub is_enemy: bool,
}