use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{camera, layers};

pub struct PixelPlugins {
    pub y_sort: bool,
}

impl Default for PixelPlugins {
    fn default() -> Self {
        Self { y_sort: false }
    }
}

/// This component is used to mark sprites. As of right now this is only used for sprite limiting.
#[derive(Component, Copy, Clone)]
pub struct PixelSprite;

impl PluginGroup for PixelPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group.add(camera::plugin::PixelCameraPlugin);
        if self.y_sort {
            group = group.add(layers::plugin::PixelLayerPlugin { y_sort: true });
        } else {
            group = group.add(layers::plugin::PixelLayerPlugin { y_sort: false });
        }
        group
    }
}
