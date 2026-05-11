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

impl Model {
    pub fn load<P: AsRef<Path>>(
        handle: &mut crate::RaylibHandle,
        path: P,
    ) -> Result<ModelId, ModelError> {
        let path_c = std::ffi::CString::new(path.as_ref().to_string_lossy().as_ref())
            .map_err(|_| ModelError::PathContainsNullByte)?;
        let inner = unsafe { crate::ffi::LoadModel(path_c.as_ptr()) };
        if inner.meshCount > 0 {
            Ok(handle.add(Self { inner }))
        } else {
            Err(ModelError::EmptyModel)
        }
    }

    pub fn load_from_mesh(
        handle: &mut crate::RaylibHandle,
        mesh_id: crate::mesh::MeshId,
    ) -> Result<ModelId, ModelError> {
        let mesh: crate::mesh::Mesh =
            unsafe { handle.take(mesh_id).ok_or(ModelError::EmptyModel)? };
        let inner = unsafe { crate::ffi::LoadModelFromMesh(mesh.into_inner()) };
        if inner.meshCount > 0 {
            Ok(handle.add(Self { inner }))
        } else {
            Err(ModelError::EmptyModel)
        }
    }

    pub fn draw(
        &self,
        _draw_3d_handle: &mut crate::Draw3DHandle,
        position: crate::ffi::Vector3,
        scale: f32,
        tint: impl crate::color::ToRlColor,
    ) {
        unsafe { crate::ffi::DrawModel(self.inner, position, scale, tint.to_rl_color()) }
    }

    pub fn get_mesh_count(&self) -> NonZeroUsize {
        let count = usize::try_from(self.inner.meshCount).unwrap();
        NonZeroUsize::new(count).unwrap()
    }

    pub fn get_material_count(&self) -> NonZeroUsize {
        let count = usize::try_from(self.inner.materialCount).unwrap();
        NonZeroUsize::new(count).unwrap()
    }

    pub fn get_meshes(&self) -> &[crate::mesh::Mesh] {
        unsafe { std::slice::from_raw_parts(self.inner.meshes.cast(), self.get_mesh_count().get()) }
    }

    pub fn get_meshes_mut(&mut self) -> &mut [crate::mesh::Mesh] {
        unsafe {
            std::slice::from_raw_parts_mut(self.inner.meshes.cast(), self.get_mesh_count().get())
        }
    }

    pub fn get_materials(&self) -> &[crate::material::Material] {
        unsafe {
            std::slice::from_raw_parts(self.inner.materials.cast(), self.get_material_count().get())
        }
    }

    pub fn get_materials_mut(&mut self) -> &mut [crate::material::Material] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.inner.materials.cast(),
                self.get_material_count().get(),
            )
        }
    }

    pub fn get_mesh_material(&self, mesh_idx: usize) -> usize {
        assert!(mesh_idx < self.get_mesh_count().get());
        let idx = unsafe { self.inner.meshMaterial.add(mesh_idx).read() };
        idx.try_into().unwrap()
    }

    pub fn set_mesh_material(&mut self, mesh_idx: usize, material_idx: usize) {
        assert!(mesh_idx < self.get_mesh_count().get());
        assert!(material_idx < self.get_material_count().get());
        let material_idx = material_idx.try_into().unwrap();
        unsafe { self.inner.meshMaterial.add(mesh_idx).write(material_idx) };
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

    unsafe fn take(&mut self, id: ModelId) -> Option<Model> {
        unsafe { self.models.take(id) }
    }

    fn get(&self, id: ModelId) -> Option<&Model> {
        self.models.get(id)
    }

    fn get_mut(&mut self, id: ModelId) -> Option<&mut Model> {
        self.models.get_mut(id)
    }
}
