// src/frontend.rs

use crate::game::{Game, EMPTY, OBSTACLE, PLAYER_O, PLAYER_X};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::shape::Circle;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use std::collections::HashSet;

// Define PowerUpType with Bomb, Skip, and Obstacle Power-Ups
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PowerUpType {
    Bomb,     // 'B'
    Skip,     // 'S'
    Obstacle, // 'H'
}

impl PowerUpType {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'B' => Some(Self::Bomb),
            'S' => Some(Self::Skip),
            'H' => Some(Self::Obstacle),
            _ => None,
        }
    }

    fn color(&self) -> Color {
        match self {
            Self::Bomb => Color::ORANGE,        // Bombs are orange
            Self::Skip => Color::YELLOW,        // Skips are yellow
            Self::Obstacle => Color::DARK_GRAY, // Obstacle Power-Ups are dark gray
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Bomb => "B",     // Bomb label
            Self::Skip => "S",     // Skip label
            Self::Obstacle => "H", // Obstacle Power-Up label
        }
    }
}

// Define application states
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

// Define GameState resource
#[derive(Resource)]
struct GameState {
    game: Game,
    previous_rows: usize,
    previous_cols: usize,
}

impl Default for GameState {
    fn default() -> Self {
        let game = Game::new();
        Self {
            previous_rows: game.get_board().len(),
            previous_cols: game.get_board()[0].len(),
            game,
        }
    }
}

// Define UI components
#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct MainMenuButton;

#[derive(Component)]
struct TurnIndicator;

#[derive(Component)]
struct Cell {
    row: usize,
    col: usize,
    power_up: Option<PowerUpType>,
}

#[derive(Component)]
struct Piece {
    player: char,
    row: usize,
    col: usize,
}

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct GameUI;

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct AnimatePiece {
    target_y: f32,
}

#[derive(Component)]
struct GameBackground;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Flashing;

// New Component for Explosion Animation
#[derive(Component)]
struct Explosion {
    timer: Timer,
}

// New Component for Static Obstacles
#[derive(Component)]
struct StaticObstacle {
    row: usize,
    col: usize,
}

// Define PowerUpActivated event
struct PowerUpActivated {
    row: usize,
    col: usize,
    power_up: PowerUpType,
}

// Implement the Event trait for PowerUpActivated
impl Event for PowerUpActivated {}

// Setup system to spawn the camera
fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// Setup main menu UI
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.3).into(),
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Rusty Connect Four",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
            );

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Use number keys 1-0 to drop pieces into columns.\nFirst to connect four in a row wins!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
            );

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(50.0),
                    ..default()
                },
                ..default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.65, 0.15).into(),
                        ..default()
                    },
                    StartButton,
                ))
                .with_children(|button| {
                    button.spawn(
                        TextBundle {
                            text: Text::from_section(
                                "Start",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_alignment(TextAlignment::Center),
                            ..default()
                        },
                    );
                });
        });
}

// System to handle interactions with the Start button in the main menu
fn main_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::rgb(0.10, 0.55, 0.10).into();
                app_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background_color = Color::rgb(0.25, 0.75, 0.25).into();
            }
            Interaction::None => {
                *background_color = Color::rgb(0.15, 0.65, 0.15).into();
            }
        }
    }
}

// Cleanup system for the main menu
fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// Setup the game UI
fn setup_game(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    state.game = Game::new();
    state.previous_rows = state.game.get_board().len();
    state.previous_cols = state.game.get_board()[0].len();

    // Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.05, 0.05, 0.2, 0.5), // Semi-transparent for aesthetics
                custom_size: Some(Vec2::new(800.0, 600.0)), // Adjusted size
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
        GameBackground,
    ));

    // Render the game board
    render_game_board(&mut commands, &state, &asset_server);

    // Adjust the camera based on the board size
    let (board_width, board_height) = get_board_dimensions(&state);
    adjust_camera(&mut camera_query, board_width, board_height);
}

// Define a color palette for column numbers (excluding blue and yellow)
const COLUMN_COLORS: [Color; 10] = [
    Color::RED,        // Column 1
    Color::GREEN,      // Column 2
    Color::PURPLE,     // Column 3
    Color::ORANGE,     // Column 4
    Color::PINK,       // Column 5
    Color::TEAL,       // Column 6
    Color::CYAN,       // Column 7
    Color::LIME_GREEN, // Column 8
    Color::INDIGO,     // Column 9
    Color::VIOLET,     // Column 10
];

// Render the game board with cells, power-ups, and obstacles
fn render_game_board(commands: &mut Commands, state: &GameState, asset_server: &Res<AssetServer>) {
    let rows = state.game.get_board().len();
    let cols = state.game.get_board()[0].len();

    let cell_size = 75.0;
    let padding = 7.5;
    let board_width = cols as f32 * (cell_size + padding) - padding;
    let board_height = rows as f32 * (cell_size + padding) - padding;

    // Render the board background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.8, 0.9),
                custom_size: Some(Vec2::new(
                    board_width + padding * 2.0,
                    board_height + padding * 2.0,
                )),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -20.0, 0.0),
            ..default()
        },
        GameUI,
    ));

    // Render column labels with different colors
    for col in 0..cols {
        let label = if col < 9 {
            (col + 1).to_string()
        } else {
            "0".to_string()
        };

        // Assign color based on column index
        let color = if col < COLUMN_COLORS.len() {
            COLUMN_COLORS[col]
        } else {
            Color::GOLD // Default color if columns exceed predefined colors
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 45.0,
                        color, // Use the assigned color
                    },
                ),
                transform: Transform::from_xyz(
                    col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0,
                    board_height / 2.0 + 40.0,
                    1.0,
                ),
                ..default()
            },
            GameUI,
        ));
    }

    // Render turn indicator
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Px(-150.0),
                        ..default()
                    },
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            GameUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Player 1's Turn",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::RED, // Player 1 is Red
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
                TurnIndicator,
            ));
        });

    // Render each cell on the board
    for row in 0..rows {
        for col in 0..cols {
            let cell_char = state.game.get_board()[row][col];
            let power_up = PowerUpType::from_char(cell_char);
            let is_obstacle = cell_char == OBSTACLE;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.9, 0.9, 0.9, 0.1),
                        custom_size: Some(Vec2::new(cell_size, cell_size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0,
                        row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0
                            - 20.0,
                        1.0, // Ensure cells are below pieces, power-ups, and obstacles
                    ),
                    ..default()
                },
                Cell { row, col, power_up },
                GameUI,
            ));

            // Render power-ups if present
            if let Some(pu) = power_up {
                // Render a colored circle behind the label
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: pu.color(),
                            custom_size: Some(Vec2::new(cell_size / 2.0, cell_size / 2.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            col as f32 * (cell_size + padding) - board_width / 2.0
                                + cell_size / 2.0,
                            row as f32 * (cell_size + padding) - board_height / 2.0
                                + cell_size / 2.0
                                - 20.0,
                            1.5, // Render between cell and label
                        ),
                        ..default()
                    },
                    GameUI,
                ));

                // Render the power-up label
                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            pu.label(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        transform: Transform::from_xyz(
                            col as f32 * (cell_size + padding) - board_width / 2.0
                                + cell_size / 2.0,
                            row as f32 * (cell_size + padding) - board_height / 2.0
                                + cell_size / 2.0
                                - 20.0,
                            2.0, // Render above the cell
                        ),
                        ..default()
                    },
                    GameUI,
                ));
            }

            // Render static obstacles if present
            if is_obstacle {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLACK, // Obstacles are black
                            custom_size: Some(Vec2::new(cell_size, cell_size)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            col as f32 * (cell_size + padding) - board_width / 2.0
                                + cell_size / 2.0,
                            row as f32 * (cell_size + padding) - board_height / 2.0
                                + cell_size / 2.0
                                - 20.0,
                            1.6, // Render above cells and power-ups
                        ),
                        ..default()
                    },
                    StaticObstacle { row, col }, // Updated to include position
                    GameUI,
                ));
            }
        }
    }
}

// System to handle game updates based on player input
fn update_game(
    mut state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut turn_query: Query<&mut Text, With<TurnIndicator>>,
    asset_server: Res<AssetServer>,
    game_ui_query: Query<Entity, With<GameUI>>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    mut power_up_activated_events: EventWriter<PowerUpActivated>,
) {
    let cols = state.game.get_board()[0].len();

    for col in 0..cols {
        let key = match col {
            0 => KeyCode::Key1,
            1 => KeyCode::Key2,
            2 => KeyCode::Key3,
            3 => KeyCode::Key4,
            4 => KeyCode::Key5,
            5 => KeyCode::Key6,
            6 => KeyCode::Key7,
            7 => KeyCode::Key8,
            8 => KeyCode::Key9,
            9 => KeyCode::Key0,
            _ => continue,
        };

        if keyboard_input.just_pressed(key) {
            if let Ok((row, col)) = state.game.drop_piece(col) {
                spawn_piece(
                    &mut commands,
                    &state.game,
                    row,
                    col,
                    &mut meshes,
                    &mut materials,
                );

                // Check if the dropped piece was on a power-up
                let cell_char = state.game.get_board()[row][col];
                if let Some(pu) = PowerUpType::from_char(cell_char) {
                    power_up_activated_events.send(PowerUpActivated {
                        row,
                        col,
                        power_up: pu,
                    });
                }

                // Check for a winner
                if let Some(_winner) = state.game.check_winner() {
                    app_state.set(AppState::GameOver);
                    return;
                }

                // Check if the board is full
                if state.game.is_full() {
                    unsafe {
                        if !crate::game::IF_EXPAND {
                            crate::game::IF_EXPAND = true;
                            state.game.expand_board();

                            state.previous_rows = state.game.get_board().len();
                            state.previous_cols = state.game.get_board()[0].len();

                            cleanup_game_board(&mut commands, &game_ui_query);
                            render_game_board(&mut commands, &state, &asset_server);

                            let (board_width, board_height) = get_board_dimensions(&state);
                            adjust_camera(&mut camera_query, board_width, board_height);

                            for row in 0..state.game.get_board().len() {
                                for col in 0..state.game.get_board()[0].len() {
                                    let player = state.game.get_board()[row][col];
                                    if player != EMPTY && player != OBSTACLE {
                                        spawn_existing_piece(
                                            &mut commands,
                                            &state.game,
                                            row,
                                            col,
                                            &mut meshes,
                                            &mut materials,
                                        );
                                    }
                                }
                            }

                            return;
                        }
                    }

                    app_state.set(AppState::GameOver);
                    return;
                }

                // Switch player
                state.game.switch_player();

                // Update turn indicator
                for mut text in &mut turn_query {
                    let player_number = if state.game.get_current_player() == PLAYER_X {
                        "1"
                    } else {
                        "2"
                    };
                    text.sections[0].value = format!("Player {}'s Turn", player_number);
                    text.sections[0].style.color = if state.game.get_current_player() == PLAYER_X {
                        Color::RED // Player 1 is Red
                    } else {
                        Color::YELLOW // Player 2 is Yellow
                    };
                }
            } else {
                println!("Column is full.");
            }

            break;
        }
    }
}

// Function to spawn a new piece with animation
fn spawn_piece(
    commands: &mut Commands,
    game: &Game,
    row: usize,
    col: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let player = game.get_board()[row][col];
    if player == PLAYER_X || player == PLAYER_O {
        // Ensure only player pieces are spawned
        let (color, z) = if player == PLAYER_X {
            (Color::RED, 2.0) // Player 1 is Red
        } else {
            (Color::YELLOW, 2.0) // Player 2 is Yellow
        };

        let cell_size = 75.0;
        let padding = 7.5;
        let cols = game.get_board()[0].len();
        let rows = game.get_board().len();
        let board_width = cols as f32 * (cell_size + padding) - padding;
        let board_height = rows as f32 * (cell_size + padding) - padding;

        let circle_mesh = meshes.add(Mesh::from(Circle::new(cell_size / 2.0 - 5.0)));

        let material_handle = materials.add(ColorMaterial::from(color));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle_mesh.into(),
                material: material_handle,
                transform: Transform::from_xyz(
                    col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0,
                    board_height / 2.0 + cell_size,
                    z,
                ),
                ..default()
            },
            Piece { player, row, col }, // Assign row and col
            AnimatePiece {
                target_y: row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0
                    - 20.0,
            },
            GameUI,
        ));
    }
}

// Function to spawn existing pieces when expanding the board
fn spawn_existing_piece(
    commands: &mut Commands,
    game: &Game,
    row: usize,
    col: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let player = game.get_board()[row][col];
    if player == PLAYER_X || player == PLAYER_O {
        // Ensure only player pieces are spawned
        let (color, z) = if player == PLAYER_X {
            (Color::RED, 2.0) // Player 1 is Red
        } else {
            (Color::YELLOW, 2.0) // Player 2 is Yellow
        };

        let cell_size = 75.0;
        let padding = 7.5;
        let cols = game.get_board()[0].len();
        let rows = game.get_board().len();
        let board_width = cols as f32 * (cell_size + padding) - padding;
        let board_height = rows as f32 * (cell_size + padding) - padding;

        let circle_mesh = meshes.add(Mesh::from(Circle::new(cell_size / 2.0 - 5.0)));

        let material_handle = materials.add(ColorMaterial::from(color));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle_mesh.into(),
                material: material_handle,
                transform: Transform::from_xyz(
                    col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0,
                    row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0
                        - 20.0,
                    z,
                ),
                ..default()
            },
            Piece { player, row, col }, // Assign row and col
            GameUI,
        ));
    }
}

// System to animate pieces dropping into place
fn animate_pieces(
    mut query: Query<(Entity, &mut Transform, &AnimatePiece)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, animate) in &mut query {
        let speed = 800.0;
        transform.translation.y -= speed * time.delta_seconds();

        if transform.translation.y <= animate.target_y {
            transform.translation.y = animate.target_y;
            commands.entity(entity).remove::<AnimatePiece>();
        }
    }
}

// Cleanup system for the game board
fn cleanup_game_board(commands: &mut Commands, query: &Query<Entity, With<GameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Cleanup system for the entire game
fn cleanup_game(
    mut commands: Commands,
    query: Query<Entity, With<GameUI>>,
    background_query: Query<Entity, With<GameBackground>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &background_query {
        commands.entity(entity).despawn_recursive();
    }
}

// Function to render the final board in the game over screen
fn render_final_board(parent: &mut ChildBuilder, game_state: &GameState) {
    let cell_size = 30.0;
    let margin = 1.0;
    let rows = game_state.game.get_board().len();
    let cols = game_state.game.get_board()[0].len();

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    width: Val::Px((cols as f32) * (cell_size + margin * 2.0)),
                    height: Val::Px((rows as f32) * (cell_size + margin * 2.0)),
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.3).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|board_parent| {
            for row in 0..rows {
                board_parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row_parent| {
                        for col in 0..cols {
                            let cell_background = Color::rgba(0.9, 0.9, 0.9, 0.8);

                            let player = game_state.game.get_board()[row][col];
                            let piece_color = if player == PLAYER_X {
                                Some(Color::RED) // Player 1 is Red
                            } else if player == PLAYER_O {
                                Some(Color::YELLOW) // Player 2 is Yellow
                            } else if player == OBSTACLE {
                                Some(Color::BLACK) // Obstacles are Black
                            } else {
                                None
                            };

                            row_parent
                                .spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(cell_size),
                                            height: Val::Px(cell_size),
                                            margin: UiRect::all(Val::Px(margin)),
                                            ..default()
                                        },
                                        background_color: cell_background.into(),
                                        ..default()
                                    },
                                    GameOverUI,
                                ))
                                .with_children(|cell_parent| {
                                    if let Some(color) = piece_color {
                                        cell_parent.spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(100.0),
                                                    height: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                background_color: color.into(),
                                                ..default()
                                            },
                                            GameOverUI,
                                        ));
                                    }
                                });
                        }
                    });
            }
        });
}

// Setup the game over screen
fn setup_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    let message = if let Some(winner) = game_state.game.check_winner() {
        let player_number = if winner == PLAYER_X { "1" } else { "2" };
        format!("Player {} Wins!", player_number)
    } else {
        "It's a Tie!".to_string()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.3).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 70.0,
                        color: Color::GOLD,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text::from_section(
                    message,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(40.0),
                    ..default()
                },
                ..default()
            });

            render_final_board(parent, &game_state);

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(40.0),
                    ..default()
                },
                ..default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(220.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.15, 0.65, 0.15).into(),
                        ..default()
                    },
                    MainMenuButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Main Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        ..default()
                    });
                });
        });
}

// System to handle interactions with the Main Menu button in the game over screen
fn game_over_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::rgb(0.10, 0.55, 0.10).into();
                app_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *background_color = Color::rgb(0.25, 0.75, 0.25).into();
            }
            Interaction::None => {
                *background_color = Color::rgb(0.15, 0.65, 0.15).into();
            }
        }
    }
}

// Cleanup system for the game over screen
fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// Function to get board dimensions based on the current game state
fn get_board_dimensions(state: &GameState) -> (f32, f32) {
    let rows = state.game.get_board().len();
    let cols = state.game.get_board()[0].len();

    let cell_size = 75.0;
    let padding = 7.5;
    let board_width = cols as f32 * (cell_size + padding) - padding;
    let board_height = rows as f32 * (cell_size + padding) - padding;

    (board_width, board_height)
}

// Function to adjust the camera based on the board size
fn adjust_camera(
    camera_query: &mut Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    board_width: f32,
    board_height: f32,
) {
    for (mut ortho, mut transform) in camera_query.iter_mut() {
        let desired_width = board_width + 200.0;
        let desired_height = board_height + 200.0;

        // Use FixedVertical scaling mode with desired_height
        ortho.scaling_mode = ScalingMode::FixedVertical(desired_height);

        ortho.area = Rect {
            min: Vec2::new(-desired_width / 2.0, -desired_height / 2.0),
            max: Vec2::new(desired_width / 2.0, desired_height / 2.0),
        };

        transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);
    }
}

// System to handle power-up activation events
fn handle_power_up_activation(
    mut events: EventReader<PowerUpActivated>,
    mut commands: Commands,
    query: Query<(Entity, &Cell), With<Cell>>,
    piece_query: Query<(Entity, &Piece), With<Piece>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    state: Res<GameState>, // Added to get board dimensions
) {
    for event in events.iter() {
        for (entity, cell) in query.iter() {
            if cell.row == event.row && cell.col == event.col {
                match event.power_up {
                    PowerUpType::Bomb => {
                        // Spawn explosion animation at the bomb's location
                        let (board_width, board_height) = get_board_dimensions(&state);
                        let cell_size = 75.0;
                        let padding = 7.5;

                        commands.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 0.5, 0.0, 1.0),
                                    custom_size: Some(Vec2::new(50.0, 50.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    cell.col as f32 * (cell_size + padding) - board_width / 2.0
                                        + cell_size / 2.0,
                                    cell.row as f32 * (cell_size + padding) - board_height / 2.0
                                        + cell_size / 2.0
                                        - 20.0,
                                    3.0, // Ensure explosion is on top
                                ),
                                ..default()
                            },
                            Explosion {
                                timer: Timer::from_seconds(0.5, TimerMode::Once),
                            },
                            GameUI,
                        ));

                        // Determine the position of the piece to remove (e.g., below the bomb)
                        if cell.row > 0 {
                            let target_row = cell.row - 1;
                            let target_col = cell.col;

                            // Find the piece entity at (target_row, target_col)
                            for (piece_entity, piece) in piece_query.iter() {
                                if piece.row == target_row && piece.col == target_col {
                                    // Despawn the piece
                                    commands.entity(piece_entity).despawn();
                                    break;
                                }
                            }
                        }
                    }
                    PowerUpType::Skip => {
                        // Handle skip turn animation or indication if needed
                        // Currently handled by flashing effect
                        commands.entity(entity).insert(Flashing);
                    }
                    PowerUpType::Obstacle => {
                        // Handle obstacle placement
                        let (board_width, board_height) = get_board_dimensions(&state);
                        let cell_size = 75.0;
                        let padding = 7.5;

                        commands.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::BLACK, // Obstacles are black
                                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    cell.col as f32 * (cell_size + padding) - board_width / 2.0
                                        + cell_size / 2.0,
                                    cell.row as f32 * (cell_size + padding) - board_height / 2.0
                                        + cell_size / 2.0
                                        - 20.0,
                                    1.6, // Render above cells and power-ups
                                ),
                                ..default()
                            },
                            StaticObstacle {
                                row: cell.row,
                                col: cell.col,
                            }, // Include position
                            GameUI,
                        ));
                    }
                }
            }
        }
    }
}

// System to handle explosion animations
fn explosion_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion, &mut Sprite)>,
) {
    for (entity, mut explosion, mut sprite) in query.iter_mut() {
        explosion.timer.tick(time.delta());

        // Simple scaling effect for explosion
        let scale_factor = 1.0 + (0.5 * explosion.timer.elapsed_secs());
        sprite.custom_size = Some(Vec2::splat(50.0 * scale_factor));

        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// System to handle flashing effect for activated power-ups
fn flash_power_up(
    mut query: Query<(Entity, &mut Sprite), With<Flashing>>,
    time: Res<Time>,
    mut timer: Local<f32>,
    mut commands: Commands,
) {
    *timer += time.delta_seconds();
    if *timer > 0.5 {
        for (entity, mut sprite) in query.iter_mut() {
            sprite.color = if sprite.color.a() == 1.0 {
                Color::WHITE
            } else {
                Color::ORANGE // Or any color indicating activation
            };
            // Remove Flashing after flashing
            commands.entity(entity).remove::<Flashing>();
        }
        *timer = 0.0;
    }
}

// System to handle synchronization of frontend with backend game state
fn synchronize_frontend(
    state: Res<GameState>,
    mut commands: Commands,
    existing_pieces: Query<(Entity, &Piece), With<Piece>>,
    existing_obstacles: Query<(Entity, &StaticObstacle), With<StaticObstacle>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let board = state.game.get_board();

    // Synchronize Player Pieces
    let mut required_piece_positions = HashSet::new();
    for (row, row_cells) in board.iter().enumerate() {
        for (col, &cell) in row_cells.iter().enumerate() {
            if cell == PLAYER_X || cell == PLAYER_O {
                required_piece_positions.insert((row, col));
            }
        }
    }

    // Despawn pieces not present on the board
    for (entity, piece) in existing_pieces.iter() {
        if !required_piece_positions.contains(&(piece.row, piece.col)) {
            commands.entity(entity).despawn();
        } else {
            // Remove the position as it's already rendered
            required_piece_positions.remove(&(piece.row, piece.col));
        }
    }

    // Spawn missing player pieces
    for &(row, col) in &required_piece_positions {
        let cell_char = board[row][col];
        spawn_piece(
            &mut commands,
            &state.game,
            row,
            col,
            &mut meshes,
            &mut materials,
        );
    }

    // Synchronize Obstacles
    let mut required_obstacle_positions = HashSet::new();
    for (row, row_cells) in board.iter().enumerate() {
        for (col, &cell) in row_cells.iter().enumerate() {
            if cell == OBSTACLE {
                required_obstacle_positions.insert((row, col));
            }
        }
    }

    // Despawn obstacles not present on the board
    for (entity, obstacle) in existing_obstacles.iter() {
        if !required_obstacle_positions.contains(&(obstacle.row, obstacle.col)) {
            commands.entity(entity).despawn();
        } else {
            // Remove the position as it's already rendered
            required_obstacle_positions.remove(&(obstacle.row, obstacle.col));
        }
    }

    // Spawn missing obstacles
    for &(row, col) in &required_obstacle_positions {
        // Spawn obstacle
        let cell_size = 75.0;
        let padding = 7.5;
        let board_dimensions = get_board_dimensions(&state);
        let board_width = board_dimensions.0;
        let board_height = board_dimensions.1;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK, // Obstacles are black
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0,
                    row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0
                        - 20.0,
                    1.6, // Render above cells and power-ups
                ),
                ..default()
            },
            StaticObstacle { row, col }, // Include position
            GameUI,
        ));
    }
}

// Main function to set up the Bevy app
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Connect Four".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.2)))
        .insert_resource(GameState::default())
        .add_event::<PowerUpActivated>() // Register the event
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(
            Update,
            main_menu_button_system.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(
            Update,
            (
                update_game,
                animate_pieces,
                handle_power_up_activation, // Updated handler with removal logic
                explosion_animation,        // Explosion animation system
                flash_power_up,             // Existing flashing system
                synchronize_frontend,       // Synchronization system
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), cleanup_game)
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(
            Update,
            game_over_button_system.run_if(in_state(AppState::GameOver)),
        )
        .add_systems(OnExit(AppState::GameOver), cleanup_game_over)
        .run();
}
