use std::{
    iter,
    mem::MaybeUninit,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
    slice,
};

use crate::traits::{One, Zero};

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

    /// Asserts that the index is within the bounds of the matrix.
    ///
    /// # Panics
    ///
    /// Panics if the `row` or `col` is out of bounds.
    pub fn assert_index(&self, (row, col): (usize, usize)) {
        assert!(
            row < self.rows,
            "invalid row index - `{row} >= {}`",
            self.rows
        );
        assert!(
            col < self.rows,
            "invalid column index - `{col} >= {}`",
            self.cols
        );
    }
}

pub struct Mat<T> {
    data: Box<[T]>,
    dims: Dims,
}

impl<T> Mat<T> {
    #[must_use]
    ///
    /// # Panics
    ///
    /// Panics if `data.len()` does not match `dims.size()`.
    pub fn from_parts(data: Box<[T]>, dims: Dims) -> Self {
        assert_eq!(
            data.len(),
            dims.size(),
            "data.len() does not match dims.size()"
        );
        Self { data, dims }
    }

    #[must_use]
    pub fn into_parts(self) -> (Box<[T]>, Dims) {
        let Self { data, dims } = self;
        (data, dims)
    }

    #[must_use]
    pub fn new_default(dims: Dims) -> Self
    where
        T: Default,
    {
        let mut mat = Self::new_uninit(dims);
        mat.iter_mut().for_each(|x| {
            x.write(T::default());
        });
        unsafe { mat.assume_init() }
    }

    #[must_use]
    pub fn new_filled(dims: Dims, val: T) -> Self
    where
        T: Clone,
    {
        let mut mat = Self::new_uninit(dims);
        let data = mat.as_slice_mut();
        if let Some((last, rest)) = data.split_last_mut() {
            for x in rest.iter_mut() {
                x.write(val.clone());
            }
            last.write(val);
        }
        unsafe { mat.assume_init() }
    }

    #[must_use]
    pub fn new_uninit(dims: Dims) -> Mat<MaybeUninit<T>> {
        let data = Box::new_uninit_slice(dims.size());
        Mat { data, dims }
    }

    pub fn new_filled_with<F>(dims: Dims, mut f: F) -> Mat<T>
    where
        F: FnMut((usize, usize)) -> T,
    {
        let mut mat = Mat::new_uninit(dims);
        for (idx, x) in mat.row_slices_mut().enumerate().flat_map(|(row_idx, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(col_idx, x)| ((row_idx, col_idx), x))
        }) {
            x.write(f(idx));
        }
        unsafe { mat.assume_init() }
    }

    #[must_use]
    pub fn zeros(dims: Dims) -> Self
    where
        T: Zero,
    {
        Self::new_filled_with(dims, |_| T::zero())
    }

    #[must_use]
    pub fn ones(dims: Dims) -> Self
    where
        T: One,
    {
        Self::new_filled_with(dims, |_| T::one())
    }

    #[must_use]
    pub fn eye(dims: Dims) -> Self
    where
        T: One + Zero,
    {
        Self::eye_k(dims, 0)
    }

    #[must_use]
    ///
    /// # Panics
    ///
    /// Panics if the row index cannot be converted to `i64`.
    pub fn eye_k(dims: Dims, k: i64) -> Self
    where
        T: One + Zero,
    {
        let selector = |(row, col): (usize, usize)| {
            let row: i64 = row.try_into().unwrap();
            let col: i64 = col.try_into().unwrap();
            if (row + k) == col {
                T::one()
            } else {
                T::zero()
            }
        };
        Self::new_filled_with(dims, selector)
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    #[must_use]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn row_slices(&self) -> slice::ChunksExact<'_, T> {
        let cols = self.cols();
        self.as_slice().chunks_exact(cols)
    }

    pub fn row_slices_mut(&mut self) -> slice::ChunksExactMut<'_, T> {
        let cols = self.cols();
        self.as_slice_mut().chunks_exact_mut(cols)
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
    pub fn len(&self) -> usize {
        self.dims.size()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn get_index(&self, index: (usize, usize)) -> usize {
        self.dims().assert_index(index);
        let (row, col) = index;
        self.cols() * row + col
    }

    #[must_use]
    pub fn get_row_col(&self, idx: usize) -> (usize, usize) {
        let col = idx % self.cols();
        let row = idx / self.cols();
        (row, col)
    }

    #[must_use]
    pub fn dims(&self) -> Dims {
        self.dims
    }

    /// # Panics
    ///
    /// Panics if the matrix length does not match the expected length.
    pub fn assert_len(&self, len: usize) {
        assert_eq!(self.len(), len, "Matrix sizes must match");
    }

    pub fn assert_len_mat(&self, mat: &Mat<T>) {
        self.assert_len(mat.len());
    }

    /// # Panics
    ///
    /// Panics if the matrix dimensions do not match the expected dimensions.
    pub fn assert_dims(&self, dims: Dims) {
        assert_eq!(self.dims(), dims, "Matrix dimensions must match");
    }

    pub fn assert_dims_mat(&self, mat: &Mat<T>) {
        self.assert_dims(mat.dims());
    }

    /// # Panics
    ///
    /// Panics if the new dimensions size does not match the current matrix length.
    pub fn reshape(&mut self, dims: Dims) {
        self.assert_dims(dims);
        self.dims = dims;
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.into_iter()
    }
}

impl<T> Mat<MaybeUninit<T>> {
    /// # Safety
    ///
    /// The caller must ensure that all elements of the matrix have been initialized.
    #[must_use]
    pub unsafe fn assume_init(self) -> Mat<T> {
        let (data, dims) = self.into_parts();
        let raw = Box::into_raw(data) as *mut [T];
        let data = unsafe { Box::from_raw(raw) };
        Mat::from_parts(data, dims)
    }
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = self.get_index(index);
        &self.as_slice()[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let idx = self.get_index(index);
        &mut self.as_slice_mut()[idx]
    }
}

impl<T: Clone> Clone for Mat<T> {
    fn clone(&self) -> Self {
        Self::from_parts(self.data.clone(), self.dims())
    }
}

impl<'a, T> IntoIterator for &'a Mat<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Mat<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice_mut().iter_mut()
    }
}

impl<T> IntoIterator for Mat<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}

impl<T> From<Vec<T>> for Mat<T> {
    fn from(value: Vec<T>) -> Self {
        let data = value.into_boxed_slice();
        let dims = Dims::new(1, data.len());
        Self { data, dims }
    }
}

impl<T> FromIterator<T> for Mat<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

macro_rules! impl_op_assign_mat {
    ($trait:ident, $method:ident, $t:ty, $mt:ty) => {
        impl<'a, T> $trait<$mt> for Mat<T>
        where
            T: $trait<$t> + Copy,
        {
            fn $method(&mut self, mat: $mt) {
                self.assert_dims(mat.dims());
                iter::zip(self, mat).for_each(|(a, b)| a.$method(b));
            }
        }
    };
}

macro_rules! impl_op_assign_t {
    ($trait:ident, $method:ident, $t:ty) => {
        impl<'a, T> $trait<$t> for Mat<T>
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
        impl_op_assign_mat!($trait, $method, &'a T, &'a Mat<T>);
        impl_op_assign_mat!($trait, $method, T, Mat<T>);
        impl_op_assign_t!($trait, $method, &'a T);
        impl_op_assign_t!($trait, $method, T);
    };
}

impl_op_assign!(AddAssign, add_assign);
impl_op_assign!(SubAssign, sub_assign);
impl_op_assign!(MulAssign, mul_assign);
impl_op_assign!(DivAssign, div_assign);
impl_op_assign!(RemAssign, rem_assign);

macro_rules! impl_op_body_mat {
    ($trait:ident, $method:ident, $ta:ty, $tb:ty, $mta: ty, $mtb:ty) => {
        impl<'a, 'b, T> $trait<$mtb> for $mta
        where
            $ta: $trait<$tb, Output = T>,
        {
            type Output = Mat<T>;
            fn $method(self, mat: $mtb) -> Self::Output {
                self.assert_dims(mat.dims());
                let mut res = Mat::new_uninit(mat.dims());
                let method_iter = iter::zip(self, mat).map(|(a, b)| a.$method(b));
                iter::zip(res.iter_mut(), method_iter).for_each(|(a, b)| {
                    a.write(b);
                });
                unsafe { res.assume_init() }
            }
        }
    };
}

macro_rules! impl_op_body_t {
    ($trait:ident, $method:ident, $ta:ty, $tb:ty, $mta: ty) => {
        impl<'a, 'b, T> $trait<$tb> for $mta
        where
            $ta: $trait<$tb, Output = T>,
            T: Copy,
        {
            type Output = Mat<T>;
            fn $method(self, val: $tb) -> Self::Output {
                let mut res = Mat::new_uninit(self.dims());
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
        impl_op_body_mat!($trait, $method, &'a T, &'b T, &'a Mat<T>, &'b Mat<T>);
        impl_op_body_mat!($trait, $method, &'a T, T, &'a Mat<T>, Mat<T>);
        impl_op_body_mat!($trait, $method, T, &'b T, Mat<T>, &'b Mat<T>);
        impl_op_body_mat!($trait, $method, T, T, Mat<T>, Mat<T>);
        impl_op_body_t!($trait, $method, &'a T, &'b T, &'a Mat<T>);
        impl_op_body_t!($trait, $method, &'a T, T, &'a Mat<T>);
        impl_op_body_t!($trait, $method, T, &'b T, Mat<T>);
        impl_op_body_t!($trait, $method, T, T, Mat<T>);
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);
impl_op!(Rem, rem);
