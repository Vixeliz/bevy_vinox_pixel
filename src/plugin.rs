use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{camera, layers};

pub struct PixelPlugins;

impl PluginGroup for PixelPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(camera::plugin::PixelCameraPlugin)
            .add(layers::plugin::PixelLayerPlugin);

        group
    }
}
