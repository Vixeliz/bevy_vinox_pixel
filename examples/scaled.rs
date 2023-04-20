use bevy::prelude::*;
use bevy_vinox_pixel::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPlugins)
        .add_startup_system(setup)
        .add_systems((rotate_sprite, movement))
        .run();
}

#[derive(Component)]
pub struct Rotate;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // .spawn(ScaledPixelCamera::from_resolution(256, 224, true))
        .spawn(ScaledPixelCamera::from_zoom(4.0))
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(400.0)),

                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
            ..default()
        });

    // We can also use from_zoom to get a fixed pixel size instead of scaling a virtual window
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0006.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
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

// Both cameras use a cameratag for easy selection of the right camera
fn movement(
    mut transform_query: Query<&mut Transform, With<PixelCameraTag>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = transform_query.get_single_mut() {
        let dt = time.delta_seconds();
        if keys.pressed(KeyCode::W) {
            transform.translation.y += 50.0 * dt;
        }
        if keys.pressed(KeyCode::S) {
            transform.translation.y -= 50.0 * dt;
        }
        if keys.pressed(KeyCode::D) {
            transform.translation.x += 50.0 * dt;
        }
        if keys.pressed(KeyCode::A) {
            transform.translation.x -= 50.0 * dt;
        }
    }
}
