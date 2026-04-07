use std::{
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    slice,
};

use crate::float::{One, Zero};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Dims {
    rows: usize,
    cols: usize,
}

impl From<(usize, usize)> for Dims {
    fn from((rows, cols): (usize, usize)) -> Self {
        Self::new(rows, cols)
    }
}

impl From<usize> for Dims {
    fn from(n: usize) -> Self {
        Self::new(n, n)
    }
}

impl Dims {
    #[must_use]
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.rows * self.cols
    }
}

pub struct Mat<T> {
    data: Box<[T]>,
    dims: Dims,
}

impl<T> Mat<T> {
    fn from_parts(data: Box<[T]>, dims: Dims) -> Self {
        Self { data, dims }
    }

    #[must_use]
    pub fn rows(&self) -> usize {
        self.dims.rows
    }

    #[must_use]
    pub fn cols(&self) -> usize {
        self.dims.cols
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn dims(&self) -> Dims {
        self.dims
    }

    pub fn assert_size(&self, mat: &Mat<T>) {
        assert_eq!(self.size(), mat.size(), "Matrix sizes must match");
    }

    pub fn assert_dims(&self, mat: &Mat<T>) {
        assert_eq!(self.dims(), mat.dims(), "Matrix dimensions must match");
    }

    pub fn reshape(&mut self, dims: Dims) {
        assert_eq!(self.size(), dims.size());
        self.dims = dims;
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.into_iter()
    }
}

impl<T: Clone> Mat<T> {
    pub fn full(dims: Dims, val: T) -> Self {
        let data = vec![val; dims.size()].into_boxed_slice();
        Self { data, dims }
    }
}

impl<T: Clone + One> Mat<T> {
    #[must_use]
    pub fn ones(dims: Dims) -> Self {
        Self::full(dims, T::one())
    }
}

impl<T: Clone + Zero> Mat<T> {
    #[must_use]
    pub fn zeros(dims: Dims) -> Self {
        Self::full(dims, T::zero())
    }
}

impl<'a, T> IntoIterator for &'a Mat<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Mat<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> IntoIterator for Mat<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}

impl<T> FromIterator<T> for Mat<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data = Vec::from_iter(iter).into_boxed_slice();
        let dims = Dims::new(data.len(), 1);
        Self { data, dims }
    }
}

macro_rules! impl_op_assign {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<'a, T: $trait<&'a T>> $trait<&'a Mat<T>> for Mat<T> {
            fn $method(&mut self, mat: &'a Self) {
                self.assert_dims(mat);
                iter::zip(self, mat).for_each(|(a, b)| *a $op b);
            }
        }

        impl<'a, T: $trait<&'a T>> $trait<&'a T> for Mat<T> {
            fn $method(&mut self, val: &'a T) {
                self.iter_mut().for_each(|a| *a $op val);
            }
        }

        impl<T: $trait<T>> $trait<Mat<T>> for Mat<T> {
            fn $method(&mut self, mat: Self) {
                self.assert_dims(&mat);
                iter::zip(self, mat).for_each(|(a, b)| *a $op b);
            }
        }
    };
}

impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);
impl_op_assign!(MulAssign, mul_assign, *=);
impl_op_assign!(DivAssign, div_assign, /=);
impl_op_assign!(RemAssign, rem_assign, %=);

macro_rules! impl_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<'a, 'b, T> $trait<&'b Mat<T>> for &'a Mat<T>
        where
            &'a T: $trait<&'b T, Output = T>,
        {
            type Output = Mat<T>;
            fn $method(self, mat: &'b Mat<T>) -> Self::Output {
                self.assert_dims(mat);
                let dims = self.dims;
                let data = iter::zip(self, mat).map(|(a, b)| a $op b).collect();
                Mat::from_parts(data, dims)
            }
        }

        impl<'a, T> $trait<&'a Mat<T>> for Mat<T>
        where
            T: $trait<&'a T, Output = T>,
        {
            type Output = Mat<T>;
            fn $method(self, mat: &'a Mat<T>) -> Self::Output {
                self.assert_dims(mat);
                let dims = self.dims;
                let data = iter::zip(self, mat).map(|(a, b)| a $op b).collect();
                Mat::from_parts(data, dims)
            }
        }

        impl<T> $trait<Mat<T>> for Mat<T>
        where
            T: $trait<T, Output = T>,
        {
            type Output = Mat<T>;
            fn $method(self, mat: Mat<T>) -> Self::Output {
                self.assert_dims(&mat);
                let dims = self.dims;
                let data = iter::zip(self, mat).map(|(a, b)| a $op b).collect();
                Mat::from_parts(data, dims)
            }
        }

        impl<'a, T> $trait<Mat<T>> for &'a Mat<T>
        where
            &'a T: $trait<T, Output = T>,
        {
            type Output = Mat<T>;
            fn $method(self, mat: Mat<T>) -> Self::Output {
                self.assert_dims(&mat);
                let dims = self.dims;
                let data = iter::zip(self, mat).map(|(a, b)| a $op b).collect();
                Mat::from_parts(data, dims)
            }
        }
    };
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);
impl_op!(Rem, rem, %);
