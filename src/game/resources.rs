use bevy::prelude::*;
	

#[derive(Resource)]
pub struct GameState {
    pub is_game_over: bool,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            is_game_over: false,
        }
    }
}

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);