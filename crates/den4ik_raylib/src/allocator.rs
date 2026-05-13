use crate::RaylibHandle;
use std::alloc::Layout;

pub struct Allocator<'l> {
    _handle: &'l RaylibHandle,
}

impl<'l> Allocator<'l> {
    pub fn new(handle: &'l RaylibHandle) -> Self {
        Self { _handle: handle }
    }

    pub unsafe fn alloc(&self, layout: Layout) -> Result<*mut u8, crate::RaylibError> {
        let size = layout
            .size()
            .try_into()
            .map_err(|_| crate::RaylibError::AllocatorSizeTooBig)?;
        if size == 0 {
            return Err(crate::RaylibError::AllocatorZeroSize);
        }
        let ptr: *mut u8 = unsafe { crate::ffi::MemAlloc(size).cast() };
        if ptr.is_null() {
            Err(crate::RaylibError::AllocatorOOM)
        } else {
            Ok(ptr)
        }
    }

    pub unsafe fn free(&self, ptr: *mut u8) {
        unsafe { crate::ffi::MemFree(ptr.cast()) }
    }
}

impl<'l> Allocator<'l> {
    pub fn alloc_default<T: Default>(&self) -> Result<*mut T, crate::RaylibError> {
        let layout = Layout::new::<T>();
        let ptr: *mut T = unsafe { self.alloc(layout) }?.cast();
        unsafe { ptr.write(T::default()) }
        Ok(ptr)
    }

    pub fn alloc_default_array<T: Default>(&self, n: usize) -> Result<*mut T, crate::RaylibError> {
        let layout = Layout::array::<T>(n).map_err(|_| crate::RaylibError::AllocatorSizeTooBig)?;
        let ptr: *mut T = unsafe { self.alloc(layout) }?.cast();
        unsafe { std::slice::from_raw_parts_mut(ptr, n) }.fill_with(T::default);
        Ok(ptr)
    }
}
