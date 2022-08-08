use core::{task::{Waker, Poll}, future::Future, ops::{Deref, DerefMut}, cell::UnsafeCell};

use alloc::collections::LinkedList;

use crate::sync::SyncCell;

pub struct Mutex<T> {
    val: UnsafeCell<T>,
    info: SyncCell<MutexInfo>
}

impl<T> Mutex<T> {
    pub fn new(val: T) -> Self {
        Mutex {
            val: UnsafeCell::new(val),
            info: SyncCell::new(MutexInfo {
                locked: false,
                queue: LinkedList::new()
            })
        }
    }

    pub fn lock(&self) -> LockFuture<T> {
        LockFuture {
            mutex: self
        }
    }
}

struct MutexInfo {
    locked: bool,
    queue: LinkedList<Waker>
}

pub struct LockFuture<'a, T> {
    mutex: &'a Mutex<T>
}

impl<'a, T> Future for LockFuture<'a, T> {
    type Output = MutexGuard<'a, T>;

    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<Self::Output> {
        let mut info = self.mutex.info.get();
        if info.locked {
            info.queue.push_back(cx.waker().clone());
            Poll::Pending
        } else {
            info.locked = true;
            Poll::Ready(MutexGuard {
                mutex: self.mutex
            })
        }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*self.mutex.val.get()
        }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *self.mutex.val.get()
        }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        let mut info = self.mutex.info.get();
        info.locked = false;
        if let Some(waker) = info.queue.pop_front() {
            waker.wake();
        }
    }
}
