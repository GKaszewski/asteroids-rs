use bevy::{
    prelude::*,
    input::gamepad::{ GamepadConnectionEvent, GamepadConnection }, window::PrimaryWindow,
};
use bevy::core_pipeline::clear_color::ClearColorConfig;

use crate::{player::{PLAYER_SPEED, components::Player, utils::handle_player_attack}, AppState};

use super::{resources::MyGamepad, SimulationState, utils::{handle_toggle_simulation, handle_transition_to_state}};

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_connection_events: EventReader<GamepadConnectionEvent>
) {
    for connection_event in gamepad_connection_events.iter() {
        info!("{:?}", connection_event);
        match connection_event.connection {
            GamepadConnection::Connected(_) => {
                println!("Gamepad connected");
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(connection_event.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                println!("Gamepad disconnected");
                commands.remove_resource::<MyGamepad>();
            }
        }
    }
}

pub fn gamepad_input(
    mut commands: Commands,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    simulation_state: Res<State<SimulationState>>
) {
    let gamepad = if let Some(gp) = my_gamepad {
        gp.0
    } else {
        return;
    };

    if let Ok(mut transform) = player_query.get_single_mut() {
        let left_stick_x = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        let left_stick_y = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickY,
        };

        if let (Some(x), Some(y)) = (axes.get(left_stick_x), axes.get(left_stick_y)) {
            let mut direction = Vec3::ZERO;
            let left_stick_pos = Vec2::new(x, y);

            if left_stick_pos.length() > 0.9 && left_stick_pos.y > 0.5 {
                direction += Vec3::new(0.0, 1.0, 0.0);
            }

            if left_stick_pos.length() > 0.9 && left_stick_pos.y < -0.5 {
                direction += Vec3::new(0.0, -1.0, 0.0);
            }

            if left_stick_pos.length() > 0.9 && left_stick_pos.x < -0.5 {
                direction += Vec3::new(-1.0, 0.0, 0.0);
            }

            if left_stick_pos.length() > 0.9 && left_stick_pos.x > 0.5 {
                direction += Vec3::new(1.0, 0.0, 0.0);
            }

            direction = direction.normalize_or_zero();
            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }

        let a_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };

        let start_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        };

        if buttons.just_pressed(a_button) {
            handle_player_attack(&mut commands, asset_server, &transform, audio);
        }

        if buttons.just_pressed(start_button) {
            handle_toggle_simulation(&mut commands, simulation_state);
        }
    }
}

pub fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.0, 0.0, 0.0)),
            ..default()
        },
        ..default()
    });
}

pub fn toggle_simulation (
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        handle_toggle_simulation(&mut commands, simulation_state)
    }
}

pub fn transition_to_game_state (
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        handle_transition_to_state(app_state, next_state, AppState::Game);
    }
}

pub fn transition_to_menu_state (
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    next_state: ResMut<NextState<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
       handle_transition_to_state(app_state, next_state, AppState::MainMenu);
    }
}