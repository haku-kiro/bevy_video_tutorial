use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        // This was like simulat_state.0 == SimulationState::Running, that is however, a private
        // member
        let state = simulation_state.get();
        if *state == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation Paused.");
        }
    }
}
