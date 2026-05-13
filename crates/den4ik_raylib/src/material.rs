use std::{path::Path, slice};

use crate::{
    RaylibError, RaylibHandle,
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
        self.materials.add(Material { inner })
    }

    pub fn load_materials<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<Vec<MaterialId>, RaylibError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| RaylibError::PathNulError)?;
        let mut count: i32 = 0;
        let materials_ptr = unsafe { crate::ffi::LoadMaterials(path_c.as_ptr(), &mut count) };
        let ids = unsafe { slice::from_raw_parts(materials_ptr, count as usize) }
            .iter()
            .map(|&inner| self.materials.add(Material { inner }))
            .collect::<Vec<_>>();
        unsafe { crate::ffi::MemFree(materials_ptr.cast()) };
        Ok(ids)
    }

    pub fn is_material_valid(&self, id: MaterialId) -> Result<bool, RaylibError> {
        let material = self
            .materials
            .get(id)
            .ok_or_else(|| RaylibError::InvalidMaterialId)?;
        let is_valid = unsafe { crate::ffi::IsMaterialValid(material.inner) };
        Ok(is_valid)
    }

    pub fn set_material_texture(
        &mut self,
        id: MaterialId,
        map: MaterialMap,
        texture_id: Texture2DId,
    ) -> Result<(), RaylibError> {
        let texture_inner = self
            .textures
            .get(texture_id)
            .ok_or_else(|| RaylibError::InvalidTexture2DId)?
            .inner;
        let material = self
            .materials
            .get_mut(id)
            .ok_or_else(|| RaylibError::InvalidMaterialId)?;
        let map_type = match map {
            MaterialMap::DIFFUSE => crate::ffi::MaterialMapIndex_MATERIAL_MAP_ALBEDO,
            MaterialMap::ALBEDO => crate::ffi::MaterialMapIndex_MATERIAL_MAP_ALBEDO,
            MaterialMap::BRDF => crate::ffi::MaterialMapIndex_MATERIAL_MAP_BRDF,
            MaterialMap::CUBEMAP => crate::ffi::MaterialMapIndex_MATERIAL_MAP_CUBEMAP,
            MaterialMap::EMISSION => crate::ffi::MaterialMapIndex_MATERIAL_MAP_EMISSION,
            MaterialMap::HEIGHT => crate::ffi::MaterialMapIndex_MATERIAL_MAP_HEIGHT,
            MaterialMap::IRRADIANCE => crate::ffi::MaterialMapIndex_MATERIAL_MAP_IRRADIANCE,
            MaterialMap::METALNESS => crate::ffi::MaterialMapIndex_MATERIAL_MAP_METALNESS,
            MaterialMap::SPECULAR => crate::ffi::MaterialMapIndex_MATERIAL_MAP_METALNESS,
            MaterialMap::NORMAL => crate::ffi::MaterialMapIndex_MATERIAL_MAP_NORMAL,
            MaterialMap::OCCLUSION => crate::ffi::MaterialMapIndex_MATERIAL_MAP_OCCLUSION,
            MaterialMap::PREFILTER => crate::ffi::MaterialMapIndex_MATERIAL_MAP_PREFILTER,
            MaterialMap::ROUGHNESS => crate::ffi::MaterialMapIndex_MATERIAL_MAP_ROUGHNESS,
        };
        unsafe {
            crate::ffi::SetMaterialTexture(&mut material.inner, map_type as i32, texture_inner)
        }
        Ok(())
    }
}
