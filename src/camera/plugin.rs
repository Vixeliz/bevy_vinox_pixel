use bevy::{prelude::*, render::camera};

use super::scaled::ScaledPixelProjection;

#[derive(Component)]
pub struct PixelCameraTag;

#[derive(Component)]
pub struct UiCameraTag;

#[derive(Component)]
pub struct CursorCameraTag;

pub struct PixelCameraPlugin;

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            camera::camera_system::<ScaledPixelProjection>.in_base_set(CoreSet::PostUpdate),
        )
        .add_system(super::texture::setup_camera.in_base_set(CoreSet::PostUpdate))
        .add_system(super::scaled::setup_camera.in_base_set(CoreSet::PostUpdate))
        .add_system(super::texture::scale_render_image)
        .add_system(super::scaled::update_scaled_viewport);
    }
}
