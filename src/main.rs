use bevy::{prelude::*, audio::*};

fn main() {
    App::new()
        //stops the sprites becoming blurry
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        //sets background colour to black
        .insert_resource(ClearColor(Color::BLACK))
        //runs once before everything when the app is launched
        .add_systems(Startup, setup)
        //executes periodically every frame while the app runs
        .add_systems(Update, sans_face_anim_sys)
        .add_systems(Update, sans_torso_logic_sys)
        //.add_systems(Update, sans_torso_movement_sys.run_if(is_sans_moving))
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
struct SansTorso{
    pub moving: bool,
}


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

    let sans_torso_comp = SansTorso{moving: false};

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
        sans_torso_comp,
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

fn sans_torso_logic_sys(
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
    mut commands: Commands,
){
    if keyboard.just_pressed(KeyCode::Left) == true{
        for(mut torso_sprite, _, _, mut torso_comp) in &mut torso_query{
            if torso_comp.moving == false{
                //using commands to change the contents of the torso_comp struct
                torso_comp.moving = true;
                torso_sprite.index = 0;
            }
        };
    }

    for (mut torso_sprite, indicies, _, torso_comp) in &mut torso_query{
        if torso_comp.moving == true{ 
                torso_sprite.index = if torso_sprite.index == indicies.last{
                    indicies.first
                } else {
                    torso_sprite.index+1
                };
                for (mut transform, _) in &mut head_query{
                    let movement = match torso_sprite.index {
                        0 => 0.0,
                        1 => -7.0,
                        2 => -10.0,
                        3 => 7.0,
                        4 => 3.0,
                        5 => 3.0,
                        _ => 69.0,
                    };
                    transform.translation.x = movement;
            }
        }
    }
}

fn sans_torso_movement_sys(
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
) {
    for (mut torso_sprite, indicies, _, _) in &mut torso_query{ 
        torso_sprite.index = if torso_sprite.index == indicies.last{
            indicies.first
        } else {
            torso_sprite.index+1
        };
        for (mut transform, _) in &mut head_query{
            let movement = match torso_sprite.index {
                0 => 0.0,
                1 => -7.0,
                2 => -10.0,
                3 => 7.0,
                4 => 3.0,
                5 => 3.0,
                _ => 69.0,
            };
            transform.translation.x = movement;
        }
    }
}

fn is_sans_moving (
    mut torso_query: Query<(
        &mut TextureAtlasSprite,
        &AnimationIndices,
        &mut AnimationTimer,
        &SansTorso,
    )>,
) -> bool {
    for (_, _, _, torso_comp) in &mut torso_query{
        if torso_comp.moving == true{
            return true;
        }
    }
    return false;
}