use bevy::prelude::*;

use crate::ui::hud::{
    components::{HUD, ScoreText, LivesText},
    styles::{get_text_style, HUD_STYLE},
};

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
}

pub fn build_hud(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let hud_entity = commands
        .spawn((
            NodeBundle {
                style: HUD_STYLE,
                ..default()
            },
            HUD {},
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Lives: 0", get_text_style(&asset_server))],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            }, LivesText {}));
        })
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Score: 0", get_text_style(&asset_server))],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            }, ScoreText {}));
        })
        .id();

    hud_entity
}

pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for hud_entity in hud_query.iter() {
        commands.entity(hud_entity).despawn_recursive();
    }
}
