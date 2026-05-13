mod ffi;

mod allocator;
mod container;

pub mod color;
pub mod core;
pub mod draw;
pub mod image;
pub mod material;
pub mod math;
pub mod mesh;
pub mod model;
pub mod texture;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::core::*;
    pub use crate::draw::*;
    pub use crate::image::ImageId;
    pub use crate::material::{MaterialId, MaterialMap};
    pub use crate::math::*;
    pub use crate::mesh::MeshId;
    pub use crate::model::ModelId;
    pub use crate::texture::{RenderTextureId, Texture2DId};
    pub use crate::{RaylibError, RaylibHandle};
}

use container::VecConainer;

pub trait Unloadable {
    fn unload(item: Self);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaylibError {
    PathNulError,
    InvalidId(&'static str),
    InvalidIndex,
    ExportToMemoryFailed,
    EmptyModel,
    TrianglePointMiscount,
    AllocatorOOM,
    AllocatorSizeTooBig,
    AllocatorZeroSize,
}

pub struct RaylibHandle {
    pub(crate) materials: VecConainer<material::Material, material::MaterialId>,
    pub(crate) meshes: VecConainer<mesh::Mesh, mesh::MeshId>,
    pub(crate) models: VecConainer<model::Model, model::ModelId>,
    pub(crate) images: VecConainer<image::Image, image::ImageId>,
    pub(crate) textures: VecConainer<texture::Texture2D, texture::Texture2DId>,
    pub(crate) render_textures: VecConainer<texture::RenderTexture2D, texture::RenderTextureId>,
}

impl RaylibHandle {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        let title_c = std::ffi::CString::new(title).unwrap();
        unsafe {
            crate::ffi::InitWindow(width, height, title_c.as_ptr());
        }
        Self {
            materials: VecConainer::new(),
            meshes: VecConainer::new(),
            models: VecConainer::new(),
            images: VecConainer::new(),
            textures: VecConainer::new(),
            render_textures: VecConainer::new(),
        }
    }

    pub fn window_should_close(&self) -> bool {
        unsafe { crate::ffi::WindowShouldClose() }
    }

    pub fn set_target_fps(&mut self, fps: i32) {
        unsafe { crate::ffi::SetTargetFPS(fps) }
    }

    // Input & Camera - Enforcing Thread Safety
    pub fn is_mouse_button_pressed(&self, button: crate::core::MouseButton) -> bool {
        unsafe { crate::ffi::IsMouseButtonPressed(button.into()) }
    }

    pub fn is_key_pressed(&self, key: crate::core::KeyboardKey) -> bool {
        unsafe { crate::ffi::IsKeyPressed(key.into()) }
    }

    pub fn update_camera(&self, camera: &mut crate::core::Camera, mode: crate::core::CameraMode) {
        unsafe { crate::ffi::UpdateCamera(&mut camera.inner as *mut _, mode.into()) }
    }
}

impl Drop for RaylibHandle {
    fn drop(&mut self) {
        std::mem::swap(&mut self.render_textures, &mut VecConainer::new());
        std::mem::swap(&mut self.textures, &mut VecConainer::new());
        std::mem::swap(&mut self.images, &mut VecConainer::new());
        std::mem::swap(&mut self.models, &mut VecConainer::new());
        std::mem::swap(&mut self.meshes, &mut VecConainer::new());
        std::mem::swap(&mut self.materials, &mut VecConainer::new());
        unsafe { crate::ffi::CloseWindow() }
    }
}
