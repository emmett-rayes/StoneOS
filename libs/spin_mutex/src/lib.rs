#![no_std]

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct MutexError<'a>(&'a str);

pub struct SpinMutex<T: ?Sized> {
    lock: AtomicBool,
    inner: UnsafeCell<T>,
}

impl<T> SpinMutex<T> {
    pub fn new(value: T) -> SpinMutex<T> {
        SpinMutex {
            lock: AtomicBool::new(false),
            inner: UnsafeCell::new(value),
        }
    }

    pub fn try_lock(&self) -> Result<SpinMutexGuard<T>, MutexError> {
        if !self.lock.swap(true, Ordering::Acquire) {
            Ok(SpinMutexGuard { mutex: self })
        } else {
            Err(MutexError("Mutex is already locked."))
        }
    }

    pub fn lock(&self) -> SpinMutexGuard<T> {
        loop {
            if let Ok(mutex_guard) = self.try_lock() {
                return mutex_guard;
            }
            core::hint::spin_loop();
        }
    }
}

unsafe impl<T: ?Sized + Send> Send for SpinMutex<T> {}

unsafe impl<T: ?Sized + Send> Sync for SpinMutex<T> {}

impl<T: ?Sized> Drop for SpinMutex<T> {
    fn drop(&mut self) {
        unsafe {
            self.inner.get().drop_in_place();
        }
    }
}

pub struct SpinMutexGuard<'a, T: ?Sized + 'a> {
    mutex: &'a SpinMutex<T>,
}

unsafe impl<T: ?Sized + Send> Send for SpinMutexGuard<'_, T> {}

unsafe impl<T: ?Sized + Sync> Sync for SpinMutexGuard<'_, T> {}

impl<T: ?Sized> Drop for SpinMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.lock.swap(false, Ordering::Release);
    }
}

impl<T> Deref for SpinMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.inner.get() }
    }
}

impl<T> DerefMut for SpinMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.inner.get() }
    }
}
