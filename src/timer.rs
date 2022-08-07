use core::future::Future;

pub trait Timer {
	type DelayFuture: Future<Output = ()>;

	fn start(&mut self);
	fn stop(&mut self);
	fn prescaller(&mut self, psx: u32);
	fn delay(&mut self, ticks: u32) -> Self::DelayFuture;
	fn one_pulse(&mut self, opm: bool);
	fn irq(&mut self, irq: bool);
	fn trigger(&mut self);
}

#[cfg(feature = "f4")]
mod f4 {
	pub mod basic;
}

#[cfg(feature = "f4")]
pub use f4::*;
