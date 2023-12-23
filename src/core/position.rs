use bevy::prelude::*;

use super::velocity::Velocity;
use crate::make_vec2_struct_random;

pub const BG_SPRITE_X: f32 = 256.0;
pub const BG_SPRITE_Y: f32 = 256.0;
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
// origin is center so divide by 2.
// -------
// |  |  |
// |--+--|
// |  |  |
// -------

pub const BOUNDS_MAX_X: f32 = BOUNDS.x / 2.0;
pub const BOUNDS_MIN_X: f32 = -BOUNDS_MAX_X;
pub const BOUNDS_MAX_Y: f32 = BOUNDS.y / 2.0;
pub const BOUNDS_MIN_Y: f32 = -BOUNDS_MAX_Y;

#[derive(Default, Component, Deref, DerefMut, Clone)]
pub struct Position(pub Vec2);

pub fn update_positions(mut query: Query<(&Velocity, &Transform, &mut Position)>) {
    // Board on taurus
    for (velocity, transform, mut position) in &mut query {
        let mut new_pos = position.0 + velocity.0;
        // Look at scale of element and move it when fully off-screen to avoid sudden jump.
        let half_scale = transform.scale.max_element();

        // If passing out of max/min ?-axis view, move it to other side.
        if new_pos.x > BOUNDS_MAX_X + half_scale {
            new_pos.x = BOUNDS_MIN_X - half_scale
        } else if new_pos.x < BOUNDS_MIN_X - half_scale {
            new_pos.x = BOUNDS_MAX_X + half_scale
        }
        if new_pos.y > BOUNDS_MAX_Y + half_scale {
            new_pos.y = BOUNDS_MIN_Y - half_scale
        } else if new_pos.y < BOUNDS_MIN_Y - half_scale {
            new_pos.y = BOUNDS_MAX_Y + half_scale
        }

        *position = Position(new_pos);
    }
}

/// Sync entity transform with position struct.
pub fn sync_transform_w_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation = Vec3::new(position.x, position.y, transform.translation.z)
    }
}

make_vec2_struct_random!(Position);
