use std::f32::consts::PI;

use bevy::prelude::*;

use super::{
    bullet::{Bullet, BulletImage, BULLET_VELOCITY},
    position::Position,
    velocity::Velocity,
};

const SHIP_ROTATION_SPEED: f32 = 10.0 * PI / 360.0;
const SHIP_ACCELERATION: f32 = 0.2;
const SHIP_DECELERATION: f32 = 0.01;
const SHIP_MAX_VELOCITY: f32 = 10.0;

/// player component
#[derive(Default, Component)]
pub struct Player {
    pub rotation_angle: f32,
    pub score: usize,
}

impl Player {
    fn direction(&self) -> Vec2 {
        // https://en.wikipedia.org/wiki/Trigonometric_functions
        let (y, x) = (self.rotation_angle + (PI / 2.0)).sin_cos();
        Vec2::new(x, y)
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    bullet_image: Res<BulletImage>,
    mut commands: Commands,
    mut query: Query<(&mut Player, &mut Position, &mut Velocity, &mut Transform)>,
) {
    let Ok((mut ship, pos, mut velocity, mut transform)) = query.get_single_mut() else {
        return;
    };

    // Pivot ship.
    if keyboard_input.pressed(KeyCode::Left) {
        ship.rotation_angle += SHIP_ROTATION_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        ship.rotation_angle -= SHIP_ROTATION_SPEED;
    }
    transform.rotation = Quat::from_rotation_z(ship.rotation_angle);

    // Accelerate and decelerate ship.
    if keyboard_input.pressed(KeyCode::Up) {
        velocity.0 += ship.direction() * SHIP_ACCELERATION;

        // Limit ship velocity.
        if velocity.0.length() > SHIP_MAX_VELOCITY {
            velocity.0 = velocity.0.normalize_or_zero() * SHIP_MAX_VELOCITY
        }
    } else {
        velocity.0 *= 1.0 - SHIP_DECELERATION
    }

    // Add bullet.
    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((
            Bullet {
                prev_pos: Position(**pos),
                dst_traveled: 0.0,
            },
            SpriteBundle {
                texture: bullet_image.clone(),
                // Spawn bullet at ship's present position.
                transform: Transform::default()
                    .with_translation(transform.translation)
                    // Use current ship rotation and reorient so perpendicular to ship.
                    .with_rotation(
                        transform.rotation * Quat::from_rotation_z(90.0_f32.to_radians()),
                    ),
                ..Default::default()
            },
            Velocity(ship.direction().normalize() * BULLET_VELOCITY),
            Position(**pos),
        ));
    }
}
