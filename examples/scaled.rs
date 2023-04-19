use bevy::prelude::*;
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
    // This will fill up the whole screen you can use from_resolution to get more similar to the texture approach
    commands.spawn(ScaledPixelCamera::from_zoom(4));
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
