use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

pub struct PhysicalAddress<T> {
    raw_address: usize,
    phantom: PhantomData<T>,
}

impl<T> PhysicalAddress<T> {
    pub unsafe fn new(raw_address: usize) -> PhysicalAddress<T> {
        PhysicalAddress {
            raw_address,
            phantom: PhantomData,
        }
    }
}

impl<T> Deref for PhysicalAddress<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.raw_address as *const T) }
    }
}

impl<T> DerefMut for PhysicalAddress<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.raw_address as *mut T) }
    }
}
