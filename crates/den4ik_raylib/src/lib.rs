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

use container::{Container, VecConainer};

pub struct Draw3DHandle<'l, 'h> {
    handle: &'l mut DrawHandle<'h>,
}

impl<'l, 'h> Draw3DHandle<'l, 'h> {
    pub fn get_handle(&self) -> &RaylibHandle {
        self.handle.get_handle()
    }

    pub fn get_handle_mut(&mut self) -> &mut RaylibHandle {
        self.handle.get_handle_mut()
    }
}

impl<'l, 'h> Drop for Draw3DHandle<'l, 'h> {
    fn drop(&mut self) {
        unsafe { crate::ffi::EndMode3D() }
    }
}

pub struct DrawHandle<'h> {
    handle: &'h mut RaylibHandle,
}

impl<'h> DrawHandle<'h> {
    pub fn get_handle(&self) -> &RaylibHandle {
        self.handle
    }

    pub fn get_handle_mut(&mut self) -> &mut RaylibHandle {
        self.handle
    }

    pub fn clear_background(&mut self, color: impl color::ToRlColor) {
        unsafe { crate::ffi::ClearBackground(color.to_rl_color()) }
    }

    pub fn begin_mode_3d<'l>(&'l mut self, camera: crate::ffi::Camera3D) -> Draw3DHandle<'l, 'h> {
        unsafe { crate::ffi::BeginMode3D(camera) }
        Draw3DHandle { handle: self }
    }

    pub fn begin_mode_3d_with<F, R>(&mut self, camera: crate::ffi::Camera3D, f: F) -> R
    where
        F: FnOnce(Draw3DHandle) -> R,
    {
        let handle = self.begin_mode_3d(camera);
        f(handle)
    }

    pub fn draw_grid(&mut self, slices: i32, spacing: f32) {
        unsafe { crate::ffi::DrawGrid(slices, spacing) }
    }

    pub fn draw_rectangle(
        &mut self,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        color: impl color::ToRlColor,
    ) {
        unsafe { crate::ffi::DrawRectangle(pos_x, pos_y, width, height, color.to_rl_color()) }
    }

    pub fn draw_rectangle_lines(
        &mut self,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        color: impl color::ToRlColor,
    ) {
        unsafe { crate::ffi::DrawRectangleLines(pos_x, pos_y, width, height, color.to_rl_color()) }
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        pos_x: i32,
        pos_y: i32,
        font_size: i32,
        color: impl color::ToRlColor,
    ) {
        let text_c = std::ffi::CString::new(text).unwrap();
        unsafe {
            crate::ffi::DrawText(
                text_c.as_ptr(),
                pos_x,
                pos_y,
                font_size,
                color.to_rl_color(),
            )
        }
    }
}

impl<'h> Drop for DrawHandle<'h> {
    fn drop(&mut self) {
        unsafe { crate::ffi::EndDrawing() };
    }
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

    pub fn begin_drawing(&mut self) -> DrawHandle {
        unsafe { crate::ffi::BeginDrawing() }
        DrawHandle { handle: self }
    }

    pub fn begin_drawing_with<F>(&mut self, f: F)
    where
        F: FnOnce(DrawHandle),
    {
        let handle = self.begin_drawing();
        f(handle);
    }

    /// Set a texture on a model material. Handles both borrows internally.
    pub fn set_model_material_texture(
        &mut self,
        model_id: model::ModelId,
        material_idx: usize,
        map_type: i32,
        texture_id: texture::Texture2DId,
    ) -> Option<()> {
        let tex_inner = self.textures.get(texture_id)?.inner;
        let model = self.models.get_mut(model_id)?;
        let mat = model.get_materials_mut().get_mut(material_idx)?;
        unsafe { crate::ffi::SetMaterialTexture(&mut mat.inner, map_type, tex_inner) }
        Some(())
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
