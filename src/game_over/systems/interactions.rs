use bevy::{prelude::*, app::AppExit};

use crate::{game_over::{
    components::*,
    styles::{ PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR },
}, AppState, game::utils::handle_transition_to_state};

pub fn interact_with_back_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>)
    >,
	app_state: Res<State<AppState>>,
    next_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<crate::game::resources::GameState>,
    mut player_lives: ResMut<crate::player::resources::PlayerLives>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                game_state.is_game_over = false;
                player_lives.value = 3;
				handle_transition_to_state(app_state, next_state, AppState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>)
    >,
	mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
				app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
