pub trait RlVector4 {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, v2: Self) -> Self;
    fn add_value(&self, add: f32) -> Self;
    fn subtract(&self, v2: Self) -> Self;
    fn subtract_value(&self, sub: f32) -> Self;
    fn length(&self) -> f32;
    fn length_sqr(&self) -> f32;
    fn dot_product(&self, v2: Self) -> f32;
    fn distance(&self, v2: Self) -> f32;
    fn distance_sqr(&self, v2: Self) -> f32;
    fn scale(&self, scale: f32) -> Self;
    fn multiply(&self, v2: Self) -> Self;
    fn negate(&self) -> Self;
    fn divide(&self, v2: Self) -> Self;
    fn normalize(&self) -> Self;
    fn min(&self, v2: Self) -> Self;
    fn max(&self, v2: Self) -> Self;
    fn lerp(&self, v2: Self, amount: f32) -> Self;
    fn move_towards(&self, target: Self, max_distance: f32) -> Self;
    fn invert(&self) -> Self;
    fn equals(&self, q: Self) -> i32;
}
