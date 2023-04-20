use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};

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
                    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                {
                    **world_cursor = world_position;
                }
            } else {
                if let Some(physical_cursor) = window.cursor_position() {
                    if let Some((viewport_min, viewport_max)) = camera.logical_viewport_rect() {
                        let cursor_x = physical_cursor
                            .x
                            .clamp(viewport_min.x as f32, viewport_max.x as f32);
                        let cursor_y = physical_cursor
                            .y
                            .clamp(viewport_min.y as f32, viewport_max.y as f32);
                        let cursor_x = ((cursor_x - viewport_min.x as f32)
                            / (window.width() as f32 - viewport_min.x as f32))
                            * (1.0 - 0.0)
                            + 0.0;
                        let cursor_y = ((cursor_y - viewport_min.y as f32)
                            / (window.height() as f32 - viewport_min.y as f32))
                            * (1.0 - 0.0)
                            + 0.0;
                        let cursor_x =
                            ((cursor_x - 0.0) / (1.0 - 0.0)) * (viewport_max.x as f32 - 0.0) + 0.0;
                        let cursor_y =
                            ((cursor_y - 0.0) / (1.0 - 0.0)) * (viewport_max.y as f32 - 0.0) + 0.0;
                        if let Some(world_position) = camera
                            .viewport_to_world_2d(camera_transform, Vec2::new(cursor_x, cursor_y))
                        {
                            **world_cursor = world_position;
                        }
                    } else {
                        if let Some(world_position) =
                            camera.viewport_to_world_2d(camera_transform, physical_cursor)
                        {
                            **world_cursor = world_position;
                        }
                    }
                }
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
