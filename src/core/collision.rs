use bevy::prelude::*;

use super::{
    asteroid::{Asteroid, AsteroidImages, AsteroidSize, ASTEROID_SPLIT_NUM, ASTEROID_VELOCITY},
    audio::{AsteroidDestroyedAudio, LossAudio, ShipDestroyedAudio},
    bullet::Bullet,
    player::Player,
    position::Position,
    velocity::Velocity,
    AppState,
};
use crate::{ui::menu::MenuState, GetRandom};

pub fn detect_asteroid_ship_collisions(
    mut commands: Commands,
    ship_query: Query<(&Transform, &Position), With<Player>>,
    asteroid_query: Query<(Entity, &Asteroid, &Position), With<Asteroid>>,
    ship_destroyed_audio: Res<ShipDestroyedAudio>,
    loss_audio: Res<LossAudio>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    let Ok((ship_transform, ship_pos)) = ship_query.get_single() else {
        return;
    };

    let ship_size = ship_transform.scale.max_element();
    for (_, asteroid, asteroid_pos) in &asteroid_query {
        let asteroid_size = asteroid.size.scale();
        let dst = (ship_pos.0 - asteroid_pos.0).length();

        // Assume hitbox is circle. When centers of objects collide.
        if dst < ship_size + asteroid_size {
            // Play ship destroyed sound once and despawn entity.
            commands.spawn(AudioBundle {
                source: ship_destroyed_audio.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
            // Then play loss audio.
            commands.spawn(AudioBundle {
                source: loss_audio.0.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
            menu_state.set(MenuState::Main);
            game_state.set(AppState::Menu);
            return;
        }
    }
}

pub fn detect_asteroid_bullet_collisions(
    mut commands: Commands,
    asteroid_images: Res<AsteroidImages>,
    asteroid_destroyed_audio: Res<AsteroidDestroyedAudio>,
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
                commands.spawn(AudioBundle {
                    source: asteroid_destroyed_audio.0.clone(),
                    settings: PlaybackSettings::DESPAWN,
                });
                // Each destroyed asteroid provides 1 pt.
                player.score += 1
            }
        }
    }
}

pub fn cleanup_game_entities(
    mut commands: Commands,
    ship_query: Query<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>,
    asteroid_query: Query<Entity, With<Asteroid>>,
) {
    // Despawn player, bullets, and asteroids.
    commands.entity(ship_query.single()).despawn();
    for asteroid_entity in &asteroid_query {
        commands.entity(asteroid_entity).despawn()
    }
    for bullet_entity in &bullet_query {
        commands.entity(bullet_entity).despawn()
    }
}
