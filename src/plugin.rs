use bevy::{prelude::*, render::camera};

use crate::scaled::ScaledPixelProjection;

pub struct PixelPlugin;

impl Plugin for PixelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            camera::camera_system::<ScaledPixelProjection>.in_base_set(CoreSet::PostUpdate),
        )
        .add_system(crate::texture::setup_camera.in_base_set(CoreSet::PostUpdate))
        // .add_system(crate::texture::scale_render_image.in_base_set(CoreSet::PostUpdate));
        .add_system(crate::texture::scale_render_image)
        .add_system(crate::scaled::update_scaled_viewport);
    }
}
