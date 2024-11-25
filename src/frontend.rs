// frontend.rs

use crate::game::{Game, COLS, EMPTY, PLAYER_X, ROWS};
use bevy::prelude::*;
use bevy::render::mesh::shape::Circle;
use bevy::render::mesh::Mesh;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Resource)]
struct GameState {
    game: Game,
}

impl Default for GameState {
    fn default() -> Self {
        Self { game: Game::new() }
    }
}

// Components to identify UI elements
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
}

#[derive(Component)]
struct Piece {
    player: char,
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

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
                background_color: Color::rgb(0.2, 0.2, 0.5).into(), // Dark blue background
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title Text
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Connect Four",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
            );

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            // Instructions Text
            parent.spawn(
                TextBundle {
                    text: Text::from_section(
                        "Use number keys 1-7 to drop pieces into columns.\nFirst to connect four in a row wins!",
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

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(50.0),
                    ..default()
                },
                ..default()
            });

            // Start Button
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

fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_game(
    mut commands: Commands,
    mut state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
) {
    state.game = Game::new();

    // Board Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.5),           // Dark blue background
                custom_size: Some(Vec2::new(600.0, 525.0)), // Increased size
                ..default()
            },
            transform: Transform::from_xyz(0.0, -37.5, -0.5),
            ..default()
        },
        GameUI,
    ));

    // Column Number Labels
    for col in 0..COLS {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    (col + 1).to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 45.0, // Increased font size
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(
                    col as f32 * 82.5 - 247.5,
                    ROWS as f32 * 82.5 - 187.5 + 75.0, // Adjusted position
                    0.0,
                ),
                ..default()
            },
            GameUI,
        ));
    }

    // Turn Indicator
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Px(-150.0), // Center the text
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
                            font_size: 50.0, // Increased font size
                            color: Color::BLUE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
                TurnIndicator,
            ));
        });

    // Create the game grid (ROWS x COLS)
    for row in 0..ROWS {
        for col in 0..COLS {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.9, 0.9, 0.9, 0.8),
                        custom_size: Some(Vec2::new(75.0, 75.0)), // Increased cell size
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        col as f32 * 82.5 - 247.5,
                        row as f32 * 82.5 - 187.5,
                        0.0,
                    ),
                    ..default()
                },
                Cell { row, col },
                GameUI,
            ));
        }
    }
}

fn update_game(
    mut state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut turn_query: Query<&mut Text, With<TurnIndicator>>,
) {
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

        // Drop the piece
        if let Ok((row, col)) = state.game.drop_piece(col) {
            spawn_piece(
                &mut commands,
                &state.game,
                row,
                col,
                &mut meshes,
                &mut materials,
            );

            // Check for winner or draw
            if let Some(_winner) = state.game.check_winner() {
                app_state.set(AppState::GameOver);
                return;
            }

            if state.game.is_full() {
                app_state.set(AppState::GameOver);
                return;
            }

            state.game.switch_player();

            // Update the turn indicator
            for mut text in &mut turn_query {
                let player_number = if state.game.get_current_player() == PLAYER_X {
                    "1"
                } else {
                    "2"
                };
                text.sections[0].value = format!("Player {}'s Turn", player_number);
                text.sections[0].style.color = if state.game.get_current_player() == PLAYER_X {
                    Color::BLUE
                } else {
                    Color::RED
                };
            }
        } else {
            println!("Column is full.");
            return;
        }
    }
}

fn spawn_piece(
    commands: &mut Commands,
    game: &Game,
    row: usize,
    col: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let player = game.get_board()[row][col];
    if player != EMPTY {
        let (color, z) = if player == PLAYER_X {
            (Color::BLUE, 1.0) // Blue for Player 1
        } else {
            (Color::RED, 1.0) // Red for Player 2
        };

        // Create a circle mesh
        let circle_mesh = meshes.add(Mesh::from(Circle::new(37.5))); // Radius adjusted for larger cells

        // Create a ColorMaterial and get a handle
        let material_handle = materials.add(ColorMaterial::from(color));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle_mesh.into(),
                material: material_handle,
                transform: Transform::from_xyz(
                    col as f32 * 82.5 - 247.5,
                    ROWS as f32 * 82.5 - 187.5 + 150.0, // Start higher due to larger board
                    z,
                ),
                ..default()
            },
            Piece { player },
            AnimatePiece {
                target_y: row as f32 * 82.5 - 187.5,
            },
            GameUI,
        ));
    }
}

fn animate_pieces(
    mut query: Query<(Entity, &mut Transform, &AnimatePiece)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, animate) in &mut query {
        let speed = 800.0; // Increased speed for longer distance
        transform.translation.y -= speed * time.delta_seconds();

        if transform.translation.y <= animate.target_y {
            transform.translation.y = animate.target_y;
            commands.entity(entity).remove::<AnimatePiece>();
        }
    }
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<GameUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn render_final_board(parent: &mut ChildBuilder, game_state: &GameState) {
    let cell_size = 30.0;
    let margin = 1.0;

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    width: Val::Px((COLS as f32) * (cell_size + margin * 2.0)),
                    height: Val::Px((ROWS as f32) * (cell_size + margin * 2.0)),
                    ..default()
                },
                background_color: Color::rgb(0.1, 0.1, 0.3).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|board_parent| {
            for row in 0..ROWS {
                board_parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row_parent| {
                        for col in 0..COLS {
                            let cell_background = Color::rgba(0.9, 0.9, 0.9, 0.8);

                            let player = game_state.game.get_board()[row][col];
                            let piece_color = if player == PLAYER_X {
                                Some(Color::BLUE)
                            } else if player != EMPTY {
                                Some(Color::RED)
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
                background_color: Color::rgb(0.2, 0.2, 0.5).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            // Game Over Text
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Game Over",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 70.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    ..default()
                },
                ..default()
            });

            // Winner Text
            parent.spawn(TextBundle {
                text: Text::from_section(
                    message,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::GOLD,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(40.0),
                    ..default()
                },
                ..default()
            });

            // Render the Final Board
            render_final_board(parent, &game_state);

            // Spacer
            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(40.0),
                    ..default()
                },
                ..default()
            });

            // Main Menu Button
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

fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState::default())
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
            (update_game, animate_pieces).run_if(in_state(AppState::InGame)),
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
