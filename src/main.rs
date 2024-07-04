// This will be bevy_ball_game
// Assets are taken from kenney.nl
use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // We know there is only one primary window, so we're using get_single here
    let window = window_query.get_single().unwrap();

    commands.spawn((
        // A bundle is a predefined set of components
        SpriteBundle {
            // Centering on window
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            // Pulls directly from the assets folder by default (in root dir)
            texture: asset_server.load("sprites/ball_blue_large.png"),
            // Rest of fields will be left at default values
            // This is a std rust trait, but a lot of bevy things implement it (nice)
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
    });
}
