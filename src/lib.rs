#![no_std]
#![no_main]
#![feature(arbitrary_self_types)]
#![feature(alloc_error_handler)]

#[cfg(feature = "heap")]
mod allocator;
#[cfg(feature = "async")]
pub mod executor;
pub mod sync;

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

pub mod timer;

#[cfg(feature = "default_panic")]
fn __panic(_: &core::panic::PanicInfo) -> ! {
	loop {
		unsafe {
			core::arch::asm!("wfi");
		}
	}
}
