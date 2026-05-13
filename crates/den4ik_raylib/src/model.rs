use std::{num::NonZeroUsize, path::Path};

use crate::container::{Container, ContainerId};



pub struct Model {
    inner: crate::ffi::Model,
}

impl crate::RaylibHandle {
    pub fn load_model<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ModelId, crate::RaylibError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| crate::RaylibError::PathNulError)?;
        let inner = unsafe { crate::ffi::LoadModel(path_c.as_ptr()) };
        if inner.meshCount > 0 {
            Ok(self.models.add(Model { inner }))
        } else {
            Err(crate::RaylibError::EmptyModel)
        }
    }

    pub fn load_model_from_mesh(
        &mut self,
        mesh_id: crate::mesh::MeshId,
    ) -> Result<ModelId, crate::RaylibError> {
        let mesh: crate::mesh::Mesh =
            unsafe { self.meshes.take(mesh_id).ok_or(crate::RaylibError::EmptyModel)? };
        let inner = unsafe { crate::ffi::LoadModelFromMesh(mesh.into_inner()) };
        if inner.meshCount > 0 {
            Ok(self.models.add(Model { inner }))
        } else {
            Err(crate::RaylibError::EmptyModel)
        }
    }

    pub fn draw_model(
        &self,
        id: ModelId,
        _draw_3d_handle: &crate::Draw3DHandle,
        position: crate::math::Vector3,
        scale: f32,
        tint: impl crate::color::ToRlColor,
    ) -> Result<(), crate::RaylibError> {
        let model = self.models.get(id).ok_or(crate::RaylibError::InvalidModelId)?;
        unsafe { crate::ffi::DrawModel(model.inner, position.into(), scale, tint.to_rl_color()) }
        Ok(())
    }

    pub fn get_model_mesh_count(&self, id: ModelId) -> Result<NonZeroUsize, crate::RaylibError> {
        let model = self.models.get(id).ok_or(crate::RaylibError::InvalidModelId)?;
        let count = usize::try_from(model.inner.meshCount).unwrap();
        Ok(NonZeroUsize::new(count).unwrap())
    }

    pub fn get_model_material_count(&self, id: ModelId) -> Result<NonZeroUsize, crate::RaylibError> {
        let model = self.models.get(id).ok_or(crate::RaylibError::InvalidModelId)?;
        let count = usize::try_from(model.inner.materialCount).unwrap();
        Ok(NonZeroUsize::new(count).unwrap())
    }

    pub fn get_model_meshes(&self, id: ModelId) -> Result<&[crate::mesh::Mesh], crate::RaylibError> {
        let count = self.get_model_mesh_count(id)?.get();
        let model = self.models.get(id).ok_or(crate::RaylibError::InvalidModelId)?;
        Ok(unsafe { std::slice::from_raw_parts(model.inner.meshes.cast(), count) })
    }

    pub fn get_model_meshes_mut(&mut self, id: ModelId) -> Result<&mut [crate::mesh::Mesh], crate::RaylibError> {
        let count = self.get_model_mesh_count(id)?.get();
        let model = self.models.get_mut(id).ok_or(crate::RaylibError::InvalidModelId)?;
        Ok(unsafe {
            std::slice::from_raw_parts_mut(model.inner.meshes.cast(), count)
        })
    }

    pub fn get_model_materials(&self, id: ModelId) -> Result<&[crate::material::Material], crate::RaylibError> {
        let count = self.get_model_material_count(id)?.get();
        let model = self.models.get(id).ok_or(crate::RaylibError::InvalidModelId)?;
        Ok(unsafe {
            std::slice::from_raw_parts(model.inner.materials.cast(), count)
        })
    }

    pub fn get_model_materials_mut(&mut self, id: ModelId) -> Result<&mut [crate::material::Material], crate::RaylibError> {
        let count = self.get_model_material_count(id)?.get();
        let model = self.models.get_mut(id).ok_or(crate::RaylibError::InvalidModelId)?;
        Ok(unsafe {
            std::slice::from_raw_parts_mut(
                model.inner.materials.cast(),
                count,
            )
        })
    }

    pub fn get_model_mesh_material(&self, id: ModelId, mesh_idx: usize) -> Result<usize, crate::RaylibError> {
        let count = self.get_model_mesh_count(id)?.get();
        assert!(mesh_idx < count);
        let model = self.models.get(id).ok_or(crate::RaylibError::InvalidModelId)?;
        let idx = unsafe { model.inner.meshMaterial.add(mesh_idx).read() };
        Ok(idx.try_into().unwrap())
    }

    pub fn set_model_mesh_material(&mut self, id: ModelId, mesh_idx: usize, material_idx: usize) -> Result<(), crate::RaylibError> {
        let mesh_count = self.get_model_mesh_count(id)?.get();
        let material_count = self.get_model_material_count(id)?.get();
        assert!(mesh_idx < mesh_count);
        assert!(material_idx < material_count);
        let material_idx = material_idx.try_into().unwrap();
        let model = self.models.get_mut(id).ok_or(crate::RaylibError::InvalidModelId)?;
        unsafe { model.inner.meshMaterial.add(mesh_idx).write(material_idx) };
        Ok(())
    }
}

impl crate::Unloadable for Model {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadModel(item.inner) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModelId(usize);

impl ContainerId for ModelId {
    fn to_usize(self) -> usize {
        self.0
    }

    fn from_usize(value: usize) -> Self {
        Self(value)
    }
}

impl Container<Model, ModelId> for crate::RaylibHandle {
    fn add(&mut self, item: Model) -> ModelId {
        self.models.add(item)
    }

    fn remove(&mut self, id: ModelId) -> bool {
        self.models.remove(id)
    }

    fn take(&mut self, id: ModelId) -> Option<Model> {
        unsafe { self.models.take(id) }
    }

    fn get(&self, id: ModelId) -> Option<&Model> {
        self.models.get(id)
    }

    fn get_mut(&mut self, id: ModelId) -> Option<&mut Model> {
        self.models.get_mut(id)
    }
}
