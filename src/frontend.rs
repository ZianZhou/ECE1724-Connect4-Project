use crate::game::{Game, OBSTACLE, PLAYER_O, PLAYER_X};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::shape::Circle;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PowerUpType {
    Bomb,
    Skip,
    Obstacle,
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
            Self::Bomb => Color::PURPLE,
            Self::Skip => Color::GREEN,
            Self::Obstacle => Color::DARK_GRAY,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Bomb => "B",
            Self::Skip => "S",
            Self::Obstacle => "H",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Resource)]
struct GameStateResource {
    game: Game,
    previous_rows: usize,
    previous_cols: usize,
    power_ups_enabled: bool,
}

impl Default for GameStateResource {
    fn default() -> Self {
        let game = Game::new(false);
        Self {
            previous_rows: game.get_board().len(),
            previous_cols: game.get_board()[0].len(),
            game,
            power_ups_enabled: false,
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

#[derive(Component)]
struct Explosion {
    timer: Timer,
}

#[derive(Component)]
struct StaticObstacle {
    row: usize,
    col: usize,
}

#[derive(Component)]
struct PowerUpsToggleButton(bool);

#[derive(Component)]
struct PowerUpSymbol {
    row: usize,
    col: usize,
}

struct PowerUpActivated {
    row: usize,
    col: usize,
    power_up: PowerUpType,
}

impl Event for PowerUpActivated {}

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
            parent.spawn(TextBundle {
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
            });

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(30.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn(TextBundle {
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
            });

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
                            width: Val::Px(300.0),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::GRAY.into(),
                        ..default()
                    },
                    PowerUpsToggleButton(false),
                ))
                .with_children(|button| {
                    button.spawn(TextBundle {
                        text: Text::from_section(
                            "Power-ups: OFF",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        ..default()
                    });
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
                    button.spawn(TextBundle {
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
                    });
                });
        });
}

fn power_ups_toggle_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut PowerUpsToggleButton,
            &Children,
        ),
        (Changed<Interaction>, With<PowerUpsToggleButton>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut background_color, mut toggle_button, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                toggle_button.0 = !toggle_button.0;
                *background_color = if toggle_button.0 {
                    Color::DARK_GREEN.into()
                } else {
                    Color::GRAY.into()
                };

                for &child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        text.sections[0].value =
                            format!("Power-ups: {}", if toggle_button.0 { "ON" } else { "OFF" });
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = if toggle_button.0 {
                    Color::GREEN.into()
                } else {
                    Color::DARK_GRAY.into()
                };
            }
            Interaction::None => {
                *background_color = if toggle_button.0 {
                    Color::DARK_GREEN.into()
                } else {
                    Color::GRAY.into()
                };
            }
        }
    }
}

fn main_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    toggle_query: Query<&PowerUpsToggleButton>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameStateResource>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::rgb(0.10, 0.55, 0.10).into();
                if let Ok(toggle_button) = toggle_query.get_single() {
                    game_state.power_ups_enabled = toggle_button.0;
                }
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
    mut state: ResMut<GameStateResource>,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    state.game = Game::new(state.power_ups_enabled);
    state.previous_rows = state.game.get_board().len();
    state.previous_cols = state.game.get_board()[0].len();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.05, 0.05, 0.2, 0.5),
                custom_size: Some(Vec2::new(800.0, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -50.0, -10.0),
            ..default()
        },
        GameBackground,
    ));

    render_game_board(
        &mut commands,
        &state,
        &asset_server,
        &mut meshes,
        &mut materials,
    );

    let (board_width, board_height) = get_board_dimensions(&state);
    adjust_camera(&mut camera_query, board_width, board_height);
}

const COLUMN_COLORS: [Color; 10] = [
    Color::RED,
    Color::GREEN,
    Color::PURPLE,
    Color::ORANGE,
    Color::PINK,
    Color::TEAL,
    Color::CYAN,
    Color::LIME_GREEN,
    Color::INDIGO,
    Color::VIOLET,
];

fn render_game_board(
    commands: &mut Commands,
    state: &GameStateResource,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let rows = state.game.get_board().len();
    let cols = state.game.get_board()[0].len();

    let cell_size = 75.0;
    let padding = 7.5;
    let board_width = cols as f32 * (cell_size + padding) - padding;
    let board_height = rows as f32 * (cell_size + padding) - padding;
    let board_offset_y = -50.0; // Move the board down by 50 units

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
            transform: Transform::from_xyz(0.0, board_offset_y, 0.0),
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

        let color = if col < COLUMN_COLORS.len() {
            COLUMN_COLORS[col]
        } else {
            Color::GOLD
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 45.0,
                        color,
                    },
                ),
                transform: Transform::from_xyz(
                    col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0,
                    board_height / 2.0 + 40.0 + board_offset_y,
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
                            color: Color::RED,
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
                        row as f32 * (cell_size + padding) - board_height / 2.0
                            + cell_size / 2.0
                            + board_offset_y,
                        1.0,
                    ),
                    ..default()
                },
                Cell { row, col, power_up },
                GameUI,
            ));

            if let Some(pu) = power_up {
                spawn_power_up_symbol(
                    commands,
                    row,
                    col,
                    pu,
                    cell_size,
                    padding,
                    board_width,
                    board_height,
                    board_offset_y,
                    asset_server,
                );
            }

            if is_obstacle {
                spawn_obstacle(
                    commands,
                    row,
                    col,
                    cell_size,
                    padding,
                    board_width,
                    board_height,
                    board_offset_y,
                    &mut Some(AnimatePiece {
                        target_y: row as f32 * (cell_size + padding) - board_height / 2.0
                            + cell_size / 2.0
                            + board_offset_y,
                    }),
                );
            }
        }
    }
}

fn spawn_power_up_symbol(
    commands: &mut Commands,
    row: usize,
    col: usize,
    pu: PowerUpType,
    cell_size: f32,
    padding: f32,
    board_width: f32,
    board_height: f32,
    board_offset_y: f32,
    asset_server: &Res<AssetServer>,
) {
    let x = col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0;
    let y =
        row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0 + board_offset_y;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: pu.color(),
                custom_size: Some(Vec2::new(cell_size / 2.0, cell_size / 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.5),
            ..default()
        },
        PowerUpSymbol { row, col },
        GameUI,
    ));

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
            transform: Transform::from_xyz(x, y, 2.0),
            ..default()
        },
        PowerUpSymbol { row, col },
        GameUI,
    ));
}

fn spawn_obstacle(
    commands: &mut Commands,
    row: usize,
    col: usize,
    cell_size: f32,
    padding: f32,
    board_width: f32,
    board_height: f32,
    board_offset_y: f32,
    animate_piece: &mut Option<AnimatePiece>,
) {
    let z = 2.0;
    let initial_y = if animate_piece.is_some() {
        board_height / 2.0 + cell_size + board_offset_y
    } else {
        row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0 + board_offset_y
    };

    let x = col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0;

    let transform = Transform::from_xyz(x, initial_y, z);

    let mut entity_commands = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(cell_size, cell_size)),
                ..default()
            },
            transform,
            ..default()
        },
        StaticObstacle { row, col },
        GameUI,
    ));

    if let Some(animate) = animate_piece.take() {
        entity_commands.insert(animate);
    }
}

fn update_game(
    mut state: ResMut<GameStateResource>,
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
                let player = state.game.get_current_player();

                if state.game.is_full() {
                    if !state.game.expanded {
                        state.game.expand_board();
                        state.game.expanded = true;

                        state.previous_rows = state.game.get_board().len();
                        state.previous_cols = state.game.get_board()[0].len();

                        cleanup_game_board(&mut commands, &game_ui_query);
                        render_game_board(
                            &mut commands,
                            &state,
                            &asset_server,
                            &mut meshes,
                            &mut materials,
                        );

                        let (board_width, board_height) = get_board_dimensions(&state);
                        adjust_camera(&mut camera_query, board_width, board_height);

                        for row in 0..state.game.get_board().len() {
                            for col in 0..state.game.get_board()[0].len() {
                                let cell_char = state.game.get_board()[row][col];
                                if cell_char == PLAYER_X
                                    || cell_char == PLAYER_O
                                    || cell_char == OBSTACLE
                                {
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
                } else {
                    spawn_piece(
                        &mut commands,
                        player,
                        row,
                        col,
                        &mut meshes,
                        &mut materials,
                        &state,
                    );
                }

                let cell_char = state.game.get_board()[row][col];
                if let Some(pu) = PowerUpType::from_char(cell_char) {
                    power_up_activated_events.send(PowerUpActivated {
                        row,
                        col,
                        power_up: pu,
                    });
                }

                if let Some(_winner) = state.game.check_winner() {
                    app_state.set(AppState::GameOver);
                    return;
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
                        Color::RED
                    } else {
                        Color::YELLOW
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
    player: char,
    row: usize,
    col: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    state: &GameStateResource,
) {
    if player == PLAYER_X || player == PLAYER_O {
        let (color, z) = if player == PLAYER_X {
            (Color::RED, 1.9)
        } else {
            (Color::YELLOW, 1.9)
        };

        let cell_size = 75.0;
        let padding = 7.5;
        let cols = state.game.get_board()[0].len();
        let rows = state.game.get_board().len();
        let board_width = cols as f32 * (cell_size + padding) - padding;
        let board_height = rows as f32 * (cell_size + padding) - padding;
        let board_offset_y = -50.0; // Move the board down by 50 units

        let circle_mesh = meshes.add(Mesh::from(Circle::new(cell_size / 2.0 - 5.0)));
        let material_handle = materials.add(ColorMaterial::from(color));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle_mesh.into(),
                material: material_handle,
                transform: Transform::from_xyz(
                    col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0,
                    board_height / 2.0 + cell_size + board_offset_y,
                    z,
                ),
                ..default()
            },
            Piece { player, row, col },
            AnimatePiece {
                target_y: row as f32 * (cell_size + padding) - board_height / 2.0
                    + cell_size / 2.0
                    + board_offset_y,
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
    let cell_char = game.get_board()[row][col];
    let cell_size = 75.0;
    let padding = 7.5;
    let cols = game.get_board()[0].len();
    let rows = game.get_board().len();
    let board_width = cols as f32 * (cell_size + padding) - padding;
    let board_height = rows as f32 * (cell_size + padding) - padding;
    let board_offset_y = -50.0; // Move the board down by 50 units
    let initial_y = board_height / 2.0 + cell_size + board_offset_y;
    let target_y =
        row as f32 * (cell_size + padding) - board_height / 2.0 + cell_size / 2.0 + board_offset_y;

    let x_position = col as f32 * (cell_size + padding) - board_width / 2.0 + cell_size / 2.0;

    if cell_char == PLAYER_X || cell_char == PLAYER_O {
        let color = if cell_char == PLAYER_X {
            Color::RED
        } else {
            Color::YELLOW
        };

        let circle_mesh = meshes.add(Mesh::from(Circle::new(cell_size / 2.0 - 5.0)));
        let material_handle = materials.add(ColorMaterial::from(color));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: circle_mesh.into(),
                material: material_handle,
                transform: Transform::from_xyz(x_position, initial_y, 1.9),
                ..default()
            },
            Piece {
                player: cell_char,
                row,
                col,
            },
            AnimatePiece { target_y },
            GameUI,
        ));
    }

    if cell_char == OBSTACLE {
        spawn_obstacle(
            commands,
            row,
            col,
            cell_size,
            padding,
            board_width,
            board_height,
            board_offset_y,
            &mut None,
        );
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

fn render_final_board(parent: &mut ChildBuilder, game_state: &GameStateResource) {
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
                                Some(Color::RED)
                            } else if player == PLAYER_O {
                                Some(Color::YELLOW)
                            } else if player == OBSTACLE {
                                Some(Color::BLACK)
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
    game_state: Res<GameStateResource>,
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

fn get_board_dimensions(state: &GameStateResource) -> (f32, f32) {
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

        ortho.scaling_mode = ScalingMode::FixedVertical(desired_height);

        ortho.area = Rect {
            min: Vec2::new(-desired_width / 2.0, -desired_height / 2.0),
            max: Vec2::new(desired_width / 2.0, desired_height / 2.0),
        };

        transform.translation = Vec3::new(0.0, 0.0, transform.translation.z);
    }
}

fn handle_power_up_activation(
    mut events: EventReader<PowerUpActivated>,
    mut commands: Commands,
    query: Query<(Entity, &Cell), With<Cell>>,
    piece_query: Query<(Entity, &Piece), With<Piece>>,
    power_up_query: Query<(Entity, &PowerUpSymbol)>,
    state: Res<GameStateResource>,
) {
    for event in events.iter() {
        for (entity, symbol) in power_up_query.iter() {
            if symbol.row == event.row && symbol.col == event.col {
                commands.entity(entity).despawn_recursive();
            }
        }

        for (entity, cell) in query.iter() {
            if cell.row == event.row && cell.col == event.col {
                match event.power_up {
                    PowerUpType::Bomb => {
                        let (board_width, board_height) = get_board_dimensions(&state);
                        let cell_size = 75.0;
                        let padding = 7.5;
                        let board_offset_y = -50.0;

                        commands.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::PURPLE,
                                    custom_size: Some(Vec2::new(50.0, 50.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    cell.col as f32 * (cell_size + padding) - board_width / 2.0
                                        + cell_size / 2.0,
                                    cell.row as f32 * (cell_size + padding) - board_height / 2.0
                                        + cell_size / 2.0
                                        + board_offset_y,
                                    3.0,
                                ),
                                ..default()
                            },
                            Explosion {
                                timer: Timer::from_seconds(0.5, TimerMode::Once),
                            },
                            GameUI,
                        ));

                        if cell.row > 0 {
                            let target_row = cell.row - 1;
                            let target_col = cell.col;

                            for (piece_entity, piece) in piece_query.iter() {
                                if piece.row == target_row && piece.col == target_col {
                                    commands.entity(piece_entity).despawn();
                                    break;
                                }
                            }
                        }
                    }
                    PowerUpType::Skip => {
                        commands.entity(entity).insert(Flashing);
                    }
                    PowerUpType::Obstacle => {
                        let (board_width, board_height) = get_board_dimensions(&state);
                        let cell_size = 75.0;
                        let padding = 7.5;
                        let board_offset_y = -50.0;

                        spawn_obstacle(
                            &mut commands,
                            cell.row,
                            cell.col,
                            cell_size,
                            padding,
                            board_width,
                            board_height,
                            board_offset_y,
                            &mut Some(AnimatePiece {
                                target_y: cell.row as f32 * (cell_size + padding)
                                    - board_height / 2.0
                                    + cell_size / 2.0
                                    + board_offset_y,
                            }),
                        );
                    }
                }
            }
        }
    }
}

fn explosion_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion, &mut Sprite)>,
) {
    for (entity, mut explosion, mut sprite) in query.iter_mut() {
        explosion.timer.tick(time.delta());

        let scale_factor = 1.0 + (0.5 * explosion.timer.elapsed_secs());
        sprite.custom_size = Some(Vec2::splat(50.0 * scale_factor));

        if explosion.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

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
                Color::GREEN
            };
            commands.entity(entity).remove::<Flashing>();
        }
        *timer = 0.0;
    }
}

fn synchronize_frontend(
    state: Res<GameStateResource>,
    mut commands: Commands,
    existing_pieces: Query<(Entity, &Piece), With<Piece>>,
    existing_obstacles: Query<(Entity, &StaticObstacle), With<StaticObstacle>>,
    existing_power_ups: Query<(Entity, &PowerUpSymbol)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let board = state.game.get_board();

    let mut required_piece_positions = HashSet::new();
    let mut required_obstacle_positions = HashSet::new();

    for (row, row_cells) in board.iter().enumerate() {
        for (col, &cell) in row_cells.iter().enumerate() {
            if cell == PLAYER_X || cell == PLAYER_O {
                required_piece_positions.insert((row, col));
            }
            if cell == OBSTACLE {
                required_obstacle_positions.insert((row, col));
            }
        }
    }

    for (entity, piece) in existing_pieces.iter() {
        if !required_piece_positions.contains(&(piece.row, piece.col)) {
            commands.entity(entity).despawn();
        } else {
            required_piece_positions.remove(&(piece.row, piece.col));
        }
    }

    for (entity, obstacle) in existing_obstacles.iter() {
        required_obstacle_positions.remove(&(obstacle.row, obstacle.col));
    }

    for &(row, col) in &required_piece_positions {
        spawn_existing_piece(
            &mut commands,
            &state.game,
            row,
            col,
            &mut meshes,
            &mut materials,
        );
    }

    for &(row, col) in &required_obstacle_positions {
        let cell_size = 75.0;
        let padding = 7.5;
        let board_dimensions = get_board_dimensions(&state);
        let board_width = board_dimensions.0;
        let board_height = board_dimensions.1;
        let board_offset_y = -50.0;

        spawn_obstacle(
            &mut commands,
            row,
            col,
            cell_size,
            padding,
            board_width,
            board_height,
            board_offset_y,
            &mut None,
        );
    }

    let mut required_power_up_positions = HashSet::new();
    for (row, row_cells) in board.iter().enumerate() {
        for (col, &cell) in row_cells.iter().enumerate() {
            if let Some(pu) = PowerUpType::from_char(cell) {
                required_power_up_positions.insert((row, col, pu));
            }
        }
    }

    for (entity, power_up_symbol) in existing_power_ups.iter() {
        if !required_power_up_positions
            .iter()
            .any(|&(r, c, _)| r == power_up_symbol.row && c == power_up_symbol.col)
        {
            commands.entity(entity).despawn_recursive();
        }
    }

    let existing_positions: HashSet<(usize, usize)> = existing_power_ups
        .iter()
        .map(|(_, symbol)| (symbol.row, symbol.col))
        .collect();

    for &(row, col, pu) in &required_power_up_positions {
        if !existing_positions.contains(&(row, col)) {
            let cell_size = 75.0;
            let padding = 7.5;
            let cols = board[0].len();
            let rows = board.len();
            let board_width = cols as f32 * (cell_size + padding) - padding;
            let board_height = rows as f32 * (cell_size + padding) - padding;
            let board_offset_y = -50.0;

            spawn_power_up_symbol(
                &mut commands,
                row,
                col,
                pu,
                cell_size,
                padding,
                board_width,
                board_height,
                board_offset_y,
                &asset_server,
            );
        }
    }
}

pub fn run() {
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
        .insert_resource(GameStateResource::default())
        .add_event::<PowerUpActivated>()
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(
            Update,
            (main_menu_button_system, power_ups_toggle_system).run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(
            Update,
            (
                update_game,
                animate_pieces,
                handle_power_up_activation,
                explosion_animation,
                flash_power_up,
                synchronize_frontend,
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
