pub use crate::ffi::Camera3D as Camera;
pub use crate::ffi::CameraMode_CAMERA_ORBITAL as CAMERA_ORBITAL;
pub use crate::ffi::Vector3;

pub fn update_camera(camera: &mut Camera, mode: i32) {
    unsafe { crate::ffi::UpdateCamera(camera, mode) }
}

// Input
pub fn is_mouse_button_pressed(button: i32) -> bool {
    unsafe { crate::ffi::IsMouseButtonPressed(button) }
}

pub fn is_key_pressed(key: i32) -> bool {
    unsafe { crate::ffi::IsKeyPressed(key) }
}

pub use crate::ffi::KeyboardKey_KEY_LEFT as KEY_LEFT;
pub use crate::ffi::KeyboardKey_KEY_RIGHT as KEY_RIGHT;
pub use crate::ffi::MouseButton_MOUSE_BUTTON_LEFT as MOUSE_BUTTON_LEFT;
