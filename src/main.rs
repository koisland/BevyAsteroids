use bevy::prelude::*;

pub mod asteroid;
pub mod bullet;
pub mod collision;
pub mod player;
pub mod position;
pub mod ui;
pub mod velocity;

use asteroid::setup_asteroids;
use bullet::{remove_bullets, BulletImage};
use collision::{detect_asteroid_bullet_collisions, detect_asteroid_ship_collisions};
use player::{move_player, Player};
use position::{
    sync_transform_w_position, update_positions, Position, BG_SPRITE_X, BG_SPRITE_Y, BOUNDS_MAX_X,
    BOUNDS_MAX_Y, BOUNDS_MIN_X, BOUNDS_MIN_Y,
};
use ui::UIPlugin;
use velocity::Velocity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        // TODO: Separate into Plugins
        .add_systems(Startup, (setup, setup_asteroids))
        .add_systems(FixedUpdate, (update_positions, sync_transform_w_position))
        .add_systems(
            FixedUpdate,
            (
                move_player,
                remove_bullets,
                detect_asteroid_ship_collisions,
                detect_asteroid_bullet_collisions,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

pub trait GetRandom {
    fn random() -> Self;
}

#[macro_export]
macro_rules! make_unit_struct_random {
    ($name:ident) => {
        use $crate::GetRandom;

        impl GetRandom for $name {
            fn random() -> Self {
                Self(Vec2::new(rand::random::<f32>(), rand::random::<f32>()))
            }
        }
    };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("Animations/obj_player/Default/000.png");
    let bullet_handle = asset_server.load("Animations/obj_bullet/Default/000.png");
    let space_bg_handle = asset_server.load("Textures/tb_space.png");
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());

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

    // player controlled ship
    commands.spawn((
        SpriteBundle {
            texture: ship_handle,
            ..default()
        },
        Player::default(),
        Velocity(Vec2::default()),
        Position::default(),
    ));

    commands.insert_resource(BulletImage(bullet_handle))
}
