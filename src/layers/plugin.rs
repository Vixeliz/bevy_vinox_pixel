use bevy::prelude::*;
use crate::prelude::PixelLayer;
use extol_sprite_layer::*;

pub struct PixelLayerPlugin {
    pub y_sort: bool,
}

impl Plugin for PixelLayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteLayerPlugin::<PixelLayer>::default());
        if self.y_sort {
            app.insert_resource(SpriteLayerOptions { y_sort: true });
        } else {
            app.insert_resource(SpriteLayerOptions { y_sort: false });
        }
    }
}
