use bevy::prelude::*;

pub mod asteroid;
pub mod bullet;
pub mod collision;
pub mod player;
pub mod position;
pub mod velocity;

use asteroid::setup_asteroids;
use bullet::{remove_bullets, BulletImage};
use collision::{detect_asteroid_bullet_collisions, detect_asteroid_ship_collisions};
use player::{move_player, Player};
use position::{sync_transform_w_position, update_positions, Position};
use velocity::Velocity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
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

    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());

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
