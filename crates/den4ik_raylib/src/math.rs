use crate::ffi;

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub(crate) inner: ffi::Vector2,
}

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            inner: ffi::Vector2 { x, y },
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }
    pub fn y(&self) -> f32 {
        self.inner.y
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<ffi::Vector2> for Vector2 {
    fn from(inner: ffi::Vector2) -> Self {
        Self { inner }
    }
}

impl Into<ffi::Vector2> for Vector2 {
    fn into(self) -> ffi::Vector2 {
        self.inner
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub(crate) inner: ffi::Vector3,
}

impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            inner: ffi::Vector3 { x, y, z },
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }
    pub fn y(&self) -> f32 {
        self.inner.y
    }
    pub fn z(&self) -> f32 {
        self.inner.z
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<ffi::Vector3> for Vector3 {
    fn from(inner: ffi::Vector3) -> Self {
        Self { inner }
    }
}

impl Into<ffi::Vector3> for Vector3 {
    fn into(self) -> ffi::Vector3 {
        self.inner
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
    pub(crate) inner: ffi::Vector4,
}

impl Vector4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            inner: ffi::Vector4 { x, y, z, w },
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }
    pub fn y(&self) -> f32 {
        self.inner.y
    }
    pub fn z(&self) -> f32 {
        self.inner.z
    }
    pub fn w(&self) -> f32 {
        self.inner.w
    }
}

impl Default for Vector4 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl From<ffi::Vector4> for Vector4 {
    fn from(inner: ffi::Vector4) -> Self {
        Self { inner }
    }
}

impl Into<ffi::Vector4> for Vector4 {
    fn into(self) -> ffi::Vector4 {
        self.inner
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    pub(crate) inner: ffi::Matrix,
}

impl Matrix {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        m0: f32,
        m4: f32,
        m8: f32,
        m12: f32,
        m1: f32,
        m5: f32,
        m9: f32,
        m13: f32,
        m2: f32,
        m6: f32,
        m10: f32,
        m14: f32,
        m3: f32,
        m7: f32,
        m11: f32,
        m15: f32,
    ) -> Self {
        Self {
            inner: ffi::Matrix {
                m0,
                m4,
                m8,
                m12,
                m1,
                m5,
                m9,
                m13,
                m2,
                m6,
                m10,
                m14,
                m3,
                m7,
                m11,
                m15,
            },
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl From<ffi::Matrix> for Matrix {
    fn from(inner: ffi::Matrix) -> Self {
        Self { inner }
    }
}

impl Into<ffi::Matrix> for Matrix {
    fn into(self) -> ffi::Matrix {
        self.inner
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub(crate) inner: ffi::Rectangle,
}

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            inner: ffi::Rectangle {
                x,
                y,
                width,
                height,
            },
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }
    pub fn y(&self) -> f32 {
        self.inner.y
    }
    pub fn width(&self) -> f32 {
        self.inner.width
    }
    pub fn height(&self) -> f32 {
        self.inner.height
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl From<ffi::Rectangle> for Rectangle {
    fn from(inner: ffi::Rectangle) -> Self {
        Self { inner }
    }
}

impl Into<ffi::Rectangle> for Rectangle {
    fn into(self) -> ffi::Rectangle {
        self.inner
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub(crate) inner: ffi::Ray,
}

impl Ray {
    pub const fn new(position: Vector3, direction: Vector3) -> Self {
        Self {
            inner: ffi::Ray {
                position: position.inner,
                direction: direction.inner,
            },
        }
    }

    pub fn position(&self) -> Vector3 {
        self.inner.position.into()
    }
    pub fn direction(&self) -> Vector3 {
        self.inner.direction.into()
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self::new(Vector3::default(), Vector3::default())
    }
}

impl From<ffi::Ray> for Ray {
    fn from(inner: ffi::Ray) -> Self {
        Self { inner }
    }
}

impl Into<ffi::Ray> for Ray {
    fn into(self) -> ffi::Ray {
        self.inner
    }
}
