use crate::game::{Game, COLS, EMPTY, PLAYER_O, PLAYER_X, ROWS};
use bevy::prelude::*; // Import necessary Bevy components

#[derive(Resource)] // Mark GameState as a resource
struct GameState {
    game: Game,
}

impl Default for GameState {
    fn default() -> Self {
        Self { game: Game::new() }
    }
}

#[derive(Component)]
struct Cell {
    row: usize,
    col: usize,
}

#[derive(Component)]
struct Piece {
    player: char,
}

fn setup(mut commands: Commands) {
    // Correctly spawn the Camera2dBundle with spawn() by passing it as an argument
    commands.spawn(Camera2dBundle::default()); // This sets up the 2D camera for the scene

    // Create the game grid (ROWS x COLS)
    for row in 0..ROWS {
        for col in 0..COLS {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.1, 0.1, 0.1),         // Grid background color
                        custom_size: Some(Vec2::new(50.0, 50.0)), // Size of the cells
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        col as f32 * 55.0 - 175.0,
                        row as f32 * 55.0 - 150.0,
                        0.0,
                    ), // Position the cells
                    ..default()
                })
                .insert(Cell { row, col });
        }
    }
}

fn update_game(
    mut state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {
    // Use `just_pressed()` to ensure the key is only processed once per press
    if let Some(key) = keyboard_input.get_just_pressed().next() {
        let col = match key {
            KeyCode::Key1 => 0,
            KeyCode::Key2 => 1,
            KeyCode::Key3 => 2,
            KeyCode::Key4 => 3,
            KeyCode::Key5 => 4,
            KeyCode::Key6 => 5,
            KeyCode::Key7 => 6,
            _ => return,
        };

        // Drop the piece and switch players
        if let Err(err) = state.game.drop_piece(col) {
            println!("{}", err);
            return;
        }
        state.game.switch_player();

        // Re-render the board after the move
        render_pieces(&mut commands, &state.game);
    }

    // Check for winner or draw
    if let Some(winner) = state.game.check_winner() {
        println!("Player {} wins!", winner);
    }

    if state.game.is_full() {
        println!("It's a tie!");
    }
}

// Render the pieces (X or O) on the board
fn render_pieces(commands: &mut Commands, game: &Game) {
    // Add new pieces based on the game state
    let board = game.get_board();
    for row in 0..ROWS {
        for col in 0..COLS {
            let player = board[row][col];
            if player != EMPTY {
                let piece_color = if player == PLAYER_X {
                    Color::rgb(1.0, 0.0, 0.0) // Red for X
                } else {
                    Color::rgb(0.0, 0.0, 1.0) // Blue for O
                };

                commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: piece_color,
                            custom_size: Some(Vec2::new(50.0, 50.0)), // Size of the piece
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            col as f32 * 55.0 - 175.0,
                            row as f32 * 55.0 - 150.0,
                            1.0,
                        ), // Position the piece
                        ..default()
                    })
                    .insert(Piece { player });
            }
        }
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState::default()) // Correct method: use insert_resource() to add GameState
        .add_systems(Startup, setup) // Use add_systems for startup systems
        .add_systems(Update, update_game) // Use add_systems for regular systems
        .run();
}
