use bevy::prelude::*;
use systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                // You can use before/after to run commands in order
                player_movement.before(confine_player_movement),
                confine_player_movement,
                enemy_hit_player,
                player_hit_star,
            ),
        );
    }
}
