pub mod camera;
pub mod cursor;
pub mod plugin;

pub mod prelude {
    pub use crate::camera::plugin::PixelCameraPlugin;
    pub use crate::camera::plugin::PixelCameraTag;
    pub use crate::camera::scaled::ScaledPixelCamera;
    pub use crate::camera::texture::TexturePixelCamera;
    pub use crate::cursor::plugin::PixelCursorPlugin;
    pub use crate::plugin::PixelPlugins;
}
