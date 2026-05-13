use std::{fmt::Debug, hash::Hash};

use crate::Unloadable;

pub trait ContainerId: Sized + Debug + Copy + Clone + Eq + Hash {
    fn to_usize(self) -> usize;
    fn from_usize(value: usize) -> Self;
}

pub trait Container<T, Id>
where
    T: Unloadable,
    Id: ContainerId,
{
    fn add(&mut self, item: T) -> Id;
    fn remove(&mut self, id: Id) -> bool;
    fn take(&mut self, id: Id) -> Option<T>;
    fn get(&self, id: Id) -> Option<&T>;
    fn get_mut(&mut self, id: Id) -> Option<&mut T>;
}

pub struct VecConainer<T: Unloadable, Id: ContainerId> {
    items: Vec<T>,
    available: Vec<Id>,
}

impl<T: Unloadable, Id: ContainerId> VecConainer<T, Id> {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            available: Vec::new(),
        }
    }
}

impl<T: Unloadable, Id: ContainerId> Container<T, Id> for VecConainer<T, Id> {
    fn add(&mut self, item: T) -> Id {
        match self.available.pop() {
            Some(id) => {
                self.items[id.to_usize()] = item;
                id
            }
            None => {
                let id = Id::from_usize(self.items.len());
                self.items.push(item);
                id
            }
        }
    }

    fn remove(&mut self, id: Id) -> bool {
        match unsafe { self.take(id) } {
            Some(item) => {
                T::unload(item);
                true
            }
            None => false,
        }
    }

    fn take(&mut self, id: Id) -> Option<T> {
        match self.get_mut(id) {
            Some(item) => {
                let item = unsafe { std::mem::transmute_copy(item) };
                self.available.push(id);
                Some(item)
            }
            None => None,
        }
    }

    fn get(&self, id: Id) -> Option<&T> {
        self.items.get(id.to_usize())
    }

    fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        self.items.get_mut(id.to_usize())
    }
}

impl<T: Unloadable, Id: ContainerId> Drop for VecConainer<T, Id> {
    fn drop(&mut self) {
        for (id, item) in self.items.drain(..).enumerate() {
            if !self.available.contains(&Id::from_usize(id)) {
                T::unload(item);
            }
        }
    }
}
