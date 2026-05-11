mod ffi;

pub mod allocator;
pub mod color;
pub mod container;
pub mod core;
pub mod image;
pub mod material;
pub mod mesh;
pub mod model;
pub mod texture;

pub(crate) trait Unloadable {
    fn unload(item: Self);
}

use container::{ContainerId, VecConainer};

pub struct RaylibHandle {
    pub(crate) materials: VecConainer<material::Material, material::MaterialId>,
    pub(crate) meshes: VecConainer<mesh::Mesh, mesh::MeshId>,
    pub(crate) models: VecConainer<model::Model, model::ModelId>,
    pub(crate) images: VecConainer<image::Image, image::ImageId>,
    pub(crate) textures: VecConainer<texture::Texture2D, texture::TextureId>,
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
}

impl Drop for RaylibHandle {
    fn drop(&mut self) {
        // Unload all GPU/CPU resources before closing the window
        macro_rules! drain_container {
            ($field:expr) => {
                std::mem::swap(&mut $field, &mut VecConainer::new());
            };
        }
        drain_container!(self.render_textures);
        drain_container!(self.textures);
        drain_container!(self.images);
        drain_container!(self.models);
        drain_container!(self.meshes);
        drain_container!(self.materials);
        unsafe { crate::ffi::CloseWindow() }
    }
}
