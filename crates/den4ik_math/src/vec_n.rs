use std::{
    array,
    iter::{self, Sum},
    mem::{self, MaybeUninit},
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem,
        RemAssign, Sub, SubAssign,
    },
    ptr, slice,
};

use crate::{One, Zero};

macro_rules! impl_view {
    ($t:ident; $($comp:ident),*) => {
        #[repr(C)]
        pub struct $t<T> {
            $(pub $comp: T),*
        }
    };
}

#[repr(C)]
pub struct VecN<const N: usize, T> {
    pub coords: [T; N],
}

macro_rules! impl_deref {
    ($t:ident, $tv:ident) => {
        impl<T> Deref for $t<T> {
            type Target = $tv<T>;

            fn deref(&self) -> &Self::Target {
                unsafe { &*ptr::from_ref(self).cast() }
            }
        }

        impl<T> DerefMut for $t<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *ptr::from_mut(self).cast() }
            }
        }
    };
}

macro_rules! impl_vec_n {
    ($t:ident, $n:expr, $tv:ident, $($comp:ident),*) => {
        impl_view!($tv; $($comp),*);
        pub type $t<T> = VecN<$n, T>;
        impl_deref!($t, $tv);
    };
}

impl_vec_n!(Vec1, 1, ViewX, x);
impl_vec_n!(Vec2, 2, ViewXY, x, y);
impl_vec_n!(Vec3, 3, ViewXYZ, x, y, z);
impl_vec_n!(Vec4, 4, ViewXYZW, x, y, z, w);

impl<const N: usize, T> PartialEq for VecN<N, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.coords.eq(&other.coords)
    }
}

impl<const N: usize, T> Eq for VecN<N, T> where T: Eq {}

impl<const N: usize, T> Default for VecN<N, T>
where
    T: Default,
{
    fn default() -> Self {
        Self::from_fn(|_| T::default())
    }
}

impl<const N: usize, T> Zero for VecN<N, T>
where
    T: Zero,
{
    fn zero() -> Self {
        Self::from_fn(|_| T::zero())
    }
}

impl<const N: usize, T> One for VecN<N, T>
where
    T: One,
{
    fn one() -> Self {
        Self::from_fn(|_| T::one())
    }
}

impl<const N: usize, T> Clone for VecN<N, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::from(self.coords.clone())
    }
}

impl<const N: usize, T> Copy for VecN<N, T> where T: Copy {}

impl<const N: usize, T> From<[T; N]> for VecN<N, T> {
    fn from(coords: [T; N]) -> Self {
        Self { coords }
    }
}

impl<const N: usize, T> Sum for VecN<N, T>
where
    T: Zero + Add<T, Output = T>,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(VecN::zero(), |a, b| a + b)
    }
}

impl<'s, const N: usize, T> Sum<&'s VecN<N, T>> for VecN<N, T>
where
    T: Zero + Add<&'s T, Output = T>,
{
    fn sum<I: Iterator<Item = &'s Self>>(iter: I) -> Self {
        iter.fold(VecN::zero(), |a, b| a + b)
    }
}

impl<const N: usize, T> VecN<N, T> {
    pub fn new_uninit() -> VecN<N, MaybeUninit<T>> {
        VecN::from_fn(|_| MaybeUninit::uninit())
    }

    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        Self::from(array::from_fn(f))
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.coords.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.coords.iter_mut()
    }
}

impl<const N: usize, T> VecN<N, MaybeUninit<T>> {
    pub unsafe fn assume_init(self) -> VecN<N, T> {
        let coords = unsafe { mem::transmute_copy::<_, [T; N]>(&self.coords) };
        VecN::from(coords)
    }
}

impl<const N: usize, T> IntoIterator for VecN<N, T> {
    type Item = T;
    type IntoIter = array::IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.coords.into_iter()
    }
}

impl<'a, const N: usize, T> IntoIterator for &'a VecN<N, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T> IntoIterator for &'a mut VecN<N, T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize, T> Index<usize> for VecN<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.coords[index]
    }
}

impl<const N: usize, T> IndexMut<usize> for VecN<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coords[index]
    }
}

macro_rules! impl_op_assign_vec {
    ($trait:ident, $method:ident, $t:ty, $mt:ty) => {
        impl<'a, const N: usize, T> $trait<$mt> for VecN<N, T>
        where
            T: $trait<$t> + Copy,
        {
            fn $method(&mut self, vec: $mt) {
                iter::zip(self, vec).for_each(|(a, b)| a.$method(b));
            }
        }
    };
}

macro_rules! impl_op_assign_t {
    ($trait:ident, $method:ident, $t:ty) => {
        impl<'a, const N: usize, T> $trait<$t> for VecN<N, T>
        where
            T: $trait<$t> + Copy,
        {
            fn $method(&mut self, val: $t) {
                self.iter_mut().for_each(|a| a.$method(val));
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($trait:ident, $method:ident) => {
        impl_op_assign_vec!($trait, $method, &'a T, &'a VecN<N, T>);
        impl_op_assign_vec!($trait, $method, T, VecN<N, T>);
        impl_op_assign_t!($trait, $method, &'a T);
        impl_op_assign_t!($trait, $method, T);
    };
}

impl_op_assign!(AddAssign, add_assign);
impl_op_assign!(SubAssign, sub_assign);
impl_op_assign!(MulAssign, mul_assign);
impl_op_assign!(DivAssign, div_assign);
impl_op_assign!(RemAssign, rem_assign);

macro_rules! impl_op_body_vec {
    ($trait:ident, $method:ident, $ta:ty, $tb:ty, $vta:ty, $vtb:ty) => {
        impl<'a, 'b, const N: usize, T> $trait<$vtb> for $vta
        where
            $ta: $trait<$tb, Output = T>,
        {
            type Output = VecN<N, T>;
            fn $method(self, vec: $vtb) -> Self::Output {
                let mut res: VecN<N, _> = VecN::new_uninit();
                let method_iter = iter::zip(self, vec).map(|(a, b)| a.$method(b));
                iter::zip(res.iter_mut(), method_iter).for_each(|(a, b)| {
                    a.write(b);
                });
                unsafe { res.assume_init() }
            }
        }
    };
}

macro_rules! impl_op_body_t {
    ($trait:ident, $method:ident, $ta:ty, $tb:ty, $vta:ty) => {
        impl<'a, 'b, const N: usize, T> $trait<$tb> for $vta
        where
            T: Copy,
            $ta: $trait<$tb, Output = T>,
        {
            type Output = VecN<N, T>;
            fn $method(self, val: $tb) -> Self::Output {
                let mut res: VecN<N, _> = VecN::new_uninit();
                let method_iter = self.into_iter().map(|a| a.$method(val));
                iter::zip(res.iter_mut(), method_iter).for_each(|(a, b)| {
                    a.write(b);
                });
                unsafe { res.assume_init() }
            }
        }
    };
}

macro_rules! impl_op {
    ($trait:ident, $method:ident) => {
        impl_op_body_vec!($trait, $method, &'a T, &'b T, &'a VecN<N, T>, &'b VecN<N, T>);
        impl_op_body_vec!($trait, $method, &'a T, T, &'a VecN<N, T>, VecN<N, T>);
        impl_op_body_vec!($trait, $method, T, &'b T, VecN<N, T>, &'b VecN<N, T>);
        impl_op_body_vec!($trait, $method, T, T, VecN<N, T>, VecN<N, T>);
        impl_op_body_t!($trait, $method, &'a T, &'b T, &'a VecN<N, T>);
        impl_op_body_t!($trait, $method, &'a T, T, &'a VecN<N, T>);
        impl_op_body_t!($trait, $method, T, &'b T, VecN<N, T>);
        impl_op_body_t!($trait, $method, T, T, VecN<N, T>);
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);
impl_op!(Rem, rem);

// impl<const N: usize, T: Float> VecN<N, T> {
//     pub fn dot(self, other: Self) -> T {
//         (self * other).into_iter().sum()
//     }

//     pub fn total_cmp(self, other: &Self) -> Ordering {
//         iter::zip(self, other).fold(Ordering::Equal, |ord, (a, b)| match ord {
//             Ordering::Equal => a.total_cmp(b),
//             _ => ord,
//         })
//     }

//     pub fn min(self, other: Self) -> Self {
//         match self.total_cmp(&other) {
//             Ordering::Less | Ordering::Equal => self,
//             Ordering::Greater => other,
//         }
//     }

//     pub fn max(self, other: Self) -> Self {
//         match self.total_cmp(&other) {
//             Ordering::Greater | Ordering::Equal => self,
//             Ordering::Less => other,
//         }
//     }

//     pub fn has_nan(self) -> bool {
//         self.iter().any(|x| x.is_nan())
//     }

//     pub fn iter(&self) -> slice::Iter<'_, T> {
//         self.coords.iter()
//     }

//     pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
//         self.coords.iter_mut()
//     }

//     pub fn length_squared(self) -> T {
//         (self * self).iter().copied().sum()
//     }

//     pub fn length(self) -> T {
//         self.length_squared().sqrt()
//     }

//     pub fn distance_squared(self, other: Self) -> T {
//         (self - other).length_squared()
//     }

//     pub fn distance(self, other: Self) -> T {
//         self.distance_squared(other).sqrt()
//     }

//     pub fn normalize(self) -> Self {
//         let len = self.length();
//         if len > T::zero() { self / len } else { self }
//     }

//     pub fn unique(points: impl IntoIterator<Item = Self>) -> Vec<Self> {
//         let mut points = points.into_iter().collect::<Vec<_>>();
//         points.sort_by(|a, b| a.total_cmp(b));
//         let mut unique = Vec::new();
//         for point in points {
//             if !unique.last().is_some_and(|previous| point.eq(previous)) {
//                 unique.push(point);
//             }
//         }
//         unique
//     }

//     pub fn avg(points: impl IntoIterator<Item = Self>) -> Option<Self> {
//         points
//             .into_iter()
//             .map(|p| (p, 1))
//             .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
//             .map(|(p, cnt)| p / T::from(Into::<f64>::into(cnt)))
//     }
// }

// impl<T: Float> Vec2<T> {
//     pub fn polar_angle(self) -> T {
//         let angle = self.y.atan2(self.x);
//         if angle >= T::zero() {
//             angle
//         } else {
//             T::from(2.0).mul_add(T::PI, angle)
//         }
//     }
// }
