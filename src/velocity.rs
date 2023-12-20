use bevy::prelude::*;

use crate::make_unit_struct_random;

#[derive(Component, Deref, Clone)]
pub struct Velocity(pub Vec2);

make_unit_struct_random!(Velocity);
