use std::{path::Path, slice};

use crate::{
    RaylibHandle, RaylibError,
    container::{Container, ContainerId},
    texture::Texture2DId,
};

pub enum MaterialMap {
    DIFFUSE,
    ALBEDO,
    BRDF,
    CUBEMAP,
    EMISSION,
    HEIGHT,
    IRRADIANCE,
    METALNESS,
    SPECULAR,
    NORMAL,
    OCCLUSION,
    PREFILTER,
    ROUGHNESS,
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

pub struct Material {
    pub(crate) inner: crate::ffi::Material,
}

impl crate::Unloadable for Material {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadMaterial(item.inner) }
    }
}

impl RaylibHandle {
    pub fn load_material_default(&mut self) -> MaterialId {
        let inner = unsafe { crate::ffi::LoadMaterialDefault() };
        handle.add(Self { inner })
    }

    pub fn load_materials<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<MaterialId>, RaylibError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| RaylibError::PathNulError)?;
        let mut count: i32 = 0;
        let materials_ptr = unsafe { crate::ffi::LoadMaterials(path_c.as_ptr(), &mut count) };
        let ids = slice::from_raw_parts(materials_ptr, count as usize)
            .map(|inner| self.materials.add(Self { inner }))
            .collect::<Vec<_>>();
        unsafe { crate::ffi::MemFree(materials_ptr.cast()) };
        Ok(ids)
    }

    pub fn is_material_valid(&self, id: MaterialId) -> Resut<bool, RaylibError> {
        let material = self
            .material
            .get(id)
            .ok_or_else(|| RaylibError::InvalidMaterialId);
        let is_valid = unsafe { crate::ffi::IsMaterialValid(material.inner) };
        Ok(is_valid)
    }

    pub fn set_material_texture(
        &mut self,
        id: MaterialId,
        map: MaterialMap,
        texture_id: Texture2DId,
    ) -> Result<(), RaylibError> {
        let texture = self
            .textures
            .take(texture_id)
            .ok_or_else(|| RaylibError::InvalidTexture2DId);
        let material = self
            .materials
            .get_mut(id)
            .ok_or_else(|| RaylibError::InvalidMaterialId);
        let map_type = match map {
            DIFFUSE => crate::ffi::MaterialMapIndex_MATERIAL_MAP_ALBEDO,
            ALBEDO => crate::ffi::MaterialMapIndex_MATERIAL_MAP_ALBEDO,
            BRDF => crate::ffi::MaterialMapIndex_MATERIAL_MAP_BRDF,
            CUBEMAP => crate::ffi::MaterialMapIndex_MATERIAL_MAP_CUBEMAP,
            EMISSION => crate::ffi::MaterialMapIndex_MATERIAL_MAP_EMISSION,
            HEIGHT => crate::ffi::MaterialMapIndex_MATERIAL_MAP_HEIGHT,
            IRRADIANCE => crate::ffi::MaterialMapIndex_MATERIAL_MAP_IRRADIANCE,
            METALNESS => crate::ffi::MaterialMapIndex_MATERIAL_MAP_METALNESS,
            SPECULAR => crate::ffi::MaterialMapIndex_MATERIAL_MAP_METALNESS,
            NORMAL => crate::ffi::MaterialMapIndex_MATERIAL_MAP_NORMAL,
            OCCLUSION => crate::ffi::MaterialMapIndex_MATERIAL_MAP_OCCLUSION,
            PREFILTER => crate::ffi::MaterialMapIndex_MATERIAL_MAP_PREFILTER,
            ROUGNESS => crate::ffi::MaterialMapIndex_MATERIAL_MAP_ROUGHNESS,
        };
        unsafe { crate::ffi::SetMaterialTexture(&mut self.inner, map_type, texture.inner) }
        Ok(())
    }
}
