use std::{path::Path, slice};

use crate::{
    RaylibError, RaylibHandle,
    container::{Container, ContainerId},
    texture::Texture2DId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialMap {
    Albedo,
    Metalness,
    Normal,
    Roughness,
    Occlusion,
    Emission,
    Height,
    Cubemap,
    Irradiance,
    Prefilter,
    Brdf,
    // Aliases
    Diffuse,
    Specular,
}

impl From<MaterialMap> for i32 {
    fn from(val: MaterialMap) -> Self {
        match val {
            MaterialMap::Albedo | MaterialMap::Diffuse => {
                crate::ffi::MaterialMapIndex_MATERIAL_MAP_ALBEDO as i32
            }
            MaterialMap::Metalness | MaterialMap::Specular => {
                crate::ffi::MaterialMapIndex_MATERIAL_MAP_METALNESS as i32
            }
            MaterialMap::Normal => crate::ffi::MaterialMapIndex_MATERIAL_MAP_NORMAL as i32,
            MaterialMap::Roughness => crate::ffi::MaterialMapIndex_MATERIAL_MAP_ROUGHNESS as i32,
            MaterialMap::Occlusion => crate::ffi::MaterialMapIndex_MATERIAL_MAP_OCCLUSION as i32,
            MaterialMap::Emission => crate::ffi::MaterialMapIndex_MATERIAL_MAP_EMISSION as i32,
            MaterialMap::Height => crate::ffi::MaterialMapIndex_MATERIAL_MAP_HEIGHT as i32,
            MaterialMap::Cubemap => crate::ffi::MaterialMapIndex_MATERIAL_MAP_CUBEMAP as i32,
            MaterialMap::Irradiance => crate::ffi::MaterialMapIndex_MATERIAL_MAP_IRRADIANCE as i32,
            MaterialMap::Prefilter => crate::ffi::MaterialMapIndex_MATERIAL_MAP_PREFILTER as i32,
            MaterialMap::Brdf => crate::ffi::MaterialMapIndex_MATERIAL_MAP_BRDF as i32,
        }
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

pub struct Material {
    pub(crate) inner: crate::ffi::Material,
}

impl crate::Unloadable for Material {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadMaterial(item.inner) }
    }
}

impl RaylibHandle {
    pub fn remove_material(&mut self, id: MaterialId) -> Result<(), crate::RaylibError> {
        self.materials.remove(id)
    }

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
        let material = self.materials.get(id)?;
        let is_valid = unsafe { crate::ffi::IsMaterialValid(material.inner) };
        Ok(is_valid)
    }

    pub fn set_material_texture(
        &mut self,
        id: MaterialId,
        map: MaterialMap,
        texture_id: Texture2DId,
    ) -> Result<(), RaylibError> {
        let texture_inner = self.textures.get(texture_id)?.inner;
        let material = self.materials.get_mut(id)?;
        unsafe { crate::ffi::SetMaterialTexture(&mut material.inner, map.into(), texture_inner) }
        Ok(())
    }
}
