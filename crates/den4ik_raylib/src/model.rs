use std::{num::NonZeroUsize, path::Path};

use crate::Container;

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
    ) -> Result<usize, ModelError> {
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
        mesh_id: usize,
    ) -> Result<usize, ModelError> {
        let mesh: crate::mesh::Mesh =
            unsafe { handle.take(mesh_id).ok_or(ModelError::EmptyModel)? };
        let inner = unsafe { crate::ffi::LoadModelFromMesh(mesh.into_inner()) };
        if inner.meshCount > 0 {
            Ok(handle.add(Self { inner }))
        } else {
            Err(ModelError::EmptyModel)
        }
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

impl Container<Model> for crate::RaylibHandle {
    fn add(&mut self, item: Model) -> usize {
        self.models.add(item)
    }

    fn remove(&mut self, id: usize) -> bool {
        self.models.remove(id)
    }

    unsafe fn take(&mut self, id: usize) -> Option<Model> {
        unsafe { self.models.take(id) }
    }

    fn get(&self, id: usize) -> Option<&Model> {
        self.models.get(id)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut Model> {
        self.models.get_mut(id)
    }
}
