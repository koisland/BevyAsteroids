

//! Demonstrates rotating entities in 2D using quaternions.

use bevy::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const PLAYER_MOVE_SPEED: f32 = 500.0;
const PLAYER_ROTATION_SPEED: f32 = 360.0;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate, player_movement_system
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, sync_positions)
        .run();
}


#[derive(Component)]
struct Position {
    x: f32,
    y: f32
}

/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}


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
    // let enemy_a_handle = asset_server.load("Animations/obj_player/Default/000.png");
    // let enemy_b_handle = asset_server.load("Animations/obj_player/Default/000.png");

    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());

    // let horizontal_margin = BOUNDS.x / 4.0;
    // let vertical_margin = BOUNDS.y / 4.0;

    // player controlled ship
    commands.spawn((
        SpriteBundle {
            texture: ship_handle,
            ..default()
        },
        Player {
            movement_speed: PLAYER_MOVE_SPEED,                  // meters per second
            rotation_speed: f32::to_radians(PLAYER_ROTATION_SPEED), // degrees per second
        },
    )).insert(Position { x: 0.0, y: 0.0});
}

/// Demonstrates applying rotation and movement based on keyboard input.
fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    // get the ship's forward vector by applying the current rotation to the ships initial facing
    // vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta
    // time
    let movement_distance = movement_factor * ship.movement_speed * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}


fn sync_positions(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.x, pos.y, 0.0)
    }
}