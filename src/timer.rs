pub trait Timer {
	fn start(&mut self);
	fn stop(&mut self);
	fn prescaller(&mut self, psc: u32);
	fn reload(&mut self, arr: u32);
	fn one_pulse(&mut self, opm: bool);
	fn irq(&mut self, irq: bool);
	fn trigger(&mut self);
	fn on_reload<F: FnMut() + Send + 'static>(&mut self, func: F);
	fn update(&mut self);
}

#[cfg(feature = "f4")]
mod f4;

#[cfg(feature = "f4")]
pub use f4::*;
