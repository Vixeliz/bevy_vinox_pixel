use bevy::prelude::*;

use super::system::update_z_coordinate_based_on_layer;

/// The plugin that handles everything related to layers
pub struct PixelLayerPlugin;

impl Plugin for PixelLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_z_coordinate_based_on_layer);
    }
}
