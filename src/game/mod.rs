pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use bevy::prelude::*;

use crate::{AppState, GameOver};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            // Will only run if we're in the Game state
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            // Not sure about this?
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
