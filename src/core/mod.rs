use bevy::prelude::*;

use crate::asteroid::setup_asteroids;
use crate::bullet::{remove_bullets, BulletImage};
use crate::collision::{detect_asteroid_bullet_collisions, detect_asteroid_ship_collisions};
use crate::player::{move_player, Player};
use crate::position::{
    sync_transform_w_position, update_positions, Position, BG_SPRITE_X, BG_SPRITE_Y, BOUNDS_MAX_X,
    BOUNDS_MAX_Y, BOUNDS_MIN_X, BOUNDS_MIN_Y,
};
use crate::velocity::Velocity;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0))
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
            );
    }
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
