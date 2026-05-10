pub trait RlMatrix {
    fn determinant(&self) -> f32;
    fn trace(&self) -> f32;
    fn transpose(&self) -> Self;
    fn invert(&self) -> Self;
    fn identity() -> Self;
    fn add(&self, right: Self) -> Self;
    fn subtract(&self, right: Self) -> Self;
    fn multiply(&self, right: Self) -> Self;
    fn multiply_value(&self, value: f32) -> Self;
    fn translate(x: f32, y: f32, z: f32) -> Self;
    fn rotate(axis: impl super::RlVector3, angle: f32) -> Self;
    fn rotate_x(angle: f32) -> Self;
    fn rotate_y(angle: f32) -> Self;
    fn rotate_z(angle: f32) -> Self;
    fn rotate_xyz(angle: impl super::RlVector3) -> Self;
    fn rotate_zyx(angle: impl super::RlVector3) -> Self;
    fn scale(x: f32, y: f32, z: f32) -> Self;
    fn frustum(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near_plane: f64,
        far_plane: f64,
    ) -> Self;
    fn perspective(fov_y: f64, aspect: f64, near_plane: f64, far_plane: f64) -> Self;
    fn ortho(left: f64, right: f64, bottom: f64, top: f64, near_plane: f64, far_plane: f64)
    -> Self;
    fn look_at(
        eye: impl super::RlVector3,
        target: impl super::RlVector3,
        up: impl super::RlVector3,
    ) -> Self;
    fn to_float_v(&self) -> crate::ffi::raymath::float16;
    fn compose(
        translation: impl super::RlVector3,
        rotation: impl super::RlQuaternion,
        scale: impl super::RlVector3,
    ) -> Self;
    fn decompose(
        &self,
        translation: &mut impl super::RlVector3,
        rotation: &mut impl super::RlQuaternion,
        scale: &mut impl super::RlVector3,
    );
}
