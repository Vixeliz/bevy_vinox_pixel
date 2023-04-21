use bevy::prelude::*;

use super::system::sprite_count_limiter;

/// The plugin that handles everything related to limitations such as sprite count, palette, etc
/// A sprite count of 0 means that this limitation is disabled
pub struct PixelLimPlugin {
    pub sprite_count: u32,
    pub random: bool,
}

#[derive(Resource, Clone, Copy)]
pub struct SpriteCount {
    pub count: u32,
    pub random: bool,
}

impl Default for PixelLimPlugin {
    fn default() -> Self {
        Self {
            sprite_count: 32,
            random: false,
        }
    }
}

impl PixelLimPlugin {
    pub fn new(sprite_count: u32, random: bool) -> Self {
        Self {
            sprite_count,
            random,
        }
    }
}

impl Plugin for PixelLimPlugin {
    fn build(&self, app: &mut App) {
        if self.sprite_count != 0 {
            app.insert_resource(SpriteCount {
                count: self.sprite_count,
                random: self.random,
            })
            .add_system(sprite_count_limiter);
        }
    }
}
