use crate::container::{Container, ContainerId};
use std::path::Path;

#[derive(Debug)]
pub enum ImageError {
    PathContainsNullByte,
    ExportToMemoryFailed,
    InvalidImageId,
}

pub struct Image {
    pub(crate) inner: crate::ffi::Image,
}

impl crate::RaylibHandle {
    pub fn load_image<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ImageId, ImageError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let inner = unsafe { crate::ffi::LoadImage(path_c.as_ptr()) };
        Ok(self.images.add(Image { inner }))
    }

    pub fn load_image_raw<P: AsRef<Path>>(
        &mut self,
        path: P,
        width: u32,
        height: u32,
        format: i32,
        header_size: u32,
    ) -> Result<ImageId, ImageError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let inner = unsafe {
            crate::ffi::LoadImageRaw(
                path_c.as_ptr(),
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                format,
                header_size.try_into().unwrap(),
            )
        };
        Ok(self.images.add(Image { inner }))
    }

    pub fn load_image_anim<P: AsRef<Path>>(
        &mut self,
        path: P,
        frames: &mut u32,
    ) -> Result<ImageId, ImageError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let mut frames_i32: i32 = 0;
        let inner = unsafe { crate::ffi::LoadImageAnim(path_c.as_ptr(), &mut frames_i32) };
        *frames = frames_i32.try_into().unwrap();
        Ok(self.images.add(Image { inner }))
    }

    pub fn load_image_anim_from_memory(
        &mut self,
        file_type: &str,
        file_data: &[u8],
        frames: &mut u32,
    ) -> Result<ImageId, ImageError> {
        let file_type_c =
            std::ffi::CString::new(file_type).map_err(|_| ImageError::PathContainsNullByte)?;
        let mut frames_i32: i32 = 0;
        let inner = unsafe {
            crate::ffi::LoadImageAnimFromMemory(
                file_type_c.as_ptr(),
                file_data.as_ptr(),
                file_data.len().try_into().unwrap(),
                &mut frames_i32,
            )
        };
        *frames = frames_i32.try_into().unwrap();
        Ok(self.images.add(Image { inner }))
    }

    pub fn load_image_from_memory(
        &mut self,
        file_type: &str,
        file_data: &[u8],
    ) -> Result<ImageId, ImageError> {
        let file_type_c =
            std::ffi::CString::new(file_type).map_err(|_| ImageError::PathContainsNullByte)?;
        let inner = unsafe {
            crate::ffi::LoadImageFromMemory(
                file_type_c.as_ptr(),
                file_data.as_ptr(),
                file_data.len().try_into().unwrap(),
            )
        };
        Ok(self.images.add(Image { inner }))
    }

    pub fn load_image_from_texture(
        &mut self,
        texture_id: crate::texture::Texture2DId,
    ) -> Option<ImageId> {
        let texture = self.textures.get(texture_id)?;
        let inner = unsafe { crate::ffi::LoadImageFromTexture(texture.inner) };
        Some(self.images.add(Image { inner }))
    }

    pub fn load_image_from_screen(&mut self) -> ImageId {
        let inner = unsafe { crate::ffi::LoadImageFromScreen() };
        self.images.add(Image { inner })
    }

    pub fn is_image_valid(&self, id: ImageId) -> Option<bool> {
        let image = self.images.get(id)?;
        Some(unsafe { crate::ffi::IsImageValid(image.inner) })
    }

    pub fn export_image<P: AsRef<Path>>(&self, id: ImageId, path: P) -> Result<bool, ImageError> {
        let image = self.images.get(id).ok_or(ImageError::InvalidImageId)?;
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let res = unsafe { crate::ffi::ExportImage(image.inner, path_c.as_ptr()) };
        Ok(res)
    }

    pub fn export_image_to_memory(&self, id: ImageId, file_type: &str) -> Result<Vec<u8>, ImageError> {
        let image = self.images.get(id).ok_or(ImageError::InvalidImageId)?;
        let file_type_c =
            std::ffi::CString::new(file_type).map_err(|_| ImageError::PathContainsNullByte)?;
        let mut file_size: i32 = 0;
        let ptr = unsafe {
            crate::ffi::ExportImageToMemory(image.inner, file_type_c.as_ptr(), &mut file_size)
        };
        if ptr.is_null() {
            return Err(ImageError::ExportToMemoryFailed);
        }
        let slice = unsafe { std::slice::from_raw_parts(ptr, file_size.try_into().unwrap()) };
        let vec = slice.to_vec();
        unsafe { crate::ffi::MemFree(ptr as *mut std::ffi::c_void) };
        Ok(vec)
    }

    pub fn export_image_as_code<P: AsRef<Path>>(&self, id: ImageId, path: P) -> Result<bool, ImageError> {
        let image = self.images.get(id).ok_or(ImageError::InvalidImageId)?;
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let res = unsafe { crate::ffi::ExportImageAsCode(image.inner, path_c.as_ptr()) };
        Ok(res)
    }

    pub fn gen_image_color(
        &mut self,
        width: u32,
        height: u32,
        color: impl crate::color::ToRlColor,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImageColor(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                color.to_rl_color(),
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_gradient_linear(
        &mut self,
        width: u32,
        height: u32,
        direction: i32,
        start: impl crate::color::ToRlColor,
        end: impl crate::color::ToRlColor,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImageGradientLinear(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                direction,
                start.to_rl_color(),
                end.to_rl_color(),
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_gradient_radial(
        &mut self,
        width: u32,
        height: u32,
        density: f32,
        inner_color: impl crate::color::ToRlColor,
        outer_color: impl crate::color::ToRlColor,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImageGradientRadial(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                density,
                inner_color.to_rl_color(),
                outer_color.to_rl_color(),
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_gradient_square(
        &mut self,
        width: u32,
        height: u32,
        density: f32,
        inner_color: impl crate::color::ToRlColor,
        outer_color: impl crate::color::ToRlColor,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImageGradientSquare(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                density,
                inner_color.to_rl_color(),
                outer_color.to_rl_color(),
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_checked(
        &mut self,
        width: u32,
        height: u32,
        checks_x: u32,
        checks_y: u32,
        col1: impl crate::color::ToRlColor,
        col2: impl crate::color::ToRlColor,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImageChecked(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                checks_x.try_into().unwrap(),
                checks_y.try_into().unwrap(),
                col1.to_rl_color(),
                col2.to_rl_color(),
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_white_noise(
        &mut self,
        width: u32,
        height: u32,
        factor: f32,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImageWhiteNoise(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                factor,
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_perlin_noise(
        &mut self,
        width: u32,
        height: u32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImagePerlinNoise(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                offset_x,
                offset_y,
                scale,
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_cellular(
        &mut self,
        width: u32,
        height: u32,
        tile_size: u32,
    ) -> ImageId {
        let inner = unsafe {
            crate::ffi::GenImageCellular(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                tile_size.try_into().unwrap(),
            )
        };
        self.images.add(Image { inner })
    }

    pub fn gen_image_text(
        &mut self,
        width: u32,
        height: u32,
        text: &str,
    ) -> Result<ImageId, ImageError> {
        let text_c = std::ffi::CString::new(text).map_err(|_| ImageError::PathContainsNullByte)?;
        let inner = unsafe {
            crate::ffi::GenImageText(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                text_c.as_ptr(),
            )
        };
        Ok(self.images.add(Image { inner }))
    }
}

impl crate::Unloadable for Image {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadImage(item.inner) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(usize);

impl ContainerId for ImageId {
    fn to_usize(self) -> usize {
        self.0
    }

    fn from_usize(value: usize) -> Self {
        Self(value)
    }
}

impl Container<Image, ImageId> for crate::RaylibHandle {
    fn add(&mut self, item: Image) -> ImageId {
        self.images.add(item)
    }

    fn remove(&mut self, id: ImageId) -> bool {
        self.images.remove(id)
    }

    fn take(&mut self, id: ImageId) -> Option<Image> {
        unsafe { self.images.take(id) }
    }

    fn get(&self, id: ImageId) -> Option<&Image> {
        self.images.get(id)
    }

    fn get_mut(&mut self, id: ImageId) -> Option<&mut Image> {
        self.images.get_mut(id)
    }
}
