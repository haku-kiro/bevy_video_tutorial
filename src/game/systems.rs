use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Running => next_state.set(SimulationState::Paused),
            SimulationState::Paused => next_state.set(SimulationState::Running),
        }
    }
}
