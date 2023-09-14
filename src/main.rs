use bevy::{prelude::*, audio::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, sans_animation)
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
pub struct SanmsFace;


fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("sanssprites_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 30.0), 15, 1, Some(Vec2{x: 5.0, y: 0.0}), Some(Vec2{x: 5.0, y: 519.0}));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 14 };



    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn sans_animation(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &AnimationIndices,
    )>,
    keyboard: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Space) == true {
        for (mut sprite, indicies) in &mut query {
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
}