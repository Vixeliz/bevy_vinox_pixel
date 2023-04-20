use bevy::{prelude::*, render::view::RenderLayers, window::PrimaryWindow};

use crate::{
    camera::{plugin::CursorCameraTag, scaled::ScaledPixelProjection, texture::FinalCameraTag},
    prelude::PixelCameraTag,
};

#[derive(Component)]
pub struct PixelCursor {
    pub normal: Handle<Image>,
    pub hover: Handle<Image>,
    pub init: bool,
    pub hovering: bool,
}

impl PixelCursor {
    pub fn new(normal: Handle<Image>, hover: Handle<Image>) -> Self {
        Self {
            normal,
            hover,
            init: false,
            hovering: false,
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct WorldCursorPostion(pub Vec2);

/// Updates the world position of the cursor. This is no longer used for drawing the cursor but we still need for interaction with the world
pub fn update_world_cursor(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PixelCameraTag>>,
    is_texture: Query<With<FinalCameraTag>>,
    mut world_cursor: ResMut<WorldCursorPostion>,
    mut notified: Local<bool>,
    touches: Res<Touches>,
) {
    if let Ok((camera, camera_transform)) = camera_q.get_single() {
        if let Ok(window) = windows.get_single() {
            if is_texture.get_single().is_ok() {
                if !*notified {
                    *notified = true;
                    panic!("Texture cameras do not support the cursor yet!");
                }
            } else if let Some(physical_cursor) = window.cursor_position() {
                if let Some((viewport_min, viewport_max)) = camera.logical_viewport_rect() {
                    let cursor_x = physical_cursor.x.clamp(viewport_min.x, viewport_max.x);
                    let cursor_y = physical_cursor.y.clamp(viewport_min.y, viewport_max.y);
                    let cursor_x = ((cursor_x - viewport_min.x)
                        / (window.width() - viewport_min.x))
                        * (1.0 - 0.0)
                        + 0.0;
                    let cursor_y = ((cursor_y - viewport_min.y)
                        / (window.height() - viewport_min.y))
                        * (1.0 - 0.0)
                        + 0.0;
                    let cursor_x = ((cursor_x - 0.0) / (1.0 - 0.0)) * (viewport_max.x - 0.0) + 0.0;
                    let cursor_y = ((cursor_y - 0.0) / (1.0 - 0.0)) * (viewport_max.y - 0.0) + 0.0;
                    if let Some(world_position) =
                        camera.viewport_to_world_2d(camera_transform, Vec2::new(cursor_x, cursor_y))
                    {
                        **world_cursor = world_position;
                    }
                } else if let Some(world_position) =
                    camera.viewport_to_world_2d(camera_transform, physical_cursor)
                {
                    **world_cursor = world_position;
                }
            } else if let Some(touch) = touches.iter().next() {
                let mut physical_cursor = touch.position();
                physical_cursor.y = window.height() - physical_cursor.y;
                if let Some((viewport_min, viewport_max)) = camera.logical_viewport_rect() {
                    let cursor_x = physical_cursor.x.clamp(viewport_min.x, viewport_max.x);
                    let cursor_y = physical_cursor.y.clamp(viewport_min.y, viewport_max.y);
                    let cursor_x = ((cursor_x - viewport_min.x)
                        / (window.width() - viewport_min.x))
                        * (1.0 - 0.0)
                        + 0.0;
                    let cursor_y = ((cursor_y - viewport_min.y)
                        / (window.height() - viewport_min.y))
                        * (1.0 - 0.0)
                        + 0.0;
                    let cursor_x = ((cursor_x - 0.0) / (1.0 - 0.0)) * (viewport_max.x - 0.0) + 0.0;
                    let cursor_y = ((cursor_y - 0.0) / (1.0 - 0.0)) * (viewport_max.y - 0.0) + 0.0;
                    if let Some(world_position) =
                        camera.viewport_to_world_2d(camera_transform, Vec2::new(cursor_x, cursor_y))
                    {
                        **world_cursor = world_position;
                    } else if let Some(world_position) =
                        camera.viewport_to_world_2d(camera_transform, physical_cursor)
                    {
                        **world_cursor = world_position;
                    }
                }
            }
        }
    }
}

pub fn update_cursor(
    mut cursor_query: Query<(&mut Transform, &mut PixelCursor, &mut Handle<Image>)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<CursorCameraTag>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    pixel_query: Query<&ScaledPixelProjection>,
    touches: Res<Touches>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    if let Ok((mut cursor_transform, mut cursor_sprite, mut cursor_handle)) =
        cursor_query.get_single_mut()
    {
        for interaction in interaction_query.iter() {
            match *interaction {
                Interaction::Clicked => {
                    cursor_sprite.hovering = true;
                }
                Interaction::Hovered => {
                    cursor_sprite.hovering = true;
                }
                Interaction::None => {
                    cursor_sprite.hovering = false;
                }
            }
        }
        if let Ok(window) = windows.get_single() {
            if cursor_sprite.init {
                if cursor_sprite.hovering && *cursor_handle != cursor_sprite.hover {
                    *cursor_handle = cursor_sprite.hover.clone();
                } else if *cursor_handle != cursor_sprite.normal && !cursor_sprite.hovering {
                    *cursor_handle = cursor_sprite.normal.clone();
                }
                if let Ok((camera, transform)) = camera_q.get_single() {
                    if let Ok(pixel) = pixel_query.get_single() {
                        if let Some(world_position) = touches.iter().next().and_then(|cursor| {
                            let mut cursor = cursor.position();
                            cursor.y = window.height() - cursor.y;

                            camera.viewport_to_world_2d(transform, cursor)
                        }) {
                            cursor_transform.translation = world_position.extend(0.0);
                            cursor_transform.scale = Vec2::splat(pixel.zoom).extend(1.0);
                        } else if let Some(world_position) = window
                            .cursor_position()
                            .and_then(|cursor| camera.viewport_to_world_2d(transform, cursor))
                        {
                            cursor_transform.translation = world_position.extend(0.0);
                            cursor_transform.scale = Vec2::splat(pixel.zoom).extend(1.0);
                        }
                    }
                }
            }
        }
    }
}

pub fn add_cursor(
    mut commands: Commands,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut cursor_query: Query<(&mut PixelCursor, Entity)>,
) {
    let cursor_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 3) as u8);
    if let Ok((mut cursor_sprite, entity)) = cursor_query.get_single_mut() {
        if !cursor_sprite.init {
            if let Ok(mut window) = windows.get_single_mut() {
                window.cursor.visible = false;
                commands.entity(entity).insert((
                    SpriteBundle {
                        texture: cursor_sprite.normal.clone(),
                        transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
                        ..Default::default()
                    },
                    cursor_layer,
                ));
                cursor_sprite.init = true;
            }
        }
    }
}
