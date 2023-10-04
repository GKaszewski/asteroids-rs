use bevy::prelude::*;

mod hud;
mod pause;

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_plugin(hud::HudPlugin);
			//.add_plugin(pause::PausePlugin);
	}
}