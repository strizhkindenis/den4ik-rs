use crate::{
    allocator::Allocator,
    container::{Container, ContainerId},
};

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

    pub fn build(self) -> Result<MeshConfig, crate::RaylibError> {
        if self.config.indices_count.is_none() {
            if self.config.vertex_count % 3 != 0
                || self.config.triangle_count * 3 != self.config.vertex_count
            {
                return Err(crate::RaylibError::TrianglePointMiscount);
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
    pub(crate) fn into_inner(self) -> crate::ffi::Mesh {
        self.inner
    }
}

impl crate::RaylibHandle {
    pub fn load_mesh(&mut self, config: MeshConfig) -> Result<MeshId, crate::RaylibError> {
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

        {
            let allocator = Allocator::new(self);
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
        }

        Ok(self.meshes.add(Mesh {
            inner,
            triangle_count,
            vertex_count,
        }))
    }

    pub fn upload_mesh(&mut self, id: MeshId, dynamic: bool) -> Result<(), crate::RaylibError> {
        let mesh = self
            .meshes
            .get_mut(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        unsafe { crate::ffi::UploadMesh(&mut mesh.inner, dynamic) }
        Ok(())
    }

    pub fn get_mesh_vertices(&self, id: MeshId) -> Result<&[[f32; 3]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        Ok(unsafe { std::slice::from_raw_parts(mesh.inner.vertices.cast(), mesh.vertex_count) })
    }

    pub fn get_mesh_vertices_mut(
        &mut self,
        id: MeshId,
    ) -> Result<&mut [[f32; 3]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get_mut(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        Ok(
            unsafe {
                std::slice::from_raw_parts_mut(mesh.inner.vertices.cast(), mesh.vertex_count)
            },
        )
    }

    pub fn get_mesh_texcoords(&self, id: MeshId) -> Result<&[[f32; 2]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        Ok(unsafe { std::slice::from_raw_parts(mesh.inner.texcoords.cast(), mesh.vertex_count) })
    }

    pub fn get_mesh_texcoords_mut(
        &mut self,
        id: MeshId,
    ) -> Result<&mut [[f32; 2]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get_mut(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        Ok(unsafe {
            std::slice::from_raw_parts_mut(mesh.inner.texcoords.cast(), mesh.vertex_count)
        })
    }

    pub fn get_mesh_normals(&self, id: MeshId) -> Result<&[[f32; 3]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.normals.is_null() {
            return Ok(&[]);
        }
        Ok(unsafe { std::slice::from_raw_parts(mesh.inner.normals.cast(), mesh.vertex_count) })
    }

    pub fn get_mesh_normals_mut(
        &mut self,
        id: MeshId,
    ) -> Result<&mut [[f32; 3]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get_mut(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.normals.is_null() {
            return Ok(&mut []);
        }
        Ok(unsafe { std::slice::from_raw_parts_mut(mesh.inner.normals.cast(), mesh.vertex_count) })
    }

    pub fn get_mesh_tangents(&self, id: MeshId) -> Result<&[[f32; 4]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.tangents.is_null() {
            return Ok(&[]);
        }
        Ok(unsafe { std::slice::from_raw_parts(mesh.inner.tangents.cast(), mesh.vertex_count) })
    }

    pub fn get_mesh_tangents_mut(
        &mut self,
        id: MeshId,
    ) -> Result<&mut [[f32; 4]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get_mut(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.tangents.is_null() {
            return Ok(&mut []);
        }
        Ok(
            unsafe {
                std::slice::from_raw_parts_mut(mesh.inner.tangents.cast(), mesh.vertex_count)
            },
        )
    }

    pub fn get_mesh_colors(&self, id: MeshId) -> Result<&[[f32; 4]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.colors.is_null() {
            return Ok(&[]);
        }
        Ok(unsafe { std::slice::from_raw_parts(mesh.inner.colors.cast(), mesh.vertex_count) })
    }

    pub fn get_mesh_colors_mut(
        &mut self,
        id: MeshId,
    ) -> Result<&mut [[f32; 4]], crate::RaylibError> {
        let mesh = self
            .meshes
            .get_mut(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.colors.is_null() {
            return Ok(&mut []);
        }
        Ok(unsafe { std::slice::from_raw_parts_mut(mesh.inner.colors.cast(), mesh.vertex_count) })
    }

    pub fn get_mesh_indices(&self, id: MeshId) -> Result<&[u16], crate::RaylibError> {
        let mesh = self
            .meshes
            .get(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.indices.is_null() {
            return Ok(&[]);
        }
        Ok(unsafe {
            std::slice::from_raw_parts(mesh.inner.indices.cast(), mesh.triangle_count * 3)
        })
    }

    pub fn get_mesh_indices_mut(&mut self, id: MeshId) -> Result<&mut [u16], crate::RaylibError> {
        let mesh = self
            .meshes
            .get_mut(id)
            .ok_or(crate::RaylibError::InvalidMeshId)?;
        if mesh.inner.indices.is_null() {
            return Ok(&mut []);
        }
        Ok(unsafe {
            std::slice::from_raw_parts_mut(mesh.inner.indices.cast(), mesh.triangle_count * 3)
        })
    }

    pub fn gen_mesh_plane(&mut self, width: f32, length: f32, res_x: u32, res_z: u32) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshPlane(
                width,
                length,
                res_x.try_into().unwrap(),
                res_z.try_into().unwrap(),
            )
        };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
    }

    pub fn gen_mesh_cube(&mut self, width: f32, height: f32, length: f32) -> MeshId {
        let inner = unsafe { crate::ffi::GenMeshCube(width, height, length) };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
    }

    pub fn gen_mesh_sphere(&mut self, radius: f32, rings: u32, slices: u32) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshSphere(
                radius,
                rings.try_into().unwrap(),
                slices.try_into().unwrap(),
            )
        };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
    }

    pub fn gen_mesh_hemisphere(&mut self, radius: f32, rings: u32, slices: u32) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshHemiSphere(
                radius,
                rings.try_into().unwrap(),
                slices.try_into().unwrap(),
            )
        };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
    }

    pub fn gen_mesh_cylinder(&mut self, radius: f32, height: f32, slices: u32) -> MeshId {
        let inner =
            unsafe { crate::ffi::GenMeshCylinder(radius, height, slices.try_into().unwrap()) };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
    }

    pub fn gen_mesh_torus(&mut self, radius: f32, size: f32, rad_seg: u32, sides: u32) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshTorus(
                radius,
                size,
                rad_seg.try_into().unwrap(),
                sides.try_into().unwrap(),
            )
        };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
    }

    pub fn gen_mesh_knot(&mut self, radius: f32, size: f32, rad_seg: u32, sides: u32) -> MeshId {
        let inner = unsafe {
            crate::ffi::GenMeshKnot(
                radius,
                size,
                rad_seg.try_into().unwrap(),
                sides.try_into().unwrap(),
            )
        };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
    }

    pub fn gen_mesh_poly(&mut self, sides: u32, radius: f32) -> MeshId {
        let inner = unsafe { crate::ffi::GenMeshPoly(sides.try_into().unwrap(), radius) };
        let m = Mesh {
            inner,
            vertex_count: inner.vertexCount as usize,
            triangle_count: inner.triangleCount as usize,
        };
        self.meshes.add(m)
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
