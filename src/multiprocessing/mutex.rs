extern crate alloc;

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};
use core::hint;

pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: The mutex ensures that only one mutable reference exists at a time.
unsafe impl<T: Send> Sync for Mutex<T> {}
unsafe impl<T: Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    /// Creates a new mutex wrapping the provided data.
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the mutex, spinning until the lock is available.
    pub fn lock(&self) -> MutexGuard<T> {
        // Attempt to acquire the lock with a compare_exchange.
        while self.lock.compare_exchange_weak(
            false, 
            true, 
            Ordering::Acquire, 
            Ordering::Relaxed
        ).is_err() {
            // Hint to the processor that we are in a spin-loop.
            hint::spin_loop();
        }
        MutexGuard { mutex: self }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Safety: The lock guarantees exclusive access.
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: The lock guarantees exclusive mutable access.
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // Release the lock when the guard is dropped.
        self.mutex.lock.store(false, Ordering::Release);
    }
}

