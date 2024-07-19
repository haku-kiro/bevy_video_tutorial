pub mod events;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use events::*;
use systems::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // You should import your events prior to your plugins incase they are required
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
