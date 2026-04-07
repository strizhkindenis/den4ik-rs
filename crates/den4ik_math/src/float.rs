use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
    num::FpCategory,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

pub trait Float:
    Sized
    + Copy
    + Clone
    + Debug
    + Default
    + Display
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + Neg
    + Sum
    + PartialEq
    + PartialOrd
    + Zero
    + One
    + FloatConsts
{
    const DIGITS: u32;
    const EPSILON: Self;
    const INFINITY: Self;
    const MANTISSA_DIGITS: u32;
    const MAX: Self;
    const MAX_10_EXP: i32;
    const MAX_EXP: i32;
    const MIN: Self;
    const MIN_10_EXP: i32;
    const MIN_EXP: i32;
    const MIN_POSITIVE: Self;
    const NAN: Self;
    const NEG_INFINITY: Self;
    const RADIX: u32;

    fn abs(self) -> Self;
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn atanh(self) -> Self;
    fn cbrt(self) -> Self;
    fn ceil(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn classify(self) -> FpCategory;
    fn copysign(self, sign: Self) -> Self;
    fn cos(self) -> Self;
    fn cosh(self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn is_subnormal(self) -> bool;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn midpoint(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn next_down(self) -> Self;
    fn next_up(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn recip(self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn round(self) -> Self;
    fn round_ties_even(self) -> Self;
    fn signum(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn sinh(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn total_cmp(&self, other: &Self) -> Ordering;
    fn trunc(self) -> Self;
}

macro_rules! impl_float {
    ($t:ty) => {
        impl Float for $t {
            const DIGITS: u32 = <$t>::DIGITS;
            const EPSILON: Self = <$t>::EPSILON;
            const INFINITY: Self = <$t>::INFINITY;
            const MANTISSA_DIGITS: u32 = <$t>::MANTISSA_DIGITS;
            const MAX: Self = <$t>::MAX;
            const MAX_10_EXP: i32 = <$t>::MAX_10_EXP;
            const MAX_EXP: i32 = <$t>::MAX_EXP;
            const MIN: Self = <$t>::MIN;
            const MIN_10_EXP: i32 = <$t>::MIN_10_EXP;
            const MIN_EXP: i32 = <$t>::MIN_EXP;
            const MIN_POSITIVE: Self = <$t>::MIN_POSITIVE;
            const NAN: Self = <$t>::NAN;
            const NEG_INFINITY: Self = <$t>::NEG_INFINITY;
            const RADIX: u32 = <$t>::RADIX;

            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
            #[inline]
            fn acos(self) -> Self {
                self.acos()
            }
            #[inline]
            fn acosh(self) -> Self {
                self.acosh()
            }
            #[inline]
            fn asin(self) -> Self {
                self.asin()
            }
            #[inline]
            fn asinh(self) -> Self {
                self.asinh()
            }
            #[inline]
            fn atan(self) -> Self {
                self.atan()
            }
            #[inline]
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            #[inline]
            fn atanh(self) -> Self {
                self.atanh()
            }
            #[inline]
            fn cbrt(self) -> Self {
                self.cbrt()
            }
            #[inline]
            fn ceil(self) -> Self {
                self.ceil()
            }
            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self {
                self.clamp(min, max)
            }
            #[inline]
            fn classify(self) -> FpCategory {
                self.classify()
            }
            #[inline]
            fn copysign(self, sign: Self) -> Self {
                self.copysign(sign)
            }
            #[inline]
            fn cos(self) -> Self {
                self.cos()
            }
            #[inline]
            fn cosh(self) -> Self {
                self.cosh()
            }
            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                self.div_euclid(rhs)
            }
            #[inline]
            fn exp(self) -> Self {
                self.exp()
            }
            #[inline]
            fn exp2(self) -> Self {
                self.exp2()
            }
            #[inline]
            fn exp_m1(self) -> Self {
                self.exp_m1()
            }
            #[inline]
            fn floor(self) -> Self {
                self.floor()
            }
            #[inline]
            fn fract(self) -> Self {
                self.fract()
            }
            #[inline]
            fn hypot(self, other: Self) -> Self {
                self.hypot(other)
            }
            #[inline]
            fn is_finite(self) -> bool {
                self.is_finite()
            }
            #[inline]
            fn is_infinite(self) -> bool {
                self.is_infinite()
            }
            #[inline]
            fn is_nan(self) -> bool {
                self.is_nan()
            }
            #[inline]
            fn is_normal(self) -> bool {
                self.is_normal()
            }
            #[inline]
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }
            #[inline]
            fn is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }
            #[inline]
            fn is_subnormal(self) -> bool {
                self.is_subnormal()
            }
            #[inline]
            fn ln(self) -> Self {
                self.ln()
            }
            #[inline]
            fn ln_1p(self) -> Self {
                self.ln_1p()
            }
            #[inline]
            fn log(self, base: Self) -> Self {
                self.log(base)
            }
            #[inline]
            fn log2(self) -> Self {
                self.log2()
            }
            #[inline]
            fn log10(self) -> Self {
                self.log10()
            }
            #[inline]
            fn max(self, other: Self) -> Self {
                self.max(other)
            }
            #[inline]
            fn midpoint(self, other: Self) -> Self {
                self.midpoint(other)
            }
            #[inline]
            fn min(self, other: Self) -> Self {
                self.min(other)
            }
            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }
            #[inline]
            fn next_down(self) -> Self {
                self.next_down()
            }
            #[inline]
            fn next_up(self) -> Self {
                self.next_up()
            }
            #[inline]
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }
            #[inline]
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }
            #[inline]
            fn recip(self) -> Self {
                self.recip()
            }
            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
            #[inline]
            fn round(self) -> Self {
                self.round()
            }
            #[inline]
            fn round_ties_even(self) -> Self {
                self.round_ties_even()
            }
            #[inline]
            fn signum(self) -> Self {
                self.signum()
            }
            #[inline]
            fn sin(self) -> Self {
                self.sin()
            }
            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }
            #[inline]
            fn sinh(self) -> Self {
                self.sinh()
            }
            #[inline]
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            #[inline]
            fn tan(self) -> Self {
                self.tan()
            }
            #[inline]
            fn tanh(self) -> Self {
                self.tanh()
            }
            #[inline]
            fn to_degrees(self) -> Self {
                self.to_degrees()
            }
            #[inline]
            fn to_radians(self) -> Self {
                self.to_radians()
            }
            #[inline]
            fn total_cmp(&self, other: &Self) -> Ordering {
                self.total_cmp(other)
            }
            #[inline]
            fn trunc(self) -> Self {
                self.trunc()
            }
        }
    };
}

pub trait FloatConsts {
    const E: Self;
    const EULER_GAMMA: Self;
    const FRAC_1_PI: Self;
    const FRAC_1_SQRT_2: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const GOLDEN_RATIO: Self;
    const LN_2: Self;
    const LN_10: Self;
    const LOG2_10: Self;
    const LOG2_E: Self;
    const LOG10_2: Self;
    const LOG10_E: Self;
    const PI: Self;
    const SQRT_2: Self;
    const TAU: Self;
}

macro_rules! impl_float_consts {
    ($t:ident) => {
        impl FloatConsts for $t {
            const E: Self = std::$t::consts::E;
            const EULER_GAMMA: Self = std::$t::consts::EULER_GAMMA;
            const FRAC_1_PI: Self = std::$t::consts::FRAC_1_PI;
            const FRAC_1_SQRT_2: Self = std::$t::consts::FRAC_1_SQRT_2;
            const FRAC_2_PI: Self = std::$t::consts::FRAC_2_PI;
            const FRAC_2_SQRT_PI: Self = std::$t::consts::FRAC_2_SQRT_PI;
            const FRAC_PI_2: Self = std::$t::consts::FRAC_PI_2;
            const FRAC_PI_3: Self = std::$t::consts::FRAC_PI_3;
            const FRAC_PI_4: Self = std::$t::consts::FRAC_PI_4;
            const FRAC_PI_6: Self = std::$t::consts::FRAC_PI_6;
            const FRAC_PI_8: Self = std::$t::consts::FRAC_PI_8;
            const GOLDEN_RATIO: Self = std::$t::consts::GOLDEN_RATIO;
            const LN_2: Self = std::$t::consts::LN_2;
            const LN_10: Self = std::$t::consts::LN_10;
            const LOG2_10: Self = std::$t::consts::LOG2_10;
            const LOG2_E: Self = std::$t::consts::LOG2_E;
            const LOG10_2: Self = std::$t::consts::LOG10_2;
            const LOG10_E: Self = std::$t::consts::LOG10_E;
            const PI: Self = std::$t::consts::PI;
            const SQRT_2: Self = std::$t::consts::SQRT_2;
            const TAU: Self = std::$t::consts::TAU;
        }
    };
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_zero_one {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0.0
            }
        }

        impl One for $t {
            fn one() -> Self {
                1.0
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
impl_float_consts!(f32);
impl_float_consts!(f64);
impl_zero_one!(f32);
impl_zero_one!(f64);
