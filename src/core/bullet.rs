use bevy::prelude::*;

use super::position::Position;

pub const BULLET_VELOCITY: f32 = 8.0;
pub const BULLET_MAX_DISTANCE: f32 = 20000.0;

#[derive(Component)]
pub struct Bullet {
    pub prev_pos: Position,
    pub dst_traveled: f32,
}

#[derive(Resource, Deref)]
pub struct BulletImage(pub Handle<Image>);

/// Remove bullets after they travel some distance.
pub fn remove_bullets(mut commands: Commands, mut query: Query<(Entity, &mut Bullet, &Position)>) {
    for (entity, mut bullet, pos) in &mut query {
        // Check distance of bullet entity by subtracting from its origin.
        // Delete
        let dst_traveled = pos.0.distance(bullet.prev_pos.0);
        bullet.dst_traveled += dst_traveled;
        if bullet.dst_traveled > BULLET_MAX_DISTANCE {
            commands.entity(entity).despawn()
        }
    }
}
