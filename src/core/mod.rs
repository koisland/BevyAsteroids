use bevy::prelude::*;

pub mod asteroid;
pub mod audio;
pub mod bullet;
pub mod collision;
pub mod player;
pub mod position;
pub mod velocity;

use asteroid::setup_asteroids;
use bullet::remove_bullets;
use collision::{detect_asteroid_bullet_collisions, detect_asteroid_ship_collisions};
use player::move_player;
use position::{
    sync_transform_w_position, update_positions, BG_SPRITE_X, BG_SPRITE_Y, BOUNDS_MAX_X,
    BOUNDS_MAX_Y, BOUNDS_MIN_X, BOUNDS_MIN_Y,
};

use self::{
    audio::setup_audio,
    collision::{check_win_condition, cleanup_game_entities},
    player::setup_player,
};

pub struct GamePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
            .add_state::<AppState>()
            .add_systems(Startup, setup_audio)
            .add_systems(Startup, (setup_camera, setup_background))
            .add_systems(
                OnTransition {
                    from: AppState::Menu,
                    to: AppState::InGame,
                },
                (setup_player, setup_asteroids),
            )
            .add_systems(
                FixedUpdate,
                (update_positions, sync_transform_w_position).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                (
                    move_player,
                    remove_bullets,
                    detect_asteroid_ship_collisions,
                    detect_asteroid_bullet_collisions,
                    check_win_condition,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(FixedUpdate, pause_continue_game)
            .add_systems(
                OnTransition {
                    from: AppState::InGame,
                    to: AppState::Menu,
                },
                cleanup_game_entities,
            );
    }
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let space_bg_handle = asset_server.load("embedded://Textures/tb_space.png");

    // Spawn background texture across entire screen bounds.
    let mut x_bg_pos = BOUNDS_MIN_X - BG_SPRITE_X;
    let x_max_bg_pos = BOUNDS_MAX_X + BG_SPRITE_X;
    let y_bg_pos = BOUNDS_MIN_Y - BG_SPRITE_Y;
    let y_max_bg_pos = BOUNDS_MAX_Y + BG_SPRITE_Y;
    // Floating point step range. Rust doesn't allow.
    while x_bg_pos < x_max_bg_pos {
        let mut y_bg_pos_new_row = y_bg_pos;
        while y_bg_pos_new_row < y_max_bg_pos {
            commands.spawn(SpriteBundle {
                texture: space_bg_handle.clone(),
                transform: Transform::from_translation(Vec3::new(x_bg_pos, y_bg_pos_new_row, 0.0)),
                ..default()
            });
            y_bg_pos_new_row += BG_SPRITE_Y
        }
        x_bg_pos += BG_SPRITE_X;
    }
}

pub fn pause_continue_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match *app_state.get() {
            AppState::InGame => game_state.set(AppState::Paused),
            AppState::Paused => game_state.set(AppState::InGame),
            _ => unreachable!(),
        }
    }
}
