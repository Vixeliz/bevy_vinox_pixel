use bevy::{prelude::*, window::PrimaryWindow};

use crate::{camera::texture::FinalCameraTag, prelude::PixelCameraTag};

#[derive(Component)]
pub struct CursorSprite;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct WorldCursorPostion(pub Vec2);

pub fn update_world_cursor(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PixelCameraTag>>,
    is_texture: Query<With<FinalCameraTag>>,
    mut world_cursor: ResMut<WorldCursorPostion>,
) {
    if let Ok((camera, camera_transform)) = camera_q.get_single() {
        if let Ok(window) = windows.get_single() {
            if is_texture.get_single().is_ok() {
                if let Some(world_position) = window
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate())
                {
                    **world_cursor = world_position;
                }
            } else if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                **world_cursor = world_position;
            }
        }
    }
}

pub fn update_cursor(
    mut cursor_query: Query<&mut Transform, With<CursorSprite>>,
    world_cursor: Res<WorldCursorPostion>,
) {
    if let Ok(mut cursor_transform) = cursor_query.get_single_mut() {
        cursor_transform.translation = world_cursor.extend(0.0);
    }
}

pub fn add_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = false;
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("cursor.png"),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
                ..Default::default()
            },
            CursorSprite,
        ));
    }
}
