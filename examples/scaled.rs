use bevy::prelude::*;
use bevy_pixel::{prelude::*, scaled::ScaledPixelCamera};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // This will fill up the whole screen you can use from_resolution to get more similar to the texture approach
    commands.spawn(ScaledPixelCamera::from_zoom(4));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tile_0006.png"),
        ..Default::default()
    });
}
