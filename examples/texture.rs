use bevy::prelude::*;
use bevy_pixel::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(PixelPlugin)
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(TexturePixelCamera::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("tile_0006.png"),
        ..Default::default()
    });
}
// Both cameras use a cameratag for easy selection of the right camera
fn movement(
    mut transform_query: Query<&mut Transform, With<PixelCameraTag>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = transform_query.get_single_mut() {
        let dt = time.delta_seconds();
        // With the texture camera we can't move in small incrementes like .5 without artifcating so you must round to pixels
        // In the future i may provide a position component for exact pixel positions.
        if keys.pressed(KeyCode::W) {
            transform.translation.y += (50.0 * dt).round();
        }
        if keys.pressed(KeyCode::S) {
            transform.translation.y -= (50.0 * dt).round();
        }
        if keys.pressed(KeyCode::D) {
            transform.translation.x += (50.0 * dt).round();
        }
        if keys.pressed(KeyCode::A) {
            transform.translation.x -= (50.0 * dt).round();
        }
    }
}
