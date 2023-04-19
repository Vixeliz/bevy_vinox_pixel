pub mod modern;
pub mod plugin;
pub mod scaled;
pub mod texture;

pub mod prelude {
    pub use crate::plugin::PixelCameraTag;
    pub use crate::plugin::PixelPlugin;
    pub use crate::scaled::ScaledPixelCamera;
    pub use crate::texture::TexturePixelCamera;
}
