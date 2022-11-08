use core::{sync::atomic::{AtomicBool, Ordering}, cell::UnsafeCell};
use crate::interrupts;

//================================================
//  TYPES
//================================================

/// 
/// Kernel Spin Lock.
///
/// This locks disables local interrupts and, on multiprocessor systems, 
/// additionally locks a global atomic flag.
/// This lock is safe to use in any context.
/// 
pub struct SpinLock<T> {
    //#[cfg(has_two_cores)]
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct SpinLockGuard<'a, T: 'a> {
    lock: &'a SpinLock<T>,
    interrupt_status: bool,
}

//================================================
//  TRAIT IMPLEMENTATIONS
//================================================

impl<'a, T: 'a> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
        if self.interrupt_status { interrupts::enable(); }
    }
}

impl<'a, T: 'a> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.lock.value.get().as_ref().expect("Null pointer in SpinLock") }
    }
}

impl<'a, T: 'a> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.lock.value.get().as_mut().expect("Null pointer in SpinLock") }
    }
}

//================================================
//  IMPLEMENTATIONS
//================================================

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { locked: AtomicBool::new(false), value: UnsafeCell::new(value) }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        let interrupt_status = interrupts::are_enabled();
        interrupts::disable();

        // Spin until the exchange succeeds.
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {}
        
        SpinLockGuard { lock: self, interrupt_status }
    }
}