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
    fn remove(&mut self, id: Id) -> Result<(), crate::RaylibError>;
    fn take(&mut self, id: Id) -> Result<T, crate::RaylibError>;
    fn get(&self, id: Id) -> Result<&T, crate::RaylibError>;
    fn get_mut(&mut self, id: Id) -> Result<&mut T, crate::RaylibError>;
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

    fn remove(&mut self, id: Id) -> Result<(), crate::RaylibError> {
        let index = id.to_usize();
        if index < self.items.len() && !self.available.contains(&id) {
            let mut empty = unsafe { std::mem::zeroed() };
            std::mem::swap(&mut self.items[index], &mut empty);
            T::unload(empty);
            self.available.push(id);
            Ok(())
        } else {
            Err(crate::RaylibError::InvalidId(std::any::type_name::<Id>()))
        }
    }

    fn take(&mut self, id: Id) -> Result<T, crate::RaylibError> {
        let index = id.to_usize();
        if index < self.items.len() && !self.available.contains(&id) {
            let mut value = unsafe { std::mem::zeroed() };
            std::mem::swap(&mut self.items[index], &mut value);
            self.available.push(id);
            Ok(value)
        } else {
            Err(crate::RaylibError::InvalidId(std::any::type_name::<Id>()))
        }
    }

    fn get(&self, id: Id) -> Result<&T, crate::RaylibError> {
        if self.available.contains(&id) {
            return Err(crate::RaylibError::InvalidId(std::any::type_name::<Id>()));
        }
        self.items
            .get(id.to_usize())
            .ok_or(crate::RaylibError::InvalidId(std::any::type_name::<Id>()))
    }

    fn get_mut(&mut self, id: Id) -> Result<&mut T, crate::RaylibError> {
        if self.available.contains(&id) {
            return Err(crate::RaylibError::InvalidId(std::any::type_name::<Id>()));
        }
        self.items
            .get_mut(id.to_usize())
            .ok_or(crate::RaylibError::InvalidId(std::any::type_name::<Id>()))
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
