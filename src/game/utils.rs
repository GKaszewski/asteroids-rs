use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub fn handle_toggle_simulation(
    commands: &mut Commands,
    simulation_state: Res<State<SimulationState>>
) {
    if simulation_state.0 == SimulationState::Running {
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
    } else {
        commands.insert_resource(NextState(Some(SimulationState::Running)));
    }
}

pub fn handle_transition_to_state(
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: AppState
) {
    if app_state.0 != state {
        next_state.set(state);
    }
}
