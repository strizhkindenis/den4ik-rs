use crate::Container;
use std::path::Path;

pub enum MaterialError {
    PathContainsNullByte,
}

#[repr(transparent)]
pub struct Material {
    inner: crate::ffi::Material,
}

impl Material {
    pub fn load_default(handle: &mut crate::RaylibHandle) -> usize {
        let inner = unsafe { crate::ffi::LoadMaterialDefault() };
        handle.add(Self { inner })
    }

    pub fn load_materials<P: AsRef<Path>>(
        handle: &mut crate::RaylibHandle,
        path: P,
    ) -> Result<Vec<usize>, MaterialError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| MaterialError::PathContainsNullByte)?;
        let mut count: i32 = 0;
        let materials_ptr = unsafe { crate::ffi::LoadMaterials(path_c.as_ptr(), &mut count) };
        let mut ids = Vec::with_capacity(count as usize);
        for i in 0..count {
            let inner = unsafe { *materials_ptr.add(i as usize) };
            ids.push(handle.add(Self { inner }));
        }
        unsafe { crate::ffi::MemFree(materials_ptr as *mut std::ffi::c_void) };
        Ok(ids)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { crate::ffi::IsMaterialValid(self.inner) }
    }

    pub fn set_texture(&mut self, map_type: i32, texture: crate::ffi::Texture2D) {
        unsafe { crate::ffi::SetMaterialTexture(&mut self.inner, map_type, texture) }
    }
}

impl crate::Unloadable for Material {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadMaterial(item.inner) }
    }
}

impl Container<Material> for crate::RaylibHandle {
    fn add(&mut self, item: Material) -> usize {
        self.materials.add(item)
    }

    fn remove(&mut self, id: usize) -> bool {
        self.materials.remove(id)
    }

    unsafe fn take(&mut self, id: usize) -> Option<Material> {
        unsafe { self.materials.take(id) }
    }

    fn get(&self, id: usize) -> Option<&Material> {
        self.materials.get(id)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut Material> {
        self.materials.get_mut(id)
    }
}
