use crate::container::{Container, ContainerId};
use std::path::Path;

pub enum TextureError {
    PathContainsNullByte,
}

pub struct Texture2D {
    pub(crate) inner: crate::ffi::Texture2D,
}

impl Texture2D {
    pub fn load<P: AsRef<Path>>(
        handle: &mut crate::RaylibHandle,
        path: P,
    ) -> Result<Texture2DId, TextureError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| TextureError::PathContainsNullByte)?;
        let inner = unsafe { crate::ffi::LoadTexture(path_c.as_ptr()) };
        Ok(handle.add(Self { inner }))
    }

    pub fn load_from_image(
        handle: &mut crate::RaylibHandle,
        image_id: crate::image::ImageId,
    ) -> Option<Texture2DId> {
        let image = handle.images.get(image_id)?;
        let inner = unsafe { crate::ffi::LoadTextureFromImage(image.inner) };
        Some(handle.add(Self { inner }))
    }

    pub fn load_cubemap(
        handle: &mut crate::RaylibHandle,
        image_id: crate::image::ImageId,
        layout: i32,
    ) -> Option<Texture2DId> {
        let image = handle.images.get(image_id)?;
        let inner = unsafe { crate::ffi::LoadTextureCubemap(image.inner, layout) };
        Some(handle.add(Self { inner }))
    }

    pub fn is_valid(&self) -> bool {
        unsafe { crate::ffi::IsTextureValid(self.inner) }
    }

    pub fn update(&mut self, pixels: &[u8]) {
        unsafe { crate::ffi::UpdateTexture(self.inner, pixels.as_ptr() as *const std::ffi::c_void) }
    }

    pub fn update_rec(&mut self, rec: crate::ffi::Rectangle, pixels: &[u8]) {
        unsafe {
            crate::ffi::UpdateTextureRec(
                self.inner,
                rec,
                pixels.as_ptr() as *const std::ffi::c_void,
            )
        }
    }
}

impl crate::Unloadable for Texture2D {
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

impl Container<Texture2D, Texture2DId> for crate::RaylibHandle {
    fn add(&mut self, item: Texture2D) -> Texture2DId {
        self.textures.add(item)
    }

    fn remove(&mut self, id: Texture2DId) -> bool {
        self.textures.remove(id)
    }

    unsafe fn take(&mut self, id: Texture2DId) -> Option<Texture2D> {
        unsafe { self.textures.take(id) }
    }

    fn get(&self, id: Texture2DId) -> Option<&Texture2D> {
        self.textures.get(id)
    }

    fn get_mut(&mut self, id: Texture2DId) -> Option<&mut Texture2D> {
        self.textures.get_mut(id)
    }
}

pub struct RenderTexture2D {
    pub(crate) inner: crate::ffi::RenderTexture2D,
}

impl RenderTexture2D {
    pub fn load(handle: &mut crate::RaylibHandle, width: u32, height: u32) -> RenderTextureId {
        let inner = unsafe {
            crate::ffi::LoadRenderTexture(width.try_into().unwrap(), height.try_into().unwrap())
        };
        handle.add(Self { inner })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { crate::ffi::IsRenderTextureValid(self.inner) }
    }
}

impl crate::Unloadable for RenderTexture2D {
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

impl Container<RenderTexture2D, RenderTextureId> for crate::RaylibHandle {
    fn add(&mut self, item: RenderTexture2D) -> RenderTextureId {
        self.render_textures.add(item)
    }

    fn remove(&mut self, id: RenderTextureId) -> bool {
        self.render_textures.remove(id)
    }

    unsafe fn take(&mut self, id: RenderTextureId) -> Option<RenderTexture2D> {
        unsafe { self.render_textures.take(id) }
    }

    fn get(&self, id: RenderTextureId) -> Option<&RenderTexture2D> {
        self.render_textures.get(id)
    }

    fn get_mut(&mut self, id: RenderTextureId) -> Option<&mut RenderTexture2D> {
        self.render_textures.get_mut(id)
    }
}
