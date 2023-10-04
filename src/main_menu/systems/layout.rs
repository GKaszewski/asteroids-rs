use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu (
	mut commands: Commands,
	asset_server: Res<AssetServer>
) {
	let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu (
	mut commands: Commands,
	main_menu_query: Query<Entity, With<MainMenu>>
) {
	for main_menu_entity in main_menu_query.iter() {
		commands.entity(main_menu_entity).despawn_recursive();
	}
}

pub fn build_main_menu (
	commands: &mut Commands,
	asset_server: &Res<AssetServer>
) -> Entity {
	let main_menu_entity = commands.spawn((NodeBundle {
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
	}, MainMenu {} ))
	.with_children(|parent| {
		// Title
		parent.spawn(
			NodeBundle {
				style: Style {
					flex_direction: FlexDirection::Row,
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					size: Size::new(Val::Percent(70.0), Val::Px(80.0)),
        			..default()
				},
				..default()
			}
		).with_children(|parent| {
			parent.spawn(TextBundle {
				text: Text {
					sections: vec![
						TextSection::new(
							"Asteroids",
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
		});
		// Play Button
		parent.spawn((
			ButtonBundle {
				style: BUTTON_STYLE,
				background_color: NORMAL_BUTTON_COLOR.into(),
				..default()
			},
			PlayButton {}
		)).with_children(|parent| {
			parent.spawn(TextBundle {
				text: Text {
					sections: vec![
						TextSection::new(
							"Play",
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

	main_menu_entity
}