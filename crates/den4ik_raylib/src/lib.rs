mod ffi;

pub mod allocator;
pub mod material;
pub mod mesh;
pub mod model;

pub(crate) trait Unloadable {
    fn unload(item: Self);
}

pub trait Container<T> {
    fn add(&mut self, item: T) -> usize;
    fn remove(&mut self, id: usize) -> bool;
    unsafe fn take(&mut self, id: usize) -> Option<T>;
    fn get(&self, id: usize) -> Option<&T>;
    fn get_mut(&mut self, id: usize) -> Option<&mut T>;
}

pub(crate) struct VecConainer<T: Unloadable> {
    items: Vec<T>,
    available: Vec<usize>,
}

impl<T: Unloadable> VecConainer<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            available: Vec::new(),
        }
    }
}

impl<T: Unloadable> Container<T> for VecConainer<T> {
    fn add(&mut self, item: T) -> usize {
        match self.available.pop() {
            Some(id) => {
                self.items[id] = item;
                id
            }
            None => {
                let id = self.items.len();
                self.items.push(item);
                id
            }
        }
    }

    fn remove(&mut self, id: usize) -> bool {
        match unsafe { self.take(id) } {
            Some(item) => {
                T::unload(item);
                true
            }
            None => false,
        }
    }

    unsafe fn take(&mut self, id: usize) -> Option<T> {
        match self.get_mut(id) {
            Some(item) => {
                let item = unsafe { std::mem::transmute_copy(item) };
                self.available.push(id);
                Some(item)
            }
            None => None,
        }
    }

    fn get(&self, id: usize) -> Option<&T> {
        self.items.get(id)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.items.get_mut(id)
    }
}

impl<T: Unloadable> Drop for VecConainer<T> {
    fn drop(&mut self) {
        for (id, item) in self.items.drain(..).enumerate() {
            if !self.available.contains(&id) {
                T::unload(item);
            }
        }
    }
}

pub struct RaylibHandle {
    pub(crate) materials: VecConainer<material::Material>,
    pub(crate) meshes: VecConainer<mesh::Mesh>,
    pub(crate) models: VecConainer<model::Model>,
}

impl RaylibHandle {
    pub fn new() -> Self {
        Self {
            materials: VecConainer::new(),
            meshes: VecConainer::new(),
            models: VecConainer::new(),
        }
    }
}
