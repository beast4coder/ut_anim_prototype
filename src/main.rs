use bevy::{prelude::*, audio::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, sans_face_anim_sys)
        .add_systems(Update, sans_torso_yeet_sys)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct SansFace;

#[derive(Component)]
pub struct SansTorso;


fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands.spawn(Camera2dBundle::default());

    let sans_texture_handle = asset_server.load("sanssprites_transparent.png");
    
    let sans_face_ta = TextureAtlas::from_grid(sans_texture_handle.clone(), Vec2::new(32.0, 30.0), 15, 2, Some(Vec2{x: 5.0, y: 1.0}), Some(Vec2{x: 5.0, y: 519.0}));
    let sans_face_tah = texture_atlases.add(sans_face_ta);

    let sans_torso_ta = TextureAtlas::from_grid(sans_texture_handle.clone(), Vec2::new(96.0, 47.0), 6, 1, Some(Vec2{x: 6.0, y: 0.0}), Some(Vec2{x: 5.0, y: 390.0}));
    let sans_torso_tah = texture_atlases.add(sans_torso_ta);

    let sans_face_animi = AnimationIndices { first: 0, last: 14 };
    let sans_torso_yeet = AnimationIndices {first: 0, last: 5};

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sans_face_tah,
            sprite: TextureAtlasSprite::new(sans_face_animi.first),
            transform: Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::new(0.0, 34.0*2.0, 0.0)),
            ..default()
        },
        sans_face_animi,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        SansFace,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sans_torso_tah,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(4.0)).with_translation(Vec3::new(60.0, -34.0*2.0, -1.0)),
            ..default()
        },
        SansTorso,
        sans_torso_yeet,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn sans_face_anim_sys(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &AnimationIndices,
        &SansFace,
    )>,
    keyboard: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Space) == true {
        for (mut sprite, indicies, _) in &mut query {
            sprite.index = if sprite.index == indicies.last {
                indicies.first
            } else {
                sprite.index + 1
            };
        }

        commands.spawn(AudioBundle{
            source: asset_server.load("sans_1.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::Relative(VolumeLevel::new(0.5)),
                ..default()
            }
        });
    }

    if keyboard.just_pressed(KeyCode::Key1) == true {
        for (mut sprite, _, _) in &mut query {
            sprite.index = 0;
        }

        commands.spawn(AudioBundle{
            source: asset_server.load("sans_2.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                volume: Volume::Absolute(VolumeLevel::new(2.0)),
                ..default()
            }
        });
    }
}

fn sans_torso_yeet_sys(
    //time: Res<Time>,
    mut torso_query: Query<(
        &mut TextureAtlasSprite,
        &AnimationIndices,
        &mut AnimationTimer,
        &SansTorso,
    )>,
    mut head_query: Query<(
        &mut Transform,
        &SansFace,
    )>,
    keyboard: Res<Input<KeyCode>>,
    //asset_server: Res<AssetServer>,
    //mut commands: Commands,
){
    if keyboard.just_pressed(KeyCode::Left) == true{
        for (mut torso_sprite, indicies, _, _) in &mut torso_query{
            torso_sprite.index = if torso_sprite.index == indicies.last{
                indicies.first
            } else {
                torso_sprite.index+1
            };
            for (mut transform, _) in &mut head_query{
                let movement = match torso_sprite.index {
                    0 => 0.0,
                    1 => -6.0,
                    2 => -8.0,
                    3 => 6.0,
                    4 => 2.0,
                    5 => 2.0,
                    _ => 69.0,
                };
                transform.translation.x = movement;
            }
        }
    }
}