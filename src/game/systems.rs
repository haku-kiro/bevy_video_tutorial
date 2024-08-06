use bevy::prelude::*;

use crate::game::SimulationState;

pub fn pause_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_simulation_state: ResMut<NextState<SimulationState>>) {
    next_simulation_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Running => {
                next_state.set(SimulationState::Paused);
                println!("Paused");
            }
            SimulationState::Paused => {
                next_state.set(SimulationState::Running);
                println!("Playing");
            }
        }
    }
}
