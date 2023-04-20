use bevy::prelude::*;
use bevy_vinox_pixel::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPlugins)
        // Cursor only supports scaled at the moment
        .add_plugin(PixelCursorPlugin)
        .add_startup_system(setup)
        .add_systems((rotate_sprite, movement))
        .run();
}

#[derive(Component)]
pub struct Rotate;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // We can also use from_zoom to get a fixed pixel size instead of scaling a virtual window
    commands
        .spawn(ScaledPixelCamera::from_resolution(256, 224, true))
        // .spawn(ScaledPixelCamera::from_zoom(4.0))
        .insert((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(400.0)),

                    ..default()
                },
                ..default()
            },
            PixelLayer::Background(0),
        ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0006.png"),
            ..Default::default()
        },
        Rotate,
        PixelLayer::Foreground(1),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::PURPLE,
                custom_size: Some(Vec2::splat(16.0)),

                ..default()
            },
            transform: Transform::from_xyz(-8.0, 0.0, 0.0),
            ..Default::default()
        },
        PixelLayer::Foreground(2),
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::splat(16.0)),

                ..default()
            },
            transform: Transform::from_xyz(8.0, 0.0, 0.0),
            ..Default::default()
        },
        PixelLayer::Foreground(0),
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
