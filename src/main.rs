use bevy::prelude::*;

pub mod asteroid;
pub mod bullet;
pub mod collision;
pub mod core;
pub mod player;
pub mod position;
pub mod ui;
pub mod velocity;

use core::GamePlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UIPlugin)
        .add_plugins(GamePlugin)
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
