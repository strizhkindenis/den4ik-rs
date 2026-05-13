use std::path::Path;

use crate::{
	container::{Container, ContainerId},
	image::ImageId,
	RaylibHandle, RaylibError, Unloadable
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

impl RenderTexture2D {
    pub fn load(handle: &mut RaylibHandle, width: u32, height: u32) -> RenderTextureId {
        let inner = unsafe {
            crate::ffi::LoadRenderTexture(width.try_into().unwrap(), height.try_into().unwrap())
        };
        handle.add(Self { inner })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { crate::ffi::IsRenderTextureValid(self.inner) }
    }
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
	load_texture
}

impl Texture2D {
    pub fn load<P: AsRef<Path>>(
        handle: &mut RaylibHandle,
        path: P,
    ) -> Result<Texture2DId, RaylibError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| RaylibError::PathNulError)?;
        let inner = unsafe { crate::ffi::LoadTexture(path_c.as_ptr()) };
        Ok(handle.add(Self { inner }))
    }

    pub fn load_from_image(
        handle: &mut RaylibHandle,
        image_id: ImageId,
    ) -> Option<Texture2DId> {
        let image = handle.images.get(image_id)?;
        let inner = unsafe { crate::ffi::LoadTextureFromImage(image.inner) };
        Some(handle.add(Self { inner }))
    }

    pub fn load_cubemap(
        handle: &mut RaylibHandle,
        image_id: ImageId,
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
