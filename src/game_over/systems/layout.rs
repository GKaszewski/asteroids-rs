use bevy::prelude::*;

use crate::game_over::components::*;
use crate::game_over::styles::*;
use crate::score::resources::Score;

pub fn spawn_game_over_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	score: Res<Score>
) {
	let _game_over_entity = build_game_over_menu(&mut commands, &asset_server, score);
}

pub fn despawn_game_over_menu(
	mut commands: Commands,
	game_over_query: Query<Entity, With<GameOver>>
) {
	for game_over_entity in game_over_query.iter() {
		commands.entity(game_over_entity).despawn_recursive();
	}
}

pub fn build_game_over_menu(
	commands: &mut Commands,
	asset_server: &Res<AssetServer>,
	score: Res<Score>
) -> Entity {
	let game_over_entity = commands.spawn((NodeBundle {
		background_color: Color::DARK_GRAY.into(),
		style: Style {
			size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
			flex_direction: FlexDirection::Column,
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
			..default()
		},
		..default()
	}, GameOver {} ))
	.with_children(|parent| {
		// Title
		parent.spawn(
			NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Column,
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					size: Size::new(Val::Percent(70.0), Val::Px(130.0)),
        			..default()
				},
				..default()
			}
		).with_children(|parent| {
			parent.spawn(TextBundle {
				text: Text {
					sections: vec![
						TextSection::new(
							"You lost!",
							TextStyle {
								font: asset_server.load("fonts/font.ttf"),
								font_size: 80.0,
								color: Color::WHITE,
							},
						),
					],
					alignment: TextAlignment::Center,
					..default()
				},
				..default()
			});
			parent.spawn(TextBundle {
				text: Text {
					sections: vec![
						TextSection::new(
							format!("Score: {}", score.value),
							TextStyle {
								font: asset_server.load("fonts/font.ttf"),
								font_size: 40.0,
								color: Color::WHITE,
							},
						),
					],
					alignment: TextAlignment::Center,
					..default()
				},
				..default()
			});
		});
		// Back Button
		parent.spawn((
			ButtonBundle {
				style: BUTTON_STYLE,
				background_color: NORMAL_BUTTON_COLOR.into(),
				..default()
			},
			BackButton {}
		)).with_children(|parent| {
			parent.spawn(TextBundle {
				text: Text {
					sections: vec![
						TextSection::new(
							"Go to Main Menu",
							TextStyle {
								font: asset_server.load("fonts/font.ttf"),
								font_size: 40.0,
								color: Color::WHITE,
							},
						),
					],
					alignment: TextAlignment::Center,
					..default()
				},
				..default()
			});
		});
		// Quit Button
		parent.spawn((
			ButtonBundle {
				style: BUTTON_STYLE,
				background_color: NORMAL_BUTTON_COLOR.into(),
				..default()
			},
			QuitButton {}
		)).with_children(|parent| {
			parent.spawn(TextBundle {
				text: Text {
					sections: vec![
						TextSection::new(
							"Quit",
							TextStyle {
								font: asset_server.load("fonts/font.ttf"),
								font_size: 40.0,
								color: Color::WHITE,
							},
						),
					],
					alignment: TextAlignment::Center,
					..default()
				},
				..default()
			});
		});

	})
	.id();

	game_over_entity
}