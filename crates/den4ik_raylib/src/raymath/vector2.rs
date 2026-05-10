use crate::ffi::raymath::Vector2 as FFIVector2;

pub trait RlVector2: Sized + Clone + Copy {
    fn zero(self) -> Self;
    fn one(self) -> Self;
    fn add(self, v2: Self) -> Self;
    fn add_value(self, add: f32) -> Self;
    fn subtract(self, v2: Self) -> Self;
    fn subtract_value(self, sub: f32) -> Self;
    fn length(self) -> f32;
    fn length_sqr(self) -> f32;
    fn dot_product(self, v2: Self) -> f32;
    fn cross_product(self, v2: Self) -> f32;
    fn distance(self, v2: Self) -> f32;
    fn distance_sqr(self, v2: Self) -> f32;
    fn angle(self, v2: Self) -> f32;
    fn line_angle(self, end: Self) -> f32;
    fn scale(self, scale: f32) -> Self;
    fn multiply(self, v2: Self) -> Self;
    fn negate(self) -> Self;
    fn divide(self, v2: Self) -> Self;
    fn normalize(self) -> Self;
    fn transform(self, mat: impl super::RlMatrix) -> Self;
    fn lerp(self, v2: Self, amount: f32) -> Self;
    fn reflect(self, normal: Self) -> Self;
    fn min(self, v2: Self) -> Self;
    fn max(self, v2: Self) -> Self;
    fn rotate(self, angle: f32) -> Self;
    fn move_towards(self, target: Self, max_distance: f32) -> Self;
    fn invert(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn clamp_value(self, min: f32, max: f32) -> Self;
    fn equals(self, q: Self) -> bool;
    fn refract(self, n: Self, r: f32) -> Self;
}

#[derive(Clone, Copy)]
pub struct Vector2 {
    inner: FFIVector2,
}

impl Default for Vector2 {
	fn default() -> Self {
		Self::new(0.0, 0.0)
	}
}

impl Vector2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self { inner: FFIVector2 { x, y } }	
	}

	pub fn get_x(self) -> f32 {
		self.inner.x	
	}
	
	pub fn get_x_mut(&mut self) -> &mut f32 {
		&mut self.inner.x	
	}
	
	pub fn get_y(self) -> f32 {
		self.inner.y	
	}
	
	pub fn get_y_mut(&mut self) -> &mut f32 {
		&mut self.inner.y	
	}
}
