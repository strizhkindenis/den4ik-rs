use std::{
	alloc::{alloc, dealloc, handle_alloc_error, Layout},
	ptr::NonNull,
	iter::{self, IntoIterator},
	slice,
};


fn layoutnf64(n: usize) -> Layout {
	Layout::array::<f64>(n).unwrap()
}

fn allocnf64(n: usize) -> NonNull<f64> {
	let layout = layoutnf64(n);
	let p = unsafe { alloc(layout) };
	if p.is_null() {
		handle_alloc_error(layout);
	}
	NonNull::new(p.cast()).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index(pub [usize; 2]);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dims(pub [usize; 2]);

impl Dims {
	pub fn size(self) -> usize {
		self.0[0] * self.0[1]
	}
	
	pub fn get_idx(self, idx: Index) -> usize {
		assert!(idx.0[0] < self.0[0]);
		assert!(idx.0[1] < self.0[1]);
		idx.0[0] * self.0[1] + idx.0[1]	
	}
	
	pub fn t(self) -> Self {
		Self([self.0[1], self.0[0]])	
	}
}

pub struct MatF64 {
	p: NonNull<f64>,
	dims: Dims,
}

impl Drop for MatF64 {
	fn drop(&mut self) {
		let layout = layoutnf64(self.len());
		let p = self.as_ptr_mut().cast();
		unsafe { dealloc(p, layout) };	
	}
}

impl MatF64 {
	pub unsafe fn unew(dims: Dims) -> Self {
		let p = allocnf64(dims.size());
		Self { p, dims }
	}
	
	pub fn apply_fn<F>(&mut self, mut f: F)
	where F: FnMut(&mut f64) 
	{
		self.apply_fn_idx(|x, _| f(x));	
	}
	
	pub fn apply_fn_idx<F>(&mut self, mut f: F)
	where F: FnMut(&mut f64, Index)
	{
		for i0 in 0..self.dims.0[0] {
			for i1 in 0..self.dims.0[1] {
				let idx = Index([i0, i1]);
				f(self.at_mut(idx), idx);
			}
		}
	}
	
	pub fn from_esiter<I>(dims: Dims, iter: I) -> Self
	where
		I: IntoIterator<Item = f64>, 
		I::IntoIter: ExactSizeIterator 
	{
		let iter = iter.into_iter();
		assert_eq!(iter.len(), dims.size());
		let mut m = unsafe { Self::unew(dims) };
		for (a, b) in iter::zip(m.iter_mut(), iter) {
			*a = b;	
		}
		m
	}
	
	pub fn from_fn<F>(dims: Dims, mut f: F) -> Self
	where F: FnMut() -> f64
	{
		Self::from_fn_idx(dims, |_| f())	
	}
	
	pub fn from_fn_idx<F>(dims: Dims, mut f: F) -> Self
	where F: FnMut(Index) -> f64
	{
		let mut m = unsafe { Self::unew(dims) };		
		m.apply_fn_idx(|x, idx| *x = f(idx));
		m
	}
	
	pub fn from_x(dims: Dims, x: f64) -> Self {
		Self::from_esiter(dims, iter::repeat_n(x, dims.size()))	
	}
	
	pub fn as_ptr(&self) -> *const f64 {
		self.p.as_ptr().cast()	
	}
	
	pub fn as_ptr_mut(&mut self) -> *mut f64 {
		self.p.as_ptr().cast()	
	}
	
	pub fn as_slice(&self) -> &[f64] {
		unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
	}
	
	pub fn as_slice_mut(&mut self) -> &mut [f64] {
		unsafe { slice::from_raw_parts_mut(self.as_ptr_mut(), self.len()) }
	}
	
	pub fn iter(&self) -> slice::Iter<'_, f64> {
		self.as_slice().iter()	
	}
	
	pub fn iter_mut(&mut self) -> slice::IterMut<'_, f64> {
		self.as_slice_mut().iter_mut()	
	}
	
	pub fn at(&self, idx: Index) -> &f64 {
		let i = self.dims.get_idx(idx);
		&self.as_slice()[i]	
	}
	
	pub fn at_mut(&mut self, idx: Index) -> &mut f64 {
		let i = self.dims.get_idx(idx);
		&mut self.as_slice_mut()[i]	
	}
	
	pub fn opp<F>(a: &Self, b: &Self, c: &mut Self, mut f: F)
	where F: FnMut(&f64, &f64, &mut f64)
	{
		assert_eq!(a.dims, b.dims);
		assert_eq!(a.len(), c.len());
		c.dims = a.dims;
		let iter = iter::zip(a.iter(), b.iter());
		for ((a, b), c) in iter::zip(iter, c.iter_mut()) {
			f(a, b, c);	
		}
	}
	
	/// ```
	/// use den4ik_math::f64::{MatF64, Dims};
	/// let dims = Dims([2, 3]);
	/// let a = MatF64::from_esiter(dims, [
	///     1.0, 3.0, 2.0,
	///     4.0, 0.0, 1.0,
	/// ]);
	/// let b = MatF64::from_esiter(dims, [
	///     5.0, 2.0,  1.0,
	///     0.0, 3.0, -2.0,
	/// ]);
	/// let mut mc = MatF64::from_x(dims, 0.0);
	/// let c = MatF64::from_esiter(dims, [
	///     6.0, 5.0,  3.0,
	///     4.0, 3.0, -1.0,
	/// ]);
	/// MatF64::add(&a, &b, &mut mc);
	/// MatF64::assert_eq_eps(&mc, &c, 1e-6);
	/// ```
	pub fn add(a: &Self, b: &Self, c: &mut Self) {
		Self::opp(a, b, c, |a, b, c| *c = a + b);	
	}
	
	/// ```
	/// use den4ik_math::f64::{MatF64, Dims};
	/// let dims = Dims([2, 3]);
	/// let a = MatF64::from_esiter(dims, [
	///     1.0, 3.0, 2.0,
	///     4.0, 0.0, 1.0,
	/// ]);
	/// let b = MatF64::from_esiter(dims, [
	///     5.0, 2.0,  1.0,
	///     0.0, 3.0, -2.0,
	/// ]);
	/// let mut mc = MatF64::from_x(dims, 0.0);
	/// let c = MatF64::from_esiter(dims, [
	///     -4.0, 1.0,  1.0,
	///      4.0, -3.0, 3.0,
	/// ]);
	/// MatF64::sub(&a, &b, &mut mc);
	/// MatF64::assert_eq_eps(&mc, &c, 1e-6);
	/// ```
	pub fn sub(a: &Self, b: &Self, c: &mut Self) {
		Self::opp(a, b, c, |a, b, c| *c = a - b);	
	}
	
	pub fn mul(a: &Self, b: &Self, c: &mut Self) {
		Self::opp(a, b, c, |a, b, c| *c = a * b);	
	}
	
	pub fn div(a: &Self, b: &Self, c: &mut Self) {
		Self::opp(a, b, c, |a, b, c| *c = a / b);	
	}
	
	pub fn t(&self, t: &mut Self) {
		assert_eq!(self.len(), t.len());
		t.dims = self.dims.t();
		t.apply_fn_idx(|t, idx| *t = *self.at(Index([idx.0[1], idx.0[0]])));	
	}
	
	pub fn dot(a: &Self, b: &Self, c: &mut Self) {
		assert_eq!(a.dims.0[1], b.dims.0[0]);
		let dims = Dims([a.dims.0[0], b.dims.0[1]]);
		assert_eq!(dims.size(), c.len());
		let mut bt = unsafe { Self::unew(b.dims) };
		b.t(&mut bt);
		c.dims = dims;
		c.apply_fn_idx(|c, idx| {
			let a_start = idx.0[0] * a.dims.0[1];
			let a_end = a_start + a.dims.0[1];
			let bt_start = idx.0[1] * bt.dims.0[1];
			let bt_end = bt_start + bt.dims.0[1];
			let sa = &a.as_slice()[a_start..a_end];
			let sbt = &bt.as_slice()[bt_start..bt_end];
			*c = sa.iter().zip(sbt.iter()).map(|(a, bt)| a * bt).sum();	
		});
	}
	
	pub fn len(&self) -> usize {
		self.dims.size()
	}
	
	pub fn assert_eq_eps(a: &Self, b: &Self, eps: f64) {
		assert!(eps > 0.0, "{eps} <= 0");
		assert_eq!(a.dims, b.dims, "a.dims({:?}) != b.dims({:?})", a.dims, b.dims);
		Self::from_fn_idx(a.dims, |idx| {
			let a = a.at(idx);
			let b = b.at(idx);
			let err = (a - b).abs();
			assert!(err < eps, "at {idx:?}: {a} != {b}");  
			err
		});
	}
}
