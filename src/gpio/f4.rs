use crate::sync::{block_irq, SyncCell};

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
	used_pins: SyncCell<u16>
}

impl Gpio {
	const fn new(addr: usize) -> Self {
		Gpio {
			hw: addr as *mut Reg,
			used_pins: SyncCell::new(0)
		}
	}

	pub fn pin(&'static self, num: u8) -> Option<Pin> {
		block_irq(move || {
			if num < 16 {
				let mut used_pins = self.used_pins.get();
				let res = if *used_pins & 1 << num == 0 {
					*used_pins |= 1 << num;
					Some(Pin {
						gpio: self,
						num
					})
				} else {
					None
				};
				res
			} else {
				None
			}
		})
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

	pub fn write(&mut self, val: bool) {
		let pos = self.num + if !val { 16 } else { 0 };
		unsafe {
			let reg = &mut (*self.gpio.hw).BSRR as *mut usize;
			reg.write_volatile(1 << pos);
		}
	}
}

impl Drop for Pin {
	fn drop(&mut self) {
		let mut used_pins = self.gpio.used_pins.get();
		*used_pins &= !(1 << self.num);
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

pub static GPIOA: Gpio = Gpio::new(0x4002_0000);
pub static GPIOC: Gpio = Gpio::new(0x4002_0800);
