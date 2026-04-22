#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

pub mod mat;
pub mod traits;
pub mod vec_n;

pub use mat::{Dims, Mat};
pub use traits::{Num, Float};
pub use vec_n::{Vec1, Vec2, Vec3, Vec4};
