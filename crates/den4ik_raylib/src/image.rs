use crate::Container;
use std::path::Path;

pub enum ImageError {
    PathContainsNullByte,
    ExportToMemoryFailed,
}

pub struct Image {
    inner: crate::ffi::Image,
}

impl Image {
    pub fn load<P: AsRef<Path>>(
        handle: &mut crate::RaylibHandle,
        path: P,
    ) -> Result<usize, ImageError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let inner = unsafe { crate::ffi::LoadImage(path_c.as_ptr()) };
        Ok(handle.add(Self { inner }))
    }

    pub fn load_raw<P: AsRef<Path>>(
        handle: &mut crate::RaylibHandle,
        path: P,
        width: u32,
        height: u32,
        format: i32,
        header_size: u32,
    ) -> Result<usize, ImageError> {
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
        Ok(handle.add(Self { inner }))
    }

    pub fn load_anim<P: AsRef<Path>>(
        handle: &mut crate::RaylibHandle,
        path: P,
        frames: &mut u32,
    ) -> Result<usize, ImageError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let mut frames_i32: i32 = 0;
        let inner = unsafe { crate::ffi::LoadImageAnim(path_c.as_ptr(), &mut frames_i32) };
        *frames = frames_i32.try_into().unwrap();
        Ok(handle.add(Self { inner }))
    }

    pub fn load_anim_from_memory(
        handle: &mut crate::RaylibHandle,
        file_type: &str,
        file_data: &[u8],
        frames: &mut u32,
    ) -> Result<usize, ImageError> {
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
        Ok(handle.add(Self { inner }))
    }

    pub fn load_from_memory(
        handle: &mut crate::RaylibHandle,
        file_type: &str,
        file_data: &[u8],
    ) -> Result<usize, ImageError> {
        let file_type_c =
            std::ffi::CString::new(file_type).map_err(|_| ImageError::PathContainsNullByte)?;
        let inner = unsafe {
            crate::ffi::LoadImageFromMemory(
                file_type_c.as_ptr(),
                file_data.as_ptr(),
                file_data.len().try_into().unwrap(),
            )
        };
        Ok(handle.add(Self { inner }))
    }

    pub fn load_from_texture(
        handle: &mut crate::RaylibHandle,
        texture: crate::ffi::Texture2D,
    ) -> usize {
        let inner = unsafe { crate::ffi::LoadImageFromTexture(texture) };
        handle.add(Self { inner })
    }

    pub fn load_from_screen(handle: &mut crate::RaylibHandle) -> usize {
        let inner = unsafe { crate::ffi::LoadImageFromScreen() };
        handle.add(Self { inner })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { crate::ffi::IsImageValid(self.inner) }
    }

    pub fn export<P: AsRef<Path>>(&self, path: P) -> Result<bool, ImageError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let res = unsafe { crate::ffi::ExportImage(self.inner, path_c.as_ptr()) };
        Ok(res)
    }

    pub fn export_to_memory(&self, file_type: &str) -> Result<Vec<u8>, ImageError> {
        let file_type_c =
            std::ffi::CString::new(file_type).map_err(|_| ImageError::PathContainsNullByte)?;
        let mut file_size: i32 = 0;
        let ptr = unsafe {
            crate::ffi::ExportImageToMemory(self.inner, file_type_c.as_ptr(), &mut file_size)
        };
        if ptr.is_null() {
            return Err(ImageError::ExportToMemoryFailed);
        }
        let slice = unsafe { std::slice::from_raw_parts(ptr, file_size.try_into().unwrap()) };
        let vec = slice.to_vec();
        unsafe { crate::ffi::MemFree(ptr as *mut std::ffi::c_void) };
        Ok(vec)
    }

    pub fn export_as_code<P: AsRef<Path>>(&self, path: P) -> Result<bool, ImageError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ImageError::PathContainsNullByte)?;
        let res = unsafe { crate::ffi::ExportImageAsCode(self.inner, path_c.as_ptr()) };
        Ok(res)
    }

    pub fn gen_color(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        color: impl crate::color::ToRlColor,
    ) -> usize {
        let inner = unsafe {
            crate::ffi::GenImageColor(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                color.to_rl_color(),
            )
        };
        handle.add(Self { inner })
    }

    pub fn gen_gradient_linear(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        direction: i32,
        start: impl crate::color::ToRlColor,
        end: impl crate::color::ToRlColor,
    ) -> usize {
        let inner = unsafe {
            crate::ffi::GenImageGradientLinear(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                direction,
                start.to_rl_color(),
                end.to_rl_color(),
            )
        };
        handle.add(Self { inner })
    }

    pub fn gen_gradient_radial(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        density: f32,
        inner_color: impl crate::color::ToRlColor,
        outer_color: impl crate::color::ToRlColor,
    ) -> usize {
        let inner = unsafe {
            crate::ffi::GenImageGradientRadial(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                density,
                inner_color.to_rl_color(),
                outer_color.to_rl_color(),
            )
        };
        handle.add(Self { inner })
    }

    pub fn gen_gradient_square(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        density: f32,
        inner_color: impl crate::color::ToRlColor,
        outer_color: impl crate::color::ToRlColor,
    ) -> usize {
        let inner = unsafe {
            crate::ffi::GenImageGradientSquare(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                density,
                inner_color.to_rl_color(),
                outer_color.to_rl_color(),
            )
        };
        handle.add(Self { inner })
    }

    pub fn gen_checked(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        checks_x: u32,
        checks_y: u32,
        col1: impl crate::color::ToRlColor,
        col2: impl crate::color::ToRlColor,
    ) -> usize {
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
        handle.add(Self { inner })
    }

    pub fn gen_white_noise(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        factor: f32,
    ) -> usize {
        let inner = unsafe {
            crate::ffi::GenImageWhiteNoise(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                factor,
            )
        };
        handle.add(Self { inner })
    }

    pub fn gen_perlin_noise(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> usize {
        let inner = unsafe {
            crate::ffi::GenImagePerlinNoise(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                offset_x,
                offset_y,
                scale,
            )
        };
        handle.add(Self { inner })
    }

    pub fn gen_cellular(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        tile_size: u32,
    ) -> usize {
        let inner = unsafe {
            crate::ffi::GenImageCellular(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                tile_size.try_into().unwrap(),
            )
        };
        handle.add(Self { inner })
    }

    pub fn gen_text(
        handle: &mut crate::RaylibHandle,
        width: u32,
        height: u32,
        text: &str,
    ) -> Result<usize, ImageError> {
        let text_c = std::ffi::CString::new(text).map_err(|_| ImageError::PathContainsNullByte)?;
        let inner = unsafe {
            crate::ffi::GenImageText(
                width.try_into().unwrap(),
                height.try_into().unwrap(),
                text_c.as_ptr(),
            )
        };
        Ok(handle.add(Self { inner }))
    }
}

impl crate::Unloadable for Image {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadImage(item.inner) }
    }
}

impl Container<Image> for crate::RaylibHandle {
    fn add(&mut self, item: Image) -> usize {
        self.images.add(item)
    }

    fn remove(&mut self, id: usize) -> bool {
        self.images.remove(id)
    }

    unsafe fn take(&mut self, id: usize) -> Option<Image> {
        unsafe { self.images.take(id) }
    }

    fn get(&self, id: usize) -> Option<&Image> {
        self.images.get(id)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut Image> {
        self.images.get_mut(id)
    }
}
