use bevy::prelude::*;

use crate::{
    asteroid::{Asteroid, AsteroidImages, AsteroidSize, ASTEROID_SPLIT_NUM, ASTEROID_VELOCITY},
    bullet::Bullet,
    player::Player,
    position::Position,
    velocity::Velocity,
    GetRandom,
};

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
    asteroid_images: Res<AsteroidImages>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut bullet_query: Query<(Entity, &Transform, &Bullet, &Position), With<Bullet>>,
    mut asteroid_query: Query<(Entity, &Asteroid, &Position), With<Asteroid>>,
) {
    let Ok(mut player) = player_query.get_single_mut() else {
        return;
    };
    for (bullet_entity, bullet_transform, _bullet, bullet_pos) in &mut bullet_query {
        let bullet_size = bullet_transform.scale.max_element();
        for (asteroid_entity, asteroid, asteroid_pos) in &mut asteroid_query {
            let asteroid_size = asteroid.size.scale();
            let dst = (bullet_pos.0 - asteroid_pos.0).length();

            if dst < bullet_size + asteroid_size {
                // TODO: Cause damage to asteroid?
                commands.entity(bullet_entity).despawn();

                // Get smaller sized asteroid and spawn more if possible.
                // Smaller asteroids go in random direction.
                // Otherwise, despawn smallest.
                if let Some(new_asteroid_size) = usize::from(asteroid.size)
                    .checked_sub(1)
                    .map(AsteroidSize::from)
                {
                    let asteroid_img_handle = asteroid_images[&new_asteroid_size].clone();
                    for _ in 0..ASTEROID_SPLIT_NUM {
                        commands.spawn((
                            Asteroid {
                                size: new_asteroid_size,
                            },
                            SpriteBundle {
                                texture: asteroid_img_handle.clone(),
                                // Translation of 1.0 keeps asteroid over ship.
                                transform: Transform::default().with_translation(Vec3::new(
                                    asteroid_pos.x,
                                    asteroid_pos.y,
                                    1.0,
                                )),
                                ..default()
                            },
                            Velocity(Velocity::random().normalize() * ASTEROID_VELOCITY),
                            Position(**asteroid_pos),
                        ));
                    }
                }

                commands.entity(asteroid_entity).despawn();

                // Each destroyed asteroid provides 1 pt.
                player.score += 1
            }
        }
    }
}
