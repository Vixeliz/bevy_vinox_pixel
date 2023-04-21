use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{camera, layers};

pub struct PixelPlugins;

/// This component is used to mark sprites. As of right now this is only used for sprite limiting.
#[derive(Component, Copy, Clone)]
pub struct PixelSprite;

impl PluginGroup for PixelPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(camera::plugin::PixelCameraPlugin)
            .add(layers::plugin::PixelLayerPlugin);

        group
    }
}
