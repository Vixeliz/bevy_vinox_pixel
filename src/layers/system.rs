use bevy::prelude::*;

/// A way to sort sprites based off of layer. The number you use inside of the enums also affect ordering
// Tooken from https://github.com/bevyengine/bevy/issues/1275#issuecomment-1079079386. Until bevy offers something more flexible
#[derive(Component, Clone, Copy)]
pub enum PixelLayer {
    Background(u8),
    Foreground(u8),
}

pub fn update_z_coordinate_based_on_layer(
    mut query: Query<(&mut Transform, &PixelLayer), Changed<PixelLayer>>,
) {
    for (mut transform, layer) in query.iter_mut() {
        transform.translation.z = match layer {
            PixelLayer::Background(order_in_layer) => {
                -(u8::MAX as i8) as f32 + *order_in_layer as f32 / 1000.
            }
            PixelLayer::Foreground(order_in_layer) => 2. + *order_in_layer as f32 / 1000.,
        }
    }
}
