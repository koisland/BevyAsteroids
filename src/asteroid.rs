use std::{collections::HashMap, path::PathBuf};

use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    position::{Position, BOUNDS},
    velocity::Velocity,
    GetRandom,
};

const ASTEROID_NUM: usize = 6;
const ASTEROID_VELOCITY: f32 = 1.0;

const ASTEROID_IMG_DIR: &str = "Animations/obj_asteroid/Default/";

#[derive(EnumIter, Default, PartialEq, Eq, Hash)]
pub enum AsteroidSize {
    #[default]
    Large,
    Medium,
    Small,
    Tiny,
}

#[derive(Default, Component)]
pub struct Asteroid {
    /// Asteroid size.
    pub size: AsteroidSize,
}

impl AsteroidSize {
    pub fn scale(&self) -> f32 {
        match self {
            AsteroidSize::Large => 55.0,
            AsteroidSize::Medium => 35.0,
            AsteroidSize::Small => 15.0,
            AsteroidSize::Tiny => 7.5,
        }
    }
}

#[derive(Resource, Deref)]
pub struct AsteroidImages(HashMap<AsteroidSize, Handle<Image>>);

pub fn setup_asteroids(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asteroid_images = AsteroidImages(
        AsteroidSize::iter()
            .enumerate()
            .map(|(i, size)| {
                let path: PathBuf = [ASTEROID_IMG_DIR, &format!("00{i}.png")].iter().collect();
                (size, asset_server.load(path))
            })
            .collect(),
    );
    for _ in 0..ASTEROID_NUM {
        let mut pos = Position::random();
        // Within bounds of window.
        pos.x = (pos.x * 2.0 - 1.0) * BOUNDS.x / 2.0;
        pos.y = (pos.y * 2.0 - 1.0) * BOUNDS.y / 2.0;

        let asteroid_size = AsteroidSize::Large;
        let asteroid_img_handle = asteroid_images[&asteroid_size].clone();

        commands.spawn((
            Asteroid {
                size: asteroid_size,
            },
            SpriteBundle {
                texture: asteroid_img_handle,
                // Translation of 1.0 keeps asteroid over ship.
                transform: Transform::default().with_translation(Vec3::new(pos.x, pos.y, 1.0)),
                ..default()
            },
            Velocity(Velocity::random().normalize() * ASTEROID_VELOCITY),
            pos,
        ));
    }
    // Insert asteroid images as a resource to access later.
    commands.insert_resource(asteroid_images);
}
