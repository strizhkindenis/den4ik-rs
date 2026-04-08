pub trait Zero {
    #[must_use]
    fn zero() -> Self;
}

pub trait One {
    #[must_use]
    fn one() -> Self;
}

macro_rules! impl_zero_one_float {
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

impl_zero_one_float!(f32);
impl_zero_one_float!(f64);

macro_rules! impl_zero_one_int {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }

        impl One for $t {
            fn one() -> Self {
                1
            }
        }
    };
}

impl_zero_one_int!(u8);
impl_zero_one_int!(u16);
impl_zero_one_int!(u32);
impl_zero_one_int!(u64);
impl_zero_one_int!(u128);
impl_zero_one_int!(usize);
impl_zero_one_int!(i8);
impl_zero_one_int!(i16);
impl_zero_one_int!(i32);
impl_zero_one_int!(i64);
impl_zero_one_int!(i128);
impl_zero_one_int!(isize);
