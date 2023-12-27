use std::collections::HashMap;

use bevy::prelude::*;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::EnumIter;

use super::{
    position::{Position, BOUNDS},
    velocity::Velocity,
};
use crate::GetRandom;

const ASTEROID_NUM: usize = 12;
pub const ASTEROID_SPLIT_NUM: usize = 2;
pub const ASTEROID_VELOCITY: f32 = 1.0;

const ASTEROID_IMG_DIR: &str = "embedded://Animations/obj_asteroid/Default";

#[derive(Debug, EnumIter, EnumCount, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AsteroidSize {
    #[default]
    Large,
    Medium,
    Small,
    Tiny,
}

#[derive(Default, Component, Clone)]
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

impl From<AsteroidSize> for usize {
    fn from(value: AsteroidSize) -> Self {
        match value {
            AsteroidSize::Large => 3,
            AsteroidSize::Medium => 2,
            AsteroidSize::Small => 1,
            AsteroidSize::Tiny => 0,
        }
    }
}

impl From<usize> for AsteroidSize {
    fn from(value: usize) -> Self {
        match value.clamp(0, AsteroidSize::COUNT - 1) {
            3 => AsteroidSize::Large,
            2 => AsteroidSize::Medium,
            1 => AsteroidSize::Small,
            0 => AsteroidSize::Tiny,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Resource, Deref)]
pub struct AsteroidImages(HashMap<AsteroidSize, Handle<Image>>);

pub fn setup_asteroids(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asteroid_images = AsteroidImages(
        AsteroidSize::iter()
            .enumerate()
            .map(|(i, size)| {
                (
                    size,
                    asset_server.load(format!("{ASTEROID_IMG_DIR}/00{i}.png")),
                )
            })
            .collect(),
    );
    for _ in 0..ASTEROID_NUM {
        let mut pos = Position::random();
        // Within bounds of window.
        pos.x *= BOUNDS.x;
        pos.y *= BOUNDS.y;

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
