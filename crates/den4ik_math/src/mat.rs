use std::{
    iter,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
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
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }
}

pub struct Mat<T> {
    data: Box<[T]>,
    dims: Dims,
}

impl<T> Mat<T> {
    pub fn rows(&self) -> usize {
        self.dims.rows
    }

    pub fn cols(&self) -> usize {
        self.dims.cols
    }

    pub fn size(&self) -> usize {
        self.data.len()
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
    pub fn ones(dims: Dims) -> Self {
        Self::full(dims, T::one())
    }
}

impl<T: Clone + Zero> Mat<T> {
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

impl<T: AddAssign + Clone> AddAssign<&Mat<T>> for Mat<T> {
    fn add_assign(&mut self, rhs: &Self) {
        iter::zip(self, rhs).for_each(|(a, b)| *a += b.clone());
    }
}
