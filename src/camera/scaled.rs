use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::render::camera::{Camera, CameraProjection, CameraRenderGraph, Viewport};
use bevy::render::primitives::Frustum;
use bevy::render::view::{RenderLayers, VisibleEntities};
use bevy::window::PrimaryWindow;

use crate::prelude::PixelCameraTag;

use super::plugin::{CursorCameraTag, UiCameraTag};

/// This is a camera that scaled up pixels and aligns them to a virtual grid. This is tooken from bevy_pixel_camera
/// The advantage of this camera is smoother scrolling, rotation, etc
#[derive(Bundle)]
pub struct ScaledPixelCamera {
    pub camera: Camera,
    pub camera_tag: PixelCameraTag,
    pub camera_render_graph: CameraRenderGraph,
    pub pixel_projection: ScaledPixelProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub camera_2d: Camera2d,
    pub camera_config: UiCameraConfig,
}

impl Default for ScaledPixelCamera {
    fn default() -> Self {
        Self::from_resolution(256, 224, false)
    }
}

impl ScaledPixelCamera {
    /// Create a component bundle for a camera with the specified projection.
    pub fn new(pixel_projection: ScaledPixelProjection) -> Self {
        let far = pixel_projection.far;
        let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);
        let view_projection =
            pixel_projection.get_projection_matrix() * transform.compute_matrix().inverse();
        let frustum = Frustum::from_view_projection_custom_far(
            &view_projection,
            &transform.translation,
            &transform.back(),
            pixel_projection.far(),
        );
        Self {
            camera_render_graph: CameraRenderGraph::new(bevy::core_pipeline::core_2d::graph::NAME),
            pixel_projection,
            visible_entities: Default::default(),
            frustum,
            transform,
            global_transform: Default::default(),
            camera: Camera::default(),
            camera_tag: PixelCameraTag,
            camera_2d: Camera2d::default(),
            camera_config: UiCameraConfig { show_ui: false },
        }
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// are specified with `zoom`.
    pub fn from_zoom(zoom: f32) -> Self {
        if zoom.round() == zoom {
            Self::new(ScaledPixelProjection {
                zoom,
                ..Default::default()
            })
        } else {
            Self::new(ScaledPixelProjection {
                zoom,
                imperfect: true,
                ..Default::default()
            })
        }
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified resolution inside the window.
    pub fn from_resolution(width: i32, height: i32, imperfect: bool) -> Self {
        Self::new(ScaledPixelProjection {
            desired_width: Some(width),
            desired_height: Some(height),
            imperfect,
            ..Default::default()
        })
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified width inside the window.
    pub fn from_width(width: i32, imperfect: bool) -> Self {
        Self::new(ScaledPixelProjection {
            desired_width: Some(width),
            imperfect,
            ..Default::default()
        })
    }

    /// Create a component bundle for a camera where the size of virtual pixels
    /// is automatically set to fit the specified height inside the window.
    pub fn from_height(height: i32, imperfect: bool) -> Self {
        Self::new(ScaledPixelProjection {
            desired_height: Some(height),
            imperfect,
            ..Default::default()
        })
    }
}

/// Component for a pixel-perfect orthographic projection.
///
/// It is similar to Bevy's OrthographicProjection, except integral world
/// coordinates are always aligned with virtual pixels (as defined by the zoom
/// field).
#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct ScaledPixelProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,

    /// If present, `zoom` will be automatically updated to always fit
    /// `desired_width` in the window as best as possible.
    pub desired_width: Option<i32>,

    /// If present, `zoom` will be automatically updated to always fit
    /// `desired_height` in the window as best as possible.
    pub desired_height: Option<i32>,

    /// If neither `desired_width` nor `desired_height` are present, zoom can be
    /// manually set. The value detemines the size of the virtual pixels.
    pub zoom: f32,

    /// If true, (0, 0) is the pixel closest to the center of the window,
    /// otherwise it's at bottom left.
    pub centered: bool,

    /// If true pixels don't have to be an integer value and can be instead a float
    pub imperfect: bool,

    pub init: bool,
}

impl CameraProjection for ScaledPixelProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            // NOTE: near and far are swapped to invert the depth range from [0,1] to [1,0]
            // This is for interoperability with pipelines using infinite reverse perspective projections.
            self.far,
            self.near,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        let mut zoom_x = None;
        if let Some(desired_width) = self.desired_width {
            if desired_width > 0 {
                zoom_x = Some(width / desired_width as f32);
            }
        }
        let mut zoom_y = None;
        if let Some(desired_height) = self.desired_height {
            if desired_height > 0 {
                zoom_y = Some(height / desired_height as f32);
            }
        }
        match (zoom_x, zoom_y) {
            (Some(zoom_x), Some(zoom_y)) => self.zoom = zoom_x.min(zoom_y).max(1.0),
            (Some(zoom_x), None) => self.zoom = zoom_x.max(1.0),
            (None, Some(zoom_y)) => self.zoom = zoom_y.max(1.0),
            (None, None) => (),
        }
        if !self.imperfect {
            self.zoom = self.zoom.round();
        }

        let actual_width = width / (self.zoom);
        let actual_height = height / (self.zoom);
        if self.centered {
            self.left = -((actual_width as i32) / 2) as f32;
            self.right = self.left + actual_width;
            self.bottom = -((actual_height as i32) / 2) as f32;
            self.top = self.bottom + actual_height;
        } else {
            self.left = 0.0;
            self.right = actual_width;
            self.bottom = 0.0;
            self.top = actual_height;
        }
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for ScaledPixelProjection {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            desired_width: None,
            desired_height: None,
            zoom: 1.0,
            centered: true,
            imperfect: false,
            init: false,
        }
    }
}

pub fn setup_camera(
    mut commands: Commands,
    mut camera: Query<(&mut ScaledPixelProjection, Entity)>,
) {
    if let Ok((mut projection, _entity)) = camera.get_single_mut() {
        if !projection.init {
            projection.init = true;
            let ui_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 2) as u8);
            commands.spawn((
                Camera2dBundle {
                    camera: Camera {
                        // renders after the camera that draws the texture
                        order: 2,
                        ..default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
                UiCameraTag,
                ui_layer,
            ));
            let cursor_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 3) as u8);
            commands.spawn((
                Camera2dBundle {
                    camera: Camera {
                        // renders after the camera that draws the texture
                        order: 3,
                        ..default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },
                    ..Default::default()
                },
                UiCameraConfig { show_ui: false },
                CursorCameraTag,
                cursor_layer,
            ));
        }
    }
}
pub fn update_scaled_viewport(
    mut camera_query: Query<(&mut Camera, &ScaledPixelProjection)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = windows.get_single_mut() {
        for (mut camera, projection) in camera_query.iter_mut() {
            let screen_width = projection.desired_width.map(|w| w as f32).unwrap_or(0.0);
            let screen_height = projection.desired_height.map(|h| h as f32).unwrap_or(0.0);
            let aspect_ratio = screen_width / screen_height;
            let window_size: UVec2 = if window.physical_height() > window.physical_width()
                || window.physical_height() as f32 * aspect_ratio > window.physical_width() as f32
            {
                UVec2::new(
                    window.physical_width(),
                    (window.physical_width() as f32 / aspect_ratio).floor() as u32,
                )
            } else {
                UVec2::new(
                    (window.physical_height() as f32 * aspect_ratio).floor() as u32,
                    window.physical_height(),
                )
            };

            let window_position: UVec2 = if window.physical_height() > window.physical_width()
                || window.physical_height() as f32 * aspect_ratio > window.physical_width() as f32
            {
                if let Some(height) = (window.physical_height() / 2).checked_sub(window_size.y / 2)
                {
                    UVec2::new(0, height)
                } else {
                    UVec2::ZERO
                }
            } else if let Some(width) = (window.physical_width() / 2).checked_sub(window_size.x / 2)
            {
                UVec2::new(width, 0)
            } else {
                UVec2::ZERO
            };

            if window_size.x != 0 && window_size.y != 0 {
                camera.viewport = Some(Viewport {
                    physical_size: window_size,
                    physical_position: window_position,
                    ..Default::default()
                });
            }
        }
    }
}
