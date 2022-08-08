use core::{cell::UnsafeCell, arch::asm, ops::{Deref, DerefMut}};

pub fn block_irq<F: FnOnce() -> R, R>(func: F) -> R {
	unsafe {
		let primask: usize;
		core::arch::asm!("mrs {}, PRIMASK", out(reg) primask);
		core::arch::asm!("cpsid i");
		let res = func();
		if primask == 0 {
			core::arch::asm!("cpsie i");
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
			let primask: usize;
			asm!("mrs {}, PRIMASK", out(reg) primask);
			asm!("cpsid i");
			SyncGuard {
				ptr: self.0.get(),
				need_unlock: primask == 0
			}
		}
	}
}

pub struct SyncGuard<T> {
	ptr: *mut T,
	need_unlock: bool
}

impl<T> Deref for SyncGuard<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		unsafe {
			&*self.ptr
		}
	}
}

impl<T> DerefMut for SyncGuard<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe {
			&mut *self.ptr
		}
	}
}

impl<T> Drop for SyncGuard<T> {
	fn drop(&mut self) {
		if self.need_unlock {
			todo!()
		}
	}
}
