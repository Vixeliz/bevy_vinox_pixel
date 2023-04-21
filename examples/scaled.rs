use bevy::prelude::*;
use bevy_vinox_pixel::{plugin::PixelSprite, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPlugins)
        // Cursor only supports scaled at the moment
        .add_plugin(PixelCursorPlugin)
        .add_plugin(PixelLimPlugin::new(4, false))
        /* If you would like to see limited sprites randomly uncomment this plugin. Be warned of flashing images! */
        // .add_plugin(PixelLimPlugin::new(4, true))
        .add_startup_system(setup)
        .add_systems((rotate_sprite, movement))
        .run();
}

#[derive(Component)]
pub struct Rotate;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PixelCursor::new(
        asset_server.load("cursor.png"),
        asset_server.load("cursor_hover.png"),
    ));
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

    // One of these will never be drawn
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0006.png"),
            ..Default::default()
        },
        Rotate,
        PixelLayer::Foreground(1),
        PixelSprite,
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
        PixelSprite,
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
        PixelSprite,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::splat(16.0)),

                ..default()
            },
            transform: Transform::from_xyz(16.0, 0.0, 0.0),
            ..Default::default()
        },
        PixelLayer::Foreground(3),
        PixelSprite,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::splat(16.0)),

                ..default()
            },
            transform: Transform::from_xyz(-16.0, 0.0, 0.0),
            ..Default::default()
        },
        PixelLayer::Foreground(3),
        PixelSprite,
    ));
}

fn rotate_sprite(mut rotate_query: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut transform in rotate_query.iter_mut() {
        transform.rotate_z(1.5 * time.delta_seconds());
    }
}

// Both cameras use a cameratag for easy selection of the right camera
// We also can show that we can manually toggle cursor state
fn movement(
    mut transform_query: Query<&mut Transform, With<PixelCameraTag>>,
    mut cursor_query: Query<&mut PixelCursor>,
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

    if keys.just_pressed(KeyCode::Space) {
        if let Ok(mut cursor) = cursor_query.get_single_mut() {
            cursor.hovering = !cursor.hovering;
        }
    }
}
