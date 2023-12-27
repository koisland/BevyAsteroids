use bevy::{audio::PlaybackMode, prelude::*};

#[derive(Resource)]
pub struct BulletFiredAudio(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct AsteroidDestroyedAudio(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct ShipDestroyedAudio(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct VictoryAudio(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct LossAudio(pub Handle<AudioSource>);

#[derive(Component)]
pub struct BGMAudio;

pub fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn background music
    commands.spawn((
        AudioBundle {
            source: asset_server.load("embedded://Files/HoliznaCC0_GameBOI1.mp3"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        BGMAudio,
    ));

    // Store audio handles for later.
    commands.insert_resource(BulletFiredAudio(
        asset_server.load("embedded://Files/shoot.ogg"),
    ));
    commands.insert_resource(AsteroidDestroyedAudio(
        asset_server.load("embedded://Files/hurt.ogg"),
    ));
    commands.insert_resource(ShipDestroyedAudio(
        asset_server.load("embedded://Files/boom.ogg"),
    ));
    // TODO: Win audio.
    commands.insert_resource(VictoryAudio(asset_server.load("embedded://Files/win.ogg")));
    commands.insert_resource(LossAudio(asset_server.load("embedded://Files/lose.ogg")));
}
