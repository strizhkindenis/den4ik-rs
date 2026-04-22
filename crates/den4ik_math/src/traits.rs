use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

pub trait Zero {
    #[must_use]
    fn zero() -> Self;
}

pub trait One {
    #[must_use]
    fn one() -> Self;
}

pub trait FromU64 {
    #[must_use]
    fn from_u64(value: u64) -> Self;
}

pub trait FromI64 {
    #[must_use]
    fn from_i64(value: i64) -> Self;
}

macro_rules! impl_num_both {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0 as $t
            }
        }

        impl One for $t {
            fn one() -> Self {
                1 as $t
            }
        }

        impl FromU64 for $t {
            fn from_u64(value: u64) -> Self {
                let s = value as $t;
                let s_u64 = s as u64;
                assert_eq!(s_u64, value);
                s
            }
        }

        impl FromI64 for $t {
            fn from_i64(value: i64) -> Self {
                let s = value as $t;
                let s_i64 = s as i64;
                assert_eq!(s_i64, value);
                s
            }
        }
    };
}

pub trait Num:
    Sized
    + Copy
    + Debug
    + Display
    + PartialEq
    + PartialOrd
    + for<'a> Add<&'a Self, Output = Self>
    + Add<Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + Sub<Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + Mul<Self, Output = Self>
    + for<'a> Div<&'a Self, Output = Self>
    + Div<Self, Output = Self>
    + for<'a> Rem<&'a Self, Output = Self>
    + Rem<Self, Output = Self>
    + AddAssign
    + for<'a> AddAssign<&'a Self>
    + SubAssign
    + for<'a> SubAssign<&'a Self>
    + MulAssign
    + for<'a> MulAssign<&'a Self>
    + DivAssign
    + for<'a> DivAssign<&'a Self>
    + RemAssign
    + for<'a> RemAssign<&'a Self>
    + FromU64
    + FromI64
where
    for<'a> &'a Self: Add<&'a Self, Output = Self>,
    for<'a> &'a Self: Add<Self, Output = Self>,
    for<'a> &'a Self: Sub<&'a Self, Output = Self>,
    for<'a> &'a Self: Sub<Self, Output = Self>,
    for<'a> &'a Self: Mul<&'a Self, Output = Self>,
    for<'a> &'a Self: Mul<Self, Output = Self>,
    for<'a> &'a Self: Div<&'a Self, Output = Self>,
    for<'a> &'a Self: Div<Self, Output = Self>,
    for<'a> &'a Self: Rem<&'a Self, Output = Self>,
    for<'a> &'a Self: Rem<Self, Output = Self>,
{
    const MIN: Self;
    const MAX: Self;

    #[must_use]
    fn min(self, other: Self) -> Self;
    #[must_use]
    fn max(self, other: Self) -> Self;
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_int {
    ($t:ty) => {
        impl_num_both!($t);

        impl Num for $t {
            const MIN: $t = <$t>::MIN;
            const MAX: $t = <$t>::MAX;

            fn min(self, other: Self) -> Self {
                Ord::min(self, other)
            }

            fn max(self, other: Self) -> Self {
                Ord::max(self, other)
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                Ord::clamp(self, min, max)
            }
        }
    };
}

impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u128);
impl_int!(usize);
impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);

// :TODO add more methods, not exhaustive
pub trait Float {
    #[must_use]
    fn acos(self) -> Self;
    #[must_use]
    fn asin(self) -> Self;
    #[must_use]
    fn cos(self) -> Self;
    #[must_use]
    fn sin(self) -> Self;
    #[must_use]
    fn is_nan(self) -> bool;
    #[must_use]
    fn sqrt(self) -> Self;
}

macro_rules! impl_float {
    ($t:ty) => {
        impl_num_both!($t);

        impl Num for $t {
            const MIN: $t = <$t>::MIN;
            const MAX: $t = <$t>::MAX;

            fn min(self, other: Self) -> Self {
                self.min(other)
            }

            fn max(self, other: Self) -> Self {
                self.max(other)
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                self.clamp(min, max)
            }
        }

        impl Float for $t {
            fn acos(self) -> Self {
                self.acos()
            }
            fn asin(self) -> Self {
                self.asin()
            }
            fn cos(self) -> Self {
                self.cos()
            }
            fn sin(self) -> Self {
                self.sin()
            }
            fn is_nan(self) -> bool {
                self.is_nan()
            }
            fn sqrt(self) -> Self {
                self.sqrt()
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
