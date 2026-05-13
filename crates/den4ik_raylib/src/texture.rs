use std::path::Path;

use crate::{
    RaylibError, RaylibHandle, Unloadable,
    container::{Container, ContainerId},
    image::ImageId,
};

pub struct Texture2D {
    pub(crate) inner: crate::ffi::Texture2D,
}

impl Unloadable for Texture2D {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadTexture(item.inner) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Texture2DId(usize);

impl ContainerId for Texture2DId {
    fn to_usize(self) -> usize {
        self.0
    }

    fn from_usize(value: usize) -> Self {
        Self(value)
    }
}

pub struct RenderTexture2D {
    pub(crate) inner: crate::ffi::RenderTexture2D,
}

impl Unloadable for RenderTexture2D {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadRenderTexture(item.inner) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderTextureId(usize);

impl ContainerId for RenderTextureId {
    fn to_usize(self) -> usize {
        self.0
    }

    fn from_usize(value: usize) -> Self {
        Self(value)
    }
}

impl RaylibHandle {
    pub fn load_render_texture(&mut self, width: u32, height: u32) -> RenderTextureId {
        let inner = unsafe {
            crate::ffi::LoadRenderTexture(width.try_into().unwrap(), height.try_into().unwrap())
        };
        self.render_textures.add(RenderTexture2D { inner })
    }

    pub fn is_render_texture_valid(&self, id: RenderTextureId) -> Result<bool, RaylibError> {
        let texture = self
            .render_textures
            .get(id)
            .ok_or(RaylibError::InvalidRenderTextureId)?;
        Ok(unsafe { crate::ffi::IsRenderTextureValid(texture.inner) })
    }

    pub fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> Result<Texture2DId, RaylibError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| RaylibError::PathNulError)?;
        let inner = unsafe { crate::ffi::LoadTexture(path_c.as_ptr()) };
        Ok(self.textures.add(Texture2D { inner }))
    }

    pub fn load_texture_from_image(
        &mut self,
        image_id: ImageId,
    ) -> Result<Texture2DId, RaylibError> {
        let image = self
            .images
            .get(image_id)
            .ok_or(RaylibError::InvalidImageId)?;
        let inner = unsafe { crate::ffi::LoadTextureFromImage(image.inner) };
        Ok(self.textures.add(Texture2D { inner }))
    }

    pub fn load_texture_cubemap(
        &mut self,
        image_id: ImageId,
        layout: i32,
    ) -> Result<Texture2DId, RaylibError> {
        let image = self
            .images
            .get(image_id)
            .ok_or(RaylibError::InvalidImageId)?;
        let inner = unsafe { crate::ffi::LoadTextureCubemap(image.inner, layout) };
        Ok(self.textures.add(Texture2D { inner }))
    }

    pub fn is_texture_valid(&self, id: Texture2DId) -> Result<bool, RaylibError> {
        let texture = self
            .textures
            .get(id)
            .ok_or_else(|| RaylibError::InvalidTexture2DId)?;
        Ok(unsafe { crate::ffi::IsTextureValid(texture.inner) })
    }

    pub fn update_texture(&mut self, id: Texture2DId, pixels: &[u8]) -> Result<(), RaylibError> {
        let texture = self
            .textures
            .get_mut(id)
            .ok_or_else(|| RaylibError::InvalidTexture2DId)?;
        unsafe {
            crate::ffi::UpdateTexture(texture.inner, pixels.as_ptr() as *const std::ffi::c_void)
        }
        Ok(())
    }

    pub fn update_texture_rec(
        &mut self,
        id: Texture2DId,
        rec: crate::ffi::Rectangle,
        pixels: &[u8],
    ) -> Result<(), RaylibError> {
        let texture = self
            .textures
            .get_mut(id)
            .ok_or_else(|| RaylibError::InvalidTexture2DId)?;
        unsafe {
            crate::ffi::UpdateTextureRec(
                texture.inner,
                rec,
                pixels.as_ptr() as *const std::ffi::c_void,
            )
        }
        Ok(())
    }
}
