use std::{
    iter,
    mem::{self, MaybeUninit},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    ptr::{self, NonNull},
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
}

pub struct Mat<T> {
    data: NonNull<T>,
    dims: Dims,
}

impl<T> Mat<T> {
    /// SAFETY: data shouold be a valid pointer that matches dims.size()
    unsafe fn from_raw_parts(data: *mut T, dims: Dims) -> Self {
        let data = NonNull::new(data).expect("Poisslbe null pointer");
        Self::from_parts(data, dims)
    }

    fn into_raw_parts(self) -> (*mut T, Dims) {
        let (data, dims) = self.into_parts();
        (data.as_ptr(), dims)
    }

    fn from_parts(data: NonNull<T>, dims: Dims) -> Self {
        Self { data, dims }
    }

    fn into_parts(self) -> (NonNull<T>, Dims) {
        let Mat { data, dims } = self;
        (data, dims)
    }

    fn from_box_parts(data: Box<[T]>, dims: Dims) -> Self {
        debug_assert_eq!(data.len(), dims.size());
        let data = Box::into_raw(data).cast();
        // SAFETY: Box<[T]> has a valid *mut T pointer
        unsafe { Self::from_raw_parts(data, dims) }
    }

    fn into_box_parts(self) -> (Box<[T]>, Dims) {
        let (ptr, dims) = self.into_raw_parts();
        let ptr = ptr::slice_from_raw_parts_mut(ptr, dims.size());
        let data = unsafe { Box::from_raw(ptr) };
        (data, dims)
    }

    pub fn new_uninit(dims: Dims) -> Mat<MaybeUninit<T>> {
        let data = Box::new_uninit_slice(dims.size());
        Mat::from_box_parts(data, dims)
    }

    pub fn new_filled_with<F>(mut f: F, dims: Dims) -> Mat<T>
    where
        F: FnMut((usize, usize)) -> T,
    {
        let mut mat = Mat::new_uninit(dims);
        (0..dims.rows)
            .flat_map(|row| (0..dims.cols).map(move |col| (row, col)))
            .zip(mat.iter_mut())
            .for_each(|(idx, x)| {
                x.write(f(idx));
            });
        unsafe { mat.assume_init() }
    }

    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn as_ptr_mut(&self) -> *mut T {
        self.data.as_ptr()
    }

    pub fn as_slice(&self) -> &[T] {
        let ptr = self.as_ptr();
        let len = self.len();
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        let ptr = self.as_ptr_mut();
        let len = self.len();
        unsafe { slice::from_raw_parts_mut(ptr, len) }
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        self.cols() * row + col
    }

    pub fn get_row_col(&self, idx: usize) -> (usize, usize) {
        let col = idx % self.cols();
        let row = idx / self.cols();
        (row, col)
    }

    #[must_use]
    pub fn dims(&self) -> Dims {
        self.dims
    }

    pub fn assert_len(&self, len: usize) {
        assert_eq!(self.len(), len, "Matrix sizes must match");
    }

    pub fn assert_len_mat(&self, mat: &Mat<T>) {
        self.assert_len(mat.len());
    }

    pub fn assert_dims(&self, dims: Dims) {
        assert_eq!(self.dims(), dims, "Matrix dimensions must match");
    }

    pub fn assert_dims_mat(&self, mat: &Mat<T>) {
        self.assert_dims(mat.dims())
    }

    pub fn reshape(&mut self, dims: Dims) {
        assert_eq!(self.len(), dims.size());
        self.dims = dims;
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.into_iter()
    }
}

impl<T> Drop for Mat<T> {
    fn drop(&mut self) {
        let ptr = self.as_ptr_mut();
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

impl<T> Mat<MaybeUninit<T>> {
    pub unsafe fn assume_init(self) -> Mat<T> {
        unsafe { mem::transmute(self) }
    }
}

impl<T: Default> Mat<T> {
    pub fn new_default(dims: Dims) -> Self {
        let mut mat = Self::new_uninit(dims);
        mat.iter_mut().for_each(|x| {
            x.write(T::default());
        });
        unsafe { mat.assume_init() }
    }
}

impl<T: Clone> Clone for Mat<T> {
    fn clone(&self) -> Self {
        let len = self.len();
        let ptr = self.as_ptr_mut();
        let ptr = ptr::slice_from_raw_parts_mut(ptr, len);
        let boxed = unsafe { Box::from_raw(ptr) };
        let data = boxed.clone();
        mem::forget(boxed);
        Self::from_box_parts(data, self.dims)
    }
}

impl<T: Clone> Mat<T> {
    pub fn new_filled(dims: Dims, val: T) -> Self {
        let mut mat = Self::new_uninit(dims);
        let mut data = mat.as_slice_mut();
        let last = data.split_off_last_mut();
        data.iter_mut().for_each(|x| {
            x.write(val.clone());
        });
        if let Some(x) = last {
            x.write(val);
        }
        unsafe { mat.assume_init() }
    }
}

impl<T: Clone + One> Mat<T> {
    #[must_use]
    pub fn ones(dims: Dims) -> Self {
        Self::new_filled(dims, T::one())
    }
}

impl<T: Clone + Zero> Mat<T> {
    #[must_use]
    pub fn zeros(dims: Dims) -> Self {
        Self::new_filled(dims, T::zero())
    }
}

impl<'a, T> IntoIterator for &'a Mat<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Mat<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice_mut().into_iter()
    }
}

impl<T> IntoIterator for Mat<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let (ptr, dims) = self.into_raw_parts();
        let len = dims.size();
        let v = unsafe { Vec::from_raw_parts(ptr, len, len) };
        v.into_iter()
    }
}

impl<T> From<Vec<T>> for Mat<T> {
    fn from(value: Vec<T>) -> Self {
        let data = value.into_boxed_slice();
        let dims = Dims::new(1, data.len());
        Self::from_box_parts(data, dims)
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
