use bevy::prelude::*;
use extol_sprite_layer::*;

/// A way to sort sprites based off of layer. The number you use inside of the enums also affect ordering
#[derive(Debug, Copy, Clone, Component, PartialEq, Eq, Hash)]
pub enum PixelLayer {
    Background(u8),
    Foreground(u8),
}

impl LayerIndex for PixelLayer {
    // Convert your type to an actual z-coordinate.
    fn as_z_coordinate(&self) -> f32 {
        use PixelLayer::*;
        match *self {
            // Note that the z-coordinates must be at least 1 apart...
            Background(val) => (val) as f32,
            Foreground(val) => u8::MAX as f32 + val as f32,
        }
    }
}
