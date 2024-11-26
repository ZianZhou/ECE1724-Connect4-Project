// src/frontend.rs

use crate::game::{Game, EMPTY, PLAYER_X};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::shape::Circle;
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

#[derive(Component)]
struct GameBackground;

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
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
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    state.game = Game::new();
    state.previous_rows = state.game.get_board().len();
    state.previous_cols = state.game.get_board()[0].len();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.05, 0.05, 0.2, 0.5), // Semi-transparent for debugging
                custom_size: Some(Vec2::new(800.0, 600.0)), // Adjusted size
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..default()
        },
        GameBackground,
    ));

    render_game_board(&mut commands, &state, &asset_server);

    let (board_width, board_height) = get_board_dimensions(&state);
    adjust_camera(&mut camera_query, board_width, board_height);
}

fn render_game_board(commands: &mut Commands, state: &GameState, asset_server: &Res<AssetServer>) {
    let rows = state.game.get_board().len();
    let cols = state.game.get_board()[0].len();

    let cell_size = 75.0;
    let padding = 7.5;
    let board_width = cols as f32 * (cell_size + padding) - padding;
    let board_height = rows as f32 * (cell_size + padding) - padding;

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

    for col in 0..cols {
        let label = if col < 9 {
            (col + 1).to_string()
        } else {
            "0".to_string()
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 45.0,
                        color: Color::GOLD,
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
                            color: Color::BLUE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
                TurnIndicator,
            ));
        });

    for row in 0..rows {
        for col in 0..cols {
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
                        1.0,
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
    asset_server: Res<AssetServer>,
    game_ui_query: Query<Entity, With<GameUI>>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
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

                if let Some(_winner) = state.game.check_winner() {
                    app_state.set(AppState::GameOver);
                    return;
                }

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
                                    if player != EMPTY {
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
                        } else {
                            app_state.set(AppState::GameOver);
                            return;
                        }
                    }
                }

                state.game.switch_player();

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
            }

            break;
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
            (Color::BLUE, 2.0)
        } else {
            (Color::RED, 2.0)
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
            Piece { player },
            AnimatePiece {
                target_y: row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0
                    - 20.0,
            },
            GameUI,
        ));
    }
}

fn spawn_existing_piece(
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
            (Color::BLUE, 2.0)
        } else {
            (Color::RED, 2.0)
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
            Piece { player },
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
        let speed = 800.0;
        transform.translation.y -= speed * time.delta_seconds();

        if transform.translation.y <= animate.target_y {
            transform.translation.y = animate.target_y;
            commands.entity(entity).remove::<AnimatePiece>();
        }
    }
}

fn cleanup_game_board(commands: &mut Commands, query: &Query<Entity, With<GameUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

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

fn get_board_dimensions(state: &GameState) -> (f32, f32) {
    let rows = state.game.get_board().len();
    let cols = state.game.get_board()[0].len();

    let cell_size = 75.0;
    let padding = 7.5;
    let board_width = cols as f32 * (cell_size + padding) - padding;
    let board_height = rows as f32 * (cell_size + padding) - padding;

    (board_width, board_height)
}

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
