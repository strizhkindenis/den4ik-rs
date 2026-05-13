use std::{num::NonZeroUsize, path::Path};

use crate::container::{Container, ContainerId};

#[derive(Debug)]
pub enum ModelError {
    EmptyModel,
    PathContainsNullByte,
}

pub struct Model {
    inner: crate::ffi::Model,
}

impl crate::RaylibHandle {
    pub fn load_model<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<ModelId, ModelError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ModelError::PathContainsNullByte)?;
        let inner = unsafe { crate::ffi::LoadModel(path_c.as_ptr()) };
        if inner.meshCount > 0 {
            Ok(self.models.add(Model { inner }))
        } else {
            Err(ModelError::EmptyModel)
        }
    }

    pub fn load_model_from_mesh(
        &mut self,
        mesh_id: crate::mesh::MeshId,
    ) -> Result<ModelId, ModelError> {
        let mesh: crate::mesh::Mesh =
            unsafe { self.meshes.take(mesh_id).ok_or(ModelError::EmptyModel)? };
        let inner = unsafe { crate::ffi::LoadModelFromMesh(mesh.into_inner()) };
        if inner.meshCount > 0 {
            Ok(self.models.add(Model { inner }))
        } else {
            Err(ModelError::EmptyModel)
        }
    }

    pub fn draw_model(
        &self,
        id: ModelId,
        _draw_3d_handle: &crate::Draw3DHandle,
        position: crate::math::Vector3,
        scale: f32,
        tint: impl crate::color::ToRlColor,
    ) -> Option<()> {
        let model = self.models.get(id)?;
        unsafe { crate::ffi::DrawModel(model.inner, position.into(), scale, tint.to_rl_color()) }
        Some(())
    }

    pub fn get_model_mesh_count(&self, id: ModelId) -> Option<NonZeroUsize> {
        let model = self.models.get(id)?;
        let count = usize::try_from(model.inner.meshCount).unwrap();
        NonZeroUsize::new(count)
    }

    pub fn get_model_material_count(&self, id: ModelId) -> Option<NonZeroUsize> {
        let model = self.models.get(id)?;
        let count = usize::try_from(model.inner.materialCount).unwrap();
        NonZeroUsize::new(count)
    }

    pub fn get_model_meshes(&self, id: ModelId) -> Option<&[crate::mesh::Mesh]> {
        let count = self.get_model_mesh_count(id)?.get();
        let model = self.models.get(id)?;
        Some(unsafe { std::slice::from_raw_parts(model.inner.meshes.cast(), count) })
    }

    pub fn get_model_meshes_mut(&mut self, id: ModelId) -> Option<&mut [crate::mesh::Mesh]> {
        let count = self.get_model_mesh_count(id)?.get();
        let model = self.models.get_mut(id)?;
        Some(unsafe {
            std::slice::from_raw_parts_mut(model.inner.meshes.cast(), count)
        })
    }

    pub fn get_model_materials(&self, id: ModelId) -> Option<&[crate::material::Material]> {
        let count = self.get_model_material_count(id)?.get();
        let model = self.models.get(id)?;
        Some(unsafe {
            std::slice::from_raw_parts(model.inner.materials.cast(), count)
        })
    }

    pub fn get_model_materials_mut(&mut self, id: ModelId) -> Option<&mut [crate::material::Material]> {
        let count = self.get_model_material_count(id)?.get();
        let model = self.models.get_mut(id)?;
        Some(unsafe {
            std::slice::from_raw_parts_mut(
                model.inner.materials.cast(),
                count,
            )
        })
    }

    pub fn get_model_mesh_material(&self, id: ModelId, mesh_idx: usize) -> Option<usize> {
        let count = self.get_model_mesh_count(id)?.get();
        assert!(mesh_idx < count);
        let model = self.models.get(id)?;
        let idx = unsafe { model.inner.meshMaterial.add(mesh_idx).read() };
        Some(idx.try_into().unwrap())
    }

    pub fn set_model_mesh_material(&mut self, id: ModelId, mesh_idx: usize, material_idx: usize) -> Option<()> {
        let mesh_count = self.get_model_mesh_count(id)?.get();
        let material_count = self.get_model_material_count(id)?.get();
        assert!(mesh_idx < mesh_count);
        assert!(material_idx < material_count);
        let material_idx = material_idx.try_into().unwrap();
        let model = self.models.get_mut(id)?;
        unsafe { model.inner.meshMaterial.add(mesh_idx).write(material_idx) };
        Some(())
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
