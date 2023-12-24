#![allow(clippy::type_complexity)]

use bevy::{prelude::*, window::EnabledButtons};

pub mod core;
pub mod ui;

use core::GamePlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins::build(DefaultPlugins).set(WindowPlugin {
            // Disallow resizing window.
            primary_window: Some(Window {
                resizable: false,
                enabled_buttons: EnabledButtons {
                    minimize: true,
                    maximize: false,
                    close: true,
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(UIPlugin)
        .add_plugins(GamePlugin)
        .run();
}

pub trait GetRandom {
    fn random() -> Self;
}

#[macro_export]
macro_rules! make_vec2_struct_random {
    ($name:ident) => {
        use $crate::GetRandom;

        impl GetRandom for $name {
            fn random() -> Self {
                Self(Vec2::new(rand::random::<f32>(), rand::random::<f32>()))
            }
        }
    };
}
