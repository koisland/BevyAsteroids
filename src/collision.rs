use bevy::prelude::*;

use crate::{asteroid::Asteroid, bullet::Bullet, player::Player, position::Position};

pub fn detect_asteroid_ship_collisions(
    mut commands: Commands,
    ship_query: Query<(Entity, &Transform, &Position), With<Player>>,
    asteroid_query: Query<(&Asteroid, &Position), With<Asteroid>>,
) {
    let Ok((ship_entity, ship_transform, ship_pos)) = ship_query.get_single() else {
        return;
    };

    let ship_size = ship_transform.scale.max_element();

    for (asteroid, asteroid_pos) in &asteroid_query {
        let asteroid_size = asteroid.size.scale();
        let dst = (ship_pos.0 - asteroid_pos.0).length();

        // Assume hitbox is circle. When centers of objects collide.
        // TODO: Add death screen and explosion or something.
        if dst < ship_size + asteroid_size {
            commands.entity(ship_entity).despawn()
        }
    }
}

pub fn detect_asteroid_bullet_collisions(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &Bullet, &Position), With<Bullet>>,
    mut asteroid_query: Query<(Entity, &Asteroid, &Position), With<Asteroid>>,
) {
    for (bullet_entity, bullet_transform, _bullet, bullet_pos) in &mut bullet_query {
        let bullet_size = bullet_transform.scale.max_element();
        for (asteroid_entity, asteroid, asteroid_pos) in &mut asteroid_query {
            let asteroid_size = asteroid.size.scale();
            let dst = (bullet_pos.0 - asteroid_pos.0).length();

            if dst < bullet_size + asteroid_size {
                commands.entity(bullet_entity).despawn();
                commands.entity(asteroid_entity).despawn();
            }
        }
    }
}
