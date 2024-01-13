use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::memory::address::Address;

pub struct DerefWrapper<T> {
    address: usize,
    phantom: PhantomData<T>,
}

impl<T> DerefWrapper<T> {
    pub fn new(address: impl Address) -> Self {
        DerefWrapper {
            address: address.raw_address(),
            phantom: PhantomData,
        }
    }
}

impl<T> Deref for DerefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.address as *const T) }
    }
}

impl<T> DerefMut for DerefWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.address as *mut T) }
    }
}
