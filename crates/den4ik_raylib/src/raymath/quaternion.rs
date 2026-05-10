pub trait RlQuaternion {
    fn add(&self, q2: Self) -> Self;
    fn add_value(&self, add: f32) -> Self;
    fn subtract(&self, q2: Self) -> Self;
    fn subtract_value(&self, sub: f32) -> Self;
    fn identity() -> Self;
    fn length(&self) -> f32;
    fn normalize(&self) -> Self;
    fn invert(&self) -> Self;
    fn multiply(&self, q2: Self) -> Self;
    fn scale(&self, mul: f32) -> Self;
    fn divide(&self, q2: Self) -> Self;
    fn lerp(&self, q2: Self, amount: f32) -> Self;
    fn nlerp(&self, q2: Self, amount: f32) -> Self;
    fn slerp(&self, q2: Self, amount: f32) -> Self;
    fn cubic_hermite_spline(&self, out_tangent1: Self, q2: Self, in_tangent2: Self, t: f32)
    -> Self;
    fn from_vector3_to_vector3(from: impl super::RlVector3, to: impl super::RlVector3) -> Self;
    fn from_matrix(mat: impl super::RlMatrix) -> Self;
    fn to_matrix(&self) -> impl super::RlMatrix;
    fn from_axis_angle(axis: impl super::RlVector3, angle: f32) -> Self;
    fn to_axis_angle(&self, out_axis: &mut impl super::RlVector3, out_angle: &mut f32);
    fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Self;
    fn to_euler(&self) -> impl super::RlVector3;
    fn transform(&self, mat: impl super::RlMatrix) -> Self;
    fn equals(&self, q: Self) -> i32;
}
