use crate::container::{Container, ContainerId};
use std::path::Path;

pub enum MaterialError {
    PathContainsNullByte,
}

pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_ALBEDO as MATERIAL_MAP_DIFFUSE;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_ALBEDO as MATERIAL_MAP_ALBEDO;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_BRDF as MATERIAL_MAP_BRDF;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_CUBEMAP as MATERIAL_MAP_CUBEMAP;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_EMISSION as MATERIAL_MAP_EMISSION;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_HEIGHT as MATERIAL_MAP_HEIGHT;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_IRRADIANCE as MATERIAL_MAP_IRRADIANCE;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_METALNESS as MATERIAL_MAP_METALNESS;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_NORMAL as MATERIAL_MAP_NORMAL;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_OCCLUSION as MATERIAL_MAP_OCCLUSION;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_PREFILTER as MATERIAL_MAP_PREFILTER;
pub use crate::ffi::MaterialMapIndex_MATERIAL_MAP_ROUGHNESS as MATERIAL_MAP_ROUGHNESS;

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

    pub fn set_texture(
        &mut self,
        handle: &crate::RaylibHandle,
        map_type: i32,
        texture_id: usize,
    ) -> Option<()> {
        let texture = handle.textures.get(texture_id)?;
        unsafe { crate::ffi::SetMaterialTexture(&mut self.inner, map_type, texture.inner) }
        Some(())
    }
}

impl crate::Unloadable for Material {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadMaterial(item.inner) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(usize);

impl ContainerId for MaterialId {
    fn to_usize(self) -> usize {
        self.0
    }

    fn from_usize(value: usize) -> Self {
        Self(value)
    }
}

impl Container<Material, MaterialId> for crate::RaylibHandle {
    fn add(&mut self, item: Material) -> MaterialId {
        self.materials.add(item)
    }

    fn remove(&mut self, id: MaterialId) -> bool {
        self.materials.remove(id)
    }

    unsafe fn take(&mut self, id: MaterialId) -> Option<Material> {
        unsafe { self.materials.take(id) }
    }

    fn get(&self, id: MaterialId) -> Option<&Material> {
        self.materials.get(id)
    }

    fn get_mut(&mut self, id: MaterialId) -> Option<&mut Material> {
        self.materials.get_mut(id)
    }
}
