use std::sync::atomic::AtomicBool; 
use std::sync::atomic::Ordering::{Relaxed, Acquire, Release}; 
use std::cell::UnsafeCell; 
use std::ops::{Deref, DerefMut}; 

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>, 
}

impl<T> Deref for Guard<'_, T> {
    type Target = T; 
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release); 
    }
}

unsafe impl<T> Send for Guard<'_, T> where T: Send {} 
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {} 


pub struct SpinLock<T> {
    locked: AtomicBool, 
    value: UnsafeCell<T>, 
}

impl<T> SpinLock<T> {
    pub const fn new(val: T) -> Self {
        Self { 
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(val),  
        }
    }

    pub fn lock<'a>(&'a self) -> Guard<T> {
        while self.locked.compare_exchange_weak(
            false, 
            true, 
            Acquire, 
            Relaxed
        ).is_err() {
            std::hint::spin_loop(); 
        }
        Guard { lock: self }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release); 
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let spin = SpinLock::new(Vec::new()); 
        std::thread::scope(|s| {
            s.spawn(|| spin.lock().push(1)); 
            s.spawn(|| {
                let mut g = spin.lock(); 
                g.push(2); 
                g.push(2); 
            }); 
        }); 

        let g = spin.lock(); 
        assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]); 
    }
}
