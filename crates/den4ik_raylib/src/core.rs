use crate::ffi;
use crate::math::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub(crate) inner: ffi::Camera3D,
}

impl Camera {
    pub const fn new(position: Vector3, target: Vector3, up: Vector3, fovy: f32, projection: i32) -> Self {
        Self {
            inner: ffi::Camera3D {
                position: position.inner,
                target: target.inner,
                up: up.inner,
                fovy,
                projection,
            }
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vector3::default(),
            Vector3::default(),
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
            0,
        )
    }
}

pub use crate::ffi::CameraMode_CAMERA_ORBITAL as CAMERA_ORBITAL;
pub use crate::ffi::KeyboardKey_KEY_LEFT as KEY_LEFT;
pub use crate::ffi::KeyboardKey_KEY_RIGHT as KEY_RIGHT;
pub use crate::ffi::MouseButton_MOUSE_BUTTON_LEFT as MOUSE_BUTTON_LEFT;
