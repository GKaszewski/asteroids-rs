use bevy::prelude::*;

use crate::AppState;

use self::systems::{layout::{spawn_hud, despawn_hud}, updates::{update_score_text, update_lives_text}};

mod components;
mod styles;
mod systems;

pub struct HudPlugin;

impl Plugin for HudPlugin {
	fn build(&self, app: &mut App) {
		app
    .add_system(spawn_hud.in_schedule(OnEnter(AppState::Game)))
    .add_systems((update_score_text, update_lives_text).in_set(OnUpdate(AppState::Game)))
	.add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
	}
}