use bevy::app::AppExit;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::SimulationState;
use crate::{events::*, AppState};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn transition_to_game_state(
    app_state: Res<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            next_state.set(AppState::Game);
            println!("In game state");
        }
    }
}

pub fn transition_to_main_menu_state(
    app_state: Res<State<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyM) {
        if *app_state.get() != AppState::MainMenu {
            // Pause the game when you go to the menu
            next_simulation.set(SimulationState::Paused);
            next_app_state.set(AppState::MainMenu);
            println!("In menu state");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
    }
}
