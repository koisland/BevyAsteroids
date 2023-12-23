use bevy::prelude::*;

use crate::make_vec2_struct_random;

#[derive(Component, Deref, Clone)]
pub struct Velocity(pub Vec2);

make_vec2_struct_random!(Velocity);
