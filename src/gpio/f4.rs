use core::{sync::atomic::{AtomicU16, Ordering}};

use crate::{rcc::{Module, Ahb1Module}, sync::block_irq};

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
	MODER: usize,
	OTYPER: usize,
	OSPEEDR: usize,
	PUPDR: usize,
	IDR: usize,
	ODR: usize,
	BSRR: usize,
	LCKR: usize,
	AFRL: usize,
	AFRH: usize
}

pub struct Gpio {
	hw: *mut Reg,
	module: Module,
	used_pins: AtomicU16
}

impl Gpio {
	const fn new(addr: usize, module: Module) -> Self {
		Gpio {
			hw: addr as *mut Reg,
			module,
			used_pins: AtomicU16::new(0)
		}
	}

	pub fn switch(&self, state: bool) {
		crate::rcc::RCC.switch(self.module, state);
	}

	pub fn pin(&'static self, num: u8) -> Option<Pin> {
		if num < 16 {
			let used_pins = self.used_pins.load(Ordering::Acquire);
			let res = if used_pins & 1 << num == 0 {
				Some(Pin {
					gpio: self,
					num
				})
			} else {
				None
			};
			self.used_pins.store(used_pins | 1 << num, Ordering::Release);
			res
		} else {
			None
		}
	}
}

unsafe impl Sync for Gpio {}

pub struct Pin {
	gpio: &'static Gpio,
	num: u8
}

impl Pin {
	pub fn mode(&mut self, mode: Mode) {
		block_irq(|| unsafe {
			let hw = self.gpio.hw;
			let moder = &mut (*hw).MODER as *mut usize;
			let moder_val = moder.read_volatile() & !(0b11 << (self.num * 2));
			let mode_num: usize = mode.into();
			moder.write_volatile(moder_val | (mode_num << (self.num * 2)));
			match mode {
				Mode::Input(pull) => self.pull(pull),
				Mode::Alternative(pull, af) => {
					self.pull(pull);
					self.alternative(af);
				}
				_ => ()
			}
		});
	}

	pub fn pull(&mut self, mode: PullMode) {
		block_irq(|| unsafe {
			let hw = self.gpio.hw;
			let reg = &mut (*hw).PUPDR as *mut usize;
			let val = reg.read_volatile() & !(0b11 << self.num * 2);
			reg.write_volatile(val | (mode as usize) << self.num * 2);
		});
	}

	pub fn alternative(&mut self, af: u8) {
		block_irq(|| unsafe {
			let af = af as usize & 0xf;
			let high = self.num >= 8;
			let hw = self.gpio.hw;
			let reg = if high { &mut (*hw).AFRH } else { &mut (*hw).AFRL } as *mut usize;
			let pos = self.num % 8 * 4;
			let val = reg.read_volatile() & !(0xf << pos);
			reg.write_volatile(val | af << pos);
		});
	}
}

#[derive(Copy, Clone)]
pub enum Mode {
	Input(PullMode),
	Output,
	Alternative(PullMode, u8),
	Analog
}

impl Into<usize> for Mode {
	fn into(self) -> usize {
		use Mode::*;
		match self {
			Input(..) => 0,
			Output => 1,
			Alternative(..) => 2,
			Analog => 3
		}
	}
}

#[derive(Copy, Clone)]
pub enum PullMode {
	NoPull = 0,
	PullUp = 1,
	PullDown = 2
}

pub static GPIOA: Gpio = Gpio::new(0x4002_0000, Module::Ahb1(Ahb1Module::GpioA));
