use bevy::prelude::*;

use crate::{ui::hud::components::{ScoreText, LivesText}, score::resources::Score, player::resources::PlayerLives};

pub fn update_score_text (mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
	if score.is_changed() {
		for mut text in text_query.iter_mut() {
			text.sections[0].value = format!("Score: {}", score.value.to_string());
		}
	}
}

pub fn update_lives_text (mut text_query: Query<&mut Text, With<LivesText>>, lives: Res<PlayerLives>) {
	if lives.is_changed() {
		for mut text in text_query.iter_mut() {
			text.sections[0].value = format!("Lives: {}", lives.value.to_string());
		}
	}
}