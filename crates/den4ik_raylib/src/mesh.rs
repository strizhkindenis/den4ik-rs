use crate::{
	Handle,
    allocator::{Allocator, AllocatorError},
    container::{Container, ContainerId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshConfigError {
    TrianglePointMiscount,
}

pub struct MeshConfig {
    vertex_count: u32,
    triangle_count: u32,
    indices_count: Option<u32>,
    has_texcoords2: bool,
    has_normals: bool,
    has_tangents: bool,
    has_colors: bool,
}

pub struct MeshConfigBuilder {
    config: MeshConfig,
}

impl MeshConfigBuilder {
    pub fn new(vertex_count: u32, triangle_count: u32) -> Self {
        Self {
            config: MeshConfig {
                vertex_count,
                triangle_count,
                indices_count: None,
                has_texcoords2: false,
                has_normals: false,
                has_tangents: false,
                has_colors: false,
            },
        }
    }

    pub fn with_texcoords2(mut self) -> Self {
        self.config.has_texcoords2 = true;
        self
    }

    pub fn with_normals(mut self) -> Self {
        self.config.has_normals = true;
        self
    }

    pub fn with_tangents(mut self) -> Self {
        self.config.has_tangents = true;
        self
    }

    pub fn with_colors(mut self) -> Self {
        self.config.has_colors = true;
        self
    }

    pub fn with_indices(mut self, indices_count: u32) -> Self {
        self.config.indices_count = Some(indices_count);
        self
    }

    pub fn build(self) -> Result<MeshConfig, MeshConfigError> {
        if self.config.indices_count.is_none() {
            if self.config.vertex_count % 3 != 0
                || self.config.triangle_count * 3 != self.config.vertex_count
            {
                return Err(MeshConfigError::TrianglePointMiscount);
            }
        }
        Ok(self.config)
    }
}

pub struct Mesh {
    inner: crate::ffi::Mesh,
    triangle_count: usize,
    vertex_count: usize,
}

impl Mesh {
    pub fn new(
        handle: &mut crate::RaylibHandle,
        config: MeshConfig,
    ) -> Result<MeshId, AllocatorError> {
        let allocator = Allocator::new(handle);
        let mut inner = crate::ffi::Mesh {
            vertexCount: config.vertex_count.try_into().unwrap(),
            triangleCount: config.triangle_count.try_into().unwrap(),
            indices: std::ptr::null_mut(),
            vertices: std::ptr::null_mut(),
            texcoords: std::ptr::null_mut(),
            texcoords2: std::ptr::null_mut(),
            normals: std::ptr::null_mut(),
            tangents: std::ptr::null_mut(),
            colors: std::ptr::null_mut(),
            boneCount: 0,
            boneIndices: std::ptr::null_mut(),
            boneWeights: std::ptr::null_mut(),
            animNormals: std::ptr::null_mut(),
            animVertices: std::ptr::null_mut(),
            vaoId: 0,
            vboId: std::ptr::null_mut(),
        };
        let vertex_count = usize::try_from(config.vertex_count).unwrap();
        let triangle_count = usize::try_from(config.triangle_count).unwrap();
        inner.vertices = allocator.alloc_default_array(vertex_count)?;
        inner.texcoords = allocator.alloc_default_array(vertex_count)?;
        if config.has_normals {
            inner.normals = allocator.alloc_default_array(vertex_count)?;
        }
        if config.has_tangents {
            inner.tangents = allocator.alloc_default_array(vertex_count)?;
        }
        if config.has_colors {
            inner.colors = allocator.alloc_default_array(vertex_count)?;
        }
        if let Some(indices_count) = config.indices_count {
            let indices_count = usize::try_from(indices_count).unwrap();
            inner.indices = allocator.alloc_default_array(indices_count)?;
        }
        Ok(handle.add(Self {
            inner,
            triangle_count,
            vertex_count,
        }))
    }

    pub fn upload(&mut self, dynamic: bool) {
        unsafe { crate::ffi::UploadMesh(&mut self.inner, dynamic) }
    }

    pub(crate) fn into_inner(self) -> crate::ffi::Mesh {
        self.inner
    }

    pub fn get_vertices(&self) -> &[[f32; 3]] {
        unsafe { std::slice::from_raw_parts(self.inner.vertices.cast(), self.vertex_count) }
    }

    pub fn get_vertices_mut(&mut self) -> &mut [[f32; 3]] {
        unsafe { std::slice::from_raw_parts_mut(self.inner.vertices.cast(), self.vertex_count) }
    }

    pub fn get_texcoords(&self) -> &[[f32; 2]] {
        unsafe { std::slice::from_raw_parts(self.inner.texcoords.cast(), self.vertex_count) }
    }

    pub fn get_texcoords_mut(&mut self) -> &mut [[f32; 2]] {
        unsafe { std::slice::from_raw_parts_mut(self.inner.texcoords.cast(), self.vertex_count) }
    }

    pub fn get_normals(&self) -> &[[f32; 3]] {
        if self.inner.normals.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.inner.normals.cast(), self.vertex_count) }
    }

    pub fn get_normals_mut(&mut self) -> &mut [[f32; 3]] {
        if self.inner.normals.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.inner.normals.cast(), self.vertex_count) }
    }

    pub fn get_tangents(&self) -> &[[f32; 4]] {
        if self.inner.tangents.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.inner.tangents.cast(), self.vertex_count) }
    }

    pub fn get_tangents_mut(&mut self) -> &mut [[f32; 4]] {
        if self.inner.tangents.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.inner.tangents.cast(), self.vertex_count) }
    }

    pub fn get_colors(&self) -> &[[f32; 4]] {
        if self.inner.colors.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.inner.colors.cast(), self.vertex_count) }
    }

    pub fn get_colors_mut(&mut self) -> &mut [[f32; 4]] {
        if self.inner.colors.is_null() {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(self.inner.colors.cast(), self.vertex_count) }
    }

    pub fn get_indices(&self) -> &[u16] {
        if self.inner.indices.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(self.inner.indices.cast(), self.triangle_count * 3) }
    }

    pub fn get_indices_mut(&mut self) -> &mut [u16] {
        if self.inner.indices.is_null() {
            return &mut [];
        }
        unsafe {
            std::slice::from_raw_parts_mut(self.inner.indices.cast(), self.triangle_count * 3)
        }
    }

    pub fn gen_plane(
        handle: &mut crate::RaylibHandle,
        width: f32,
        length: f32,
        res_x: u32,
        res_z: u32,
    ) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshPlane(
                width,
                length,
                res_x.try_into().unwrap(),
                res_z.try_into().unwrap(),
            )
        };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }

    pub fn gen_cube(
        handle: &mut crate::RaylibHandle,
        width: f32,
        height: f32,
        length: f32,
    ) -> MeshId {
        let inner = unsafe { crate::ffi::GenMeshCube(width, height, length) };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }

    pub fn gen_sphere(
        handle: &mut crate::RaylibHandle,
        radius: f32,
        rings: u32,
        slices: u32,
    ) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshSphere(
                radius,
                rings.try_into().unwrap(),
                slices.try_into().unwrap(),
            )
        };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }

    pub fn gen_hemisphere(
        handle: &mut crate::RaylibHandle,
        radius: f32,
        rings: u32,
        slices: u32,
    ) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshHemiSphere(
                radius,
                rings.try_into().unwrap(),
                slices.try_into().unwrap(),
            )
        };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }

    pub fn gen_cylinder(
        handle: &mut crate::RaylibHandle,
        radius: f32,
        height: f32,
        slices: u32,
    ) -> MeshId {
        let inner =
            unsafe { crate::ffi::GenMeshCylinder(radius, height, slices.try_into().unwrap()) };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }

    pub fn gen_torus(
        handle: &mut crate::RaylibHandle,
        radius: f32,
        size: f32,
        rad_seg: u32,
        sides: u32,
    ) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshTorus(
                radius,
                size,
                rad_seg.try_into().unwrap(),
                sides.try_into().unwrap(),
            )
        };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }

    pub fn gen_knot(
        handle: &mut crate::RaylibHandle,
        radius: f32,
        size: f32,
        rad_seg: u32,
        sides: u32,
    ) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshKnot(
                radius,
                size,
                rad_seg.try_into().unwrap(),
                sides.try_into().unwrap(),
            )
        };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }

    pub fn gen_poly(handle: &mut crate::RaylibHandle, sides: u32, radius: f32) -> MeshId {
        let inner = unsafe { crate::ffi::GenMeshPoly(sides.try_into().unwrap(), radius) };
        let m = Self {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        handle.add(m)
    }
}

impl crate::Unloadable for Mesh {
    fn unload(item: Self) {
        unsafe { crate::ffi::UnloadMesh(item.inner) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshId(usize);

impl ContainerId for MeshId {
    fn to_usize(self) -> usize {
        self.0
    }

    fn from_usize(value: usize) -> Self {
        Self(value)
    }
}

impl Container<Mesh, MeshId> for Handle {
    fn add(&mut self, item: Mesh) -> MeshId {
        self.meshes.add(item)
    }

    fn remove(&mut self, id: MeshId) -> bool {
        self.meshes.remove(id)
    }

    fn take(&mut self, id: MeshId) -> Option<Mesh> {
        unsafe { self.meshes.take(id) }
    }

    fn get(&self, id: MeshId) -> Option<&Mesh> {
        self.meshes.get(id)
    }

    fn get_mut(&mut self, id: MeshId) -> Option<&mut Mesh> {
        self.meshes.get_mut(id)
    }
}
