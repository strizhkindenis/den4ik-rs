mod ffi;

pub mod allocator;
pub mod mesh;

pub(crate) trait Unloadable {
    fn unload(item: Self);
}

pub trait Container<T> {
    fn add(&mut self, item: T) -> usize;
    fn remove(&mut self, id: usize) -> bool;
    fn get(&self, id: usize) -> Option<&T>;
    fn get_mut(&mut self, id: usize) -> Option<&mut T>;
}

struct VecConainer<T: Unloadable> {
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

    unsafe fn pop(&mut self, id: usize) -> Option<T> {
        match self.get_mut(id) {
            Some(item) => {
                let item = unsafe { std::mem::transmute_copy(item) };
                self.available.push(id);
                Some(item)
            }
            None => None,
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
        match unsafe { self.pop(id) } {
            Some(item) => {
                T::unload(item);
                true
            }
            None => false,
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
    meshes: VecConainer<mesh::Mesh>,
}

impl RaylibHandle {
    pub fn new() -> Self {
        Self {
            meshes: VecConainer::new(),
        }
    }
}

impl Container<mesh::Mesh> for RaylibHandle {
    fn add(&mut self, item: mesh::Mesh) -> usize {
        self.meshes.add(item)
    }

    fn remove(&mut self, id: usize) -> bool {
        self.meshes.remove(id)
    }

    fn get(&self, id: usize) -> Option<&mesh::Mesh> {
        self.meshes.get(id)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut mesh::Mesh> {
        self.meshes.get_mut(id)
    }
}
