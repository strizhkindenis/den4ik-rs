#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

mod float;
pub mod mat;
pub mod traits;

pub use float::{Float, FloatConsts};
pub use mat::{Dims, Mat};
pub use traits::{One, Zero};
