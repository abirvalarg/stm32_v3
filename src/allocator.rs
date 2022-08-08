use core::{mem::size_of, alloc::GlobalAlloc, ptr::null_mut};

pub struct Allocator {
	first_free: *mut Header
}

#[no_mangle]
pub unsafe fn heap_init(start: usize, end: usize) {
	let start = start as *mut Header;
	let tail = (end - size_of::<usize>()) as *mut Header;
	start.set_used(false);
	start.set_len((end - start as usize) / 4 - 2);
	tail.set_len(0);
	ALLOC.first_free = start;
}

unsafe impl GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
		let size = layout.size().max(4);
		let size = if size % 4 != 0 {
			size / 4 + 1
		} else {
			size / 4
		};
		let align = layout.align().max(4);
		let align_mask = align - 1;
		let mut head = self.first_free;
		loop {
			if !head.used() && head.layout_fits(size, align) {
				let aligned = head as usize & !align_mask + align;
				let pad_blocks = (aligned - head as usize) / 4;
				if pad_blocks == 1 {
					head.offset(1).set_len(0);
					head.cut(size + 1);
				} else if pad_blocks > 1 {
					let mid = head.offset(pad_blocks as isize);
					mid.set_len(head.len() - pad_blocks);
					head.set_len(pad_blocks - 1);
					head = mid;
					head.cut(size);
				} else {
					head.cut(size);
				}
				break aligned as *mut u8;
			} else if let Some(next) = head.next_mut() {
				if !next.used() {
					head.set_len(head.len() + next.len() + 1);
				} else {
					head = next;
				}
			} else {
				break null_mut();
			}
		}
	}

	unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
		let mut head = ptr as *mut Header;
		loop {
			head = head.offset(-1);
			if head.len() > 0 {
				break;
			}
		}
		head.set_used(false);
		if (head as usize) < (self.first_free as usize) {
			ALLOC.first_free = head;
		}
	}
}

#[global_allocator]
pub static mut ALLOC: Allocator = Allocator {
	first_free: 0 as *mut _
};

#[repr(transparent)]
struct Header(usize);

impl Header {
	unsafe fn used(self: *const Self) -> bool {
		(*self).0 & (1 << 31) != 0
	}

	unsafe fn set_used(self: *mut Self, used: bool) {
		let val = (*self).0 & !(1<< 31);
		(*self).0 = val | if used { 1 << 31 } else { 0 };
	}

	unsafe fn len(self: *const Self) -> usize {
		(*self).0 & !(1 << 31)
	}

	unsafe fn set_len(self: *mut Self, len: usize) {
		(*self).0 = len | if self.used() { 1 << 31 } else { 0 };
	}

	unsafe fn next_mut(self: *mut Self) -> Option<*mut Self> {
		if self.len() > 0 {
			Some(self.offset(self.len() as isize + 1))
		} else {
			None
		}
	}

	unsafe fn next(self: *const Self) -> Option<*const Self> {
		if self.len() > 0 {
			Some(self.offset(self.len() as isize + 1))
		} else {
			None
		}
	}

	unsafe fn layout_fits(self: *const Self, size: usize, align: usize) -> bool {
		let align_mask = align - 1;
		let aligned = (self as usize & !align_mask) + align;
		let end = aligned + size;
		end <= self.next().unwrap_or(0 as *const _) as usize
	}

	unsafe fn cut(self: *mut Self, size: usize) {
		let next_len = self.len() - size - 1;
		if next_len > 0 {
			let next = self.offset(size as isize + 1);
			next.set_len(next_len);
			next.set_used(self.used());
			self.set_len(size);
		}
	}
}

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
	panic!("ALLOC");
}
