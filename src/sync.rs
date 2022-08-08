use core::{
	cell::UnsafeCell,
	arch::asm,
	ops::{Deref, DerefMut},
	sync::atomic::{AtomicU8, Ordering}
};

static LOCK_COUNT: AtomicU8 = AtomicU8::new(0);

pub fn block_irq<F: FnOnce() -> R, R>(func: F) -> R {
	unsafe {
		core::arch::asm!("cpsid i");
		let count = LOCK_COUNT.load(Ordering::Acquire);
		LOCK_COUNT.store(count + 1, Ordering::Release);
		LOCK_COUNT.store(count, Ordering::Relaxed);
		let res = func();
		if count == 0 {
			asm!("cpsie i");
		}
		res
	}
}

#[repr(transparent)]
pub struct SyncCell<T>(UnsafeCell<T>);

impl<T> SyncCell<T> {
	pub const fn new(val: T) -> Self {
		SyncCell(UnsafeCell::new(val))
	}

	pub fn get(&self) -> SyncGuard<T> {
		unsafe {
			asm!("cpsid i");
		}
		let count = LOCK_COUNT.load(Ordering::Acquire);
		LOCK_COUNT.store(count + 1, Ordering::Release);
		SyncGuard(self.0.get())
	}
}

unsafe impl<T> Sync for SyncCell<T> {}

pub struct SyncGuard<T>(*mut T);

impl<T> Deref for SyncGuard<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			&*self.0
		}
	}
}

impl<T> DerefMut for SyncGuard<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			&mut *self.0
		}
	}
}

impl<T> Drop for SyncGuard<T> {
	fn drop(&mut self) {
		let count = LOCK_COUNT.load(Ordering::Acquire);
		LOCK_COUNT.store(count - 1, Ordering::Release);
		if count == 1 {
			unsafe {
				asm!("cpsie i");
			}
		}
	}
}
