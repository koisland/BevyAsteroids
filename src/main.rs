use std::{f32::consts::PI, path::PathBuf};

use bevy::{math::vec3, prelude::*, utils::HashMap};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
// origin is center so divide by 2.
// -------
// |  |  |
// |--+--|
// |  |  |
// -------

const BOUNDS_MAX_X: f32 = BOUNDS.x / 2.0;
const BOUNDS_MIN_X: f32 = -BOUNDS_MAX_X;
const BOUNDS_MAX_Y: f32 = BOUNDS.y / 2.0;
const BOUNDS_MIN_Y: f32 = -BOUNDS_MAX_Y;

const SHIP_ROTATION_SPEED: f32 = 10.0 * PI / 360.0;
const SHIP_ACCELERATION: f32 = 0.2;
const SHIP_DECELERATION: f32 = 0.01;
const SHIP_MAX_VELOCITY: f32 = 10.0;

const NUM_ASTEROIDS: usize = 6;
const ASTEROID_VELOCITY: f32 = 1.0;
const BULLET_VELOCITY: f32 = 8.0;
const BULLET_MAX_DISTANCE: f32 = 20000.0;

const ASTEROID_IMG_DIR: &str = "Animations/obj_asteroid/Default/";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, (setup, setup_asteroids))
        .add_systems(FixedUpdate, (update_positions, sync_transform_w_position))
        .add_systems(
            FixedUpdate,
            (
                player_movement_system,
                remove_bullets,
                detect_asteroid_ship_collisions,
                detect_asteroid_bullet_collisions,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Default, Component, Deref, DerefMut)]
struct Position(Vec2);

#[derive(Component, Deref)]
struct Velocity(Vec2);

#[derive(Component)]
struct Bullet {
    prev_pos: Position,
    dst_traveled: f32,
}

trait GetRandom {
    fn random() -> Self;
}

macro_rules! make_unit_struct_random {
    ($name:ident) => {
        impl GetRandom for $name {
            fn random() -> Self {
                Self(Vec2::new(rand::random::<f32>(), rand::random::<f32>()))
            }
        }
    };
}
make_unit_struct_random!(Position);
make_unit_struct_random!(Velocity);

/// player component
#[derive(Default, Component)]
struct Player {
    rotation_angle: f32,
}

impl Player {
    fn direction(&self) -> Vec2 {
        let (y, x) = (self.rotation_angle + (PI / 2.0)).sin_cos();
        Vec2::new(x, y)
    }
}

#[derive(EnumIter, Default, PartialEq, Eq, Hash)]
enum AsteroidSize {
    #[default]
    Large,
    Medium,
    Small,
    Tiny,
}

#[derive(Default, Component)]
struct Asteroid {
    /// Asteroid size.
    size: AsteroidSize,
}

impl AsteroidSize {
    fn scale(&self) -> f32 {
        match self {
            AsteroidSize::Large => 55.0,
            AsteroidSize::Medium => 35.0,
            AsteroidSize::Small => 15.0,
            AsteroidSize::Tiny => 7.5,
        }
    }
}

#[derive(Resource, Deref)]
struct AsteroidImages(HashMap<AsteroidSize, Handle<Image>>);

#[derive(Resource, Deref)]
struct BulletImage(Handle<Image>);

/// Add the game's entities to our world and creates an orthographic camera for 2D rendering.
///
/// The Bevy coordinate system is the same for 2D and 3D, in terms of 2D this means that:
///
/// * `X` axis goes from left to right (`+X` points right)
/// * `Y` axis goes from bottom to top (`+Y` point up)
/// * `Z` axis goes from far to near (`+Z` points towards you, out of the screen)
///
/// The origin is at the center of the screen.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ship_handle = asset_server.load("Animations/obj_player/Default/000.png");
    let bullet_handle = asset_server.load("Animations/obj_bullet/Default/000.png");

    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());

    // player controlled ship
    commands.spawn((
        SpriteBundle {
            texture: ship_handle,
            ..default()
        },
        Player::default(),
        Velocity(Vec2::default()),
        Position::default(),
    ));

    commands.insert_resource(BulletImage(bullet_handle))
}

fn setup_asteroids(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asteroid_images = AsteroidImages(
        AsteroidSize::iter()
            .enumerate()
            .map(|(i, size)| {
                let path: PathBuf = [ASTEROID_IMG_DIR, &format!("00{i}.png")].iter().collect();
                (size, asset_server.load(path))
            })
            .collect(),
    );
    for _ in 0..NUM_ASTEROIDS {
        let mut pos = Position::random();
        // Within bounds of window.
        pos.x = (pos.x * 2.0 - 1.0) * BOUNDS.x / 2.0;
        pos.y = (pos.y * 2.0 - 1.0) * BOUNDS.y / 2.0;

        let asteroid_size = AsteroidSize::Large;
        let asteroid_img_handle = asteroid_images[&asteroid_size].clone();

        commands.spawn((
            Asteroid {
                size: asteroid_size,
            },
            SpriteBundle {
                texture: asteroid_img_handle,
                // Translation of 1.0 keeps asteroid over ship.
                transform: Transform::default().with_translation(vec3(pos.x, pos.y, 1.0)),
                ..default()
            },
            Velocity(Velocity::random().normalize() * ASTEROID_VELOCITY),
            pos,
        ));
    }
    // Insert asteroid images as a resource to access later.
    commands.insert_resource(asteroid_images);
}

fn update_positions(mut query: Query<(&Velocity, &Transform, &mut Position)>) {
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
fn sync_transform_w_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation = Vec3::new(position.x, position.y, transform.translation.z)
    }
}

/// Remove bullets after they travel some distance.
fn remove_bullets(mut commands: Commands, mut query: Query<(Entity, &mut Bullet, &Position)>) {
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

fn detect_asteroid_ship_collisions(
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

fn detect_asteroid_bullet_collisions(
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

/// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
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
