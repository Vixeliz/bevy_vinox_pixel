use bevy::prelude::*;

use super::system::{add_cursor, update_cursor, update_world_cursor, WorldCursorPostion};

/// A plugin for pixel cursors. You must provide an image. Right now a cursor.png in your assets folder. Soon will implement a handle system. Will also add more states such as when you are about to click.
pub struct PixelCursorPlugin;

impl Plugin for PixelCursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCursorPostion::default())
            .add_systems((update_cursor, update_world_cursor, add_cursor));
    }
}
