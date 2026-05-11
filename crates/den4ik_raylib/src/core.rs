use crate::color::ToRlColor;

// Draw
pub fn begin_drawing() {
    unsafe { crate::ffi::BeginDrawing() }
}

pub fn end_drawing() {
    unsafe { crate::ffi::EndDrawing() }
}

pub fn clear_background(color: impl ToRlColor) {
    unsafe { crate::ffi::ClearBackground(color.to_rl_color()) }
}

pub fn begin_mode_3d(camera: crate::ffi::Camera3D) {
    unsafe { crate::ffi::BeginMode3D(camera) }
}

pub fn end_mode_3d() {
    unsafe { crate::ffi::EndMode3D() }
}

pub fn draw_grid(slices: i32, spacing: f32) {
    unsafe { crate::ffi::DrawGrid(slices, spacing) }
}

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: impl ToRlColor) {
    unsafe { crate::ffi::DrawRectangle(pos_x, pos_y, width, height, color.to_rl_color()) }
}

pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: impl ToRlColor) {
    unsafe { crate::ffi::DrawRectangleLines(pos_x, pos_y, width, height, color.to_rl_color()) }
}

pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: impl ToRlColor) {
    let text_c = std::ffi::CString::new(text).unwrap();
    unsafe { crate::ffi::DrawText(text_c.as_ptr(), pos_x, pos_y, font_size, color.to_rl_color()) }
}

// Camera
pub fn update_camera(camera: &mut crate::ffi::Camera3D, mode: i32) {
    unsafe { crate::ffi::UpdateCamera(camera as *mut _, mode) }
}

pub use crate::ffi::CameraMode_CAMERA_ORBITAL as CAMERA_ORBITAL;
pub use crate::ffi::Camera3D as Camera;
pub use crate::ffi::Vector3;

// Input
pub fn is_mouse_button_pressed(button: i32) -> bool {
    unsafe { crate::ffi::IsMouseButtonPressed(button) }
}

pub fn is_key_pressed(key: i32) -> bool {
    unsafe { crate::ffi::IsKeyPressed(key) }
}

pub use crate::ffi::MouseButton_MOUSE_BUTTON_LEFT as MOUSE_BUTTON_LEFT;
pub use crate::ffi::KeyboardKey_KEY_RIGHT as KEY_RIGHT;
pub use crate::ffi::KeyboardKey_KEY_LEFT as KEY_LEFT;
