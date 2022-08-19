#![no_std]
#![no_main]
#![feature(arbitrary_self_types)]
#![feature(alloc_error_handler)]

#[cfg(feature = "heap")]
extern crate alloc;

#[cfg(feature = "heap")]
mod allocator;
#[cfg(feature = "async")]
pub mod executor;
pub mod sync;
pub mod mutex;
pub mod timer;
mod irq;

#[cfg(feature = "f40")]
pub mod rcc {
	mod f40;
	pub use f40::*;
}

#[cfg(feature = "f4")]
pub mod gpio {
	mod f4;
	pub use f4::*;
}

#[cfg(feature = "f4")]
pub mod nvic {
	mod f4;
	pub use f4::*;
}

#[cfg(feature = "default_panic")]
#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
	loop {
		unsafe {
			core::arch::asm!("wfi");
		}
	}
}

#[no_mangle]
extern "C" fn __aeabi_unwind_cpp_pr0() -> ! {
	panic!();
}
