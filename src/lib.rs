#![no_std]
#![no_main]

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
