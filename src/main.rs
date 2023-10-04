#![windows_subsystem = "windows"]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub mod asteroid;
pub mod enemy;
pub mod player;
pub mod score;
pub mod bullet;
pub mod game;
pub mod main_menu;
pub mod game_over;
pub mod ui;
pub mod utils;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Asteroids".to_string(),
                ..default()
            }),
            ..default()
            }))
        .add_state::<AppState>()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(game_over::GameOverPlugin)
        .add_plugin(ui::UIPlugin)
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}


