use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_pixel::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelPlugin)
        .add_startup_system(setup)
        .add_system(rotate_sprite)
        .run();
}

#[derive(Component)]
pub struct Rotate;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(400.0)),

            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
        ..default()
    });

    // We can also use from_zoom to get a fixed pixel size instead of scaling a virtual window
    commands.spawn(ScaledPixelCamera::from_resolution(256, 224, true));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0006.png"),
            ..Default::default()
        },
        Rotate,
    ));
}

fn rotate_sprite(mut rotate_query: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut transform in rotate_query.iter_mut() {
        transform.rotate_z(1.5 * time.delta_seconds());
    }
}
