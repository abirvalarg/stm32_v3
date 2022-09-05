use crate::sync::block_irq;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
	CR: usize,
	PLLCFGR: usize,
	CFGR: usize,
	CIR: usize,
	AHB1RSTR: usize,
	AHB2RSTR: usize,
	AHB3RSTR: usize,
	_res0: usize,
	APB1RSTR: usize,
	APB2RSTR: usize,
	_res1: usize,
	_res2: usize,
	AHB1ENR: usize,
	AHB2ENR: usize,
	AHB3ENR: usize,
	_res3: usize,
	APB1ENR: usize,
	APB2ENR: usize,
	_res4: usize,
	_res5: usize
}

pub struct Rcc(*mut Reg);

impl Rcc {
	pub fn switch_ahb1(&self, module: Ahb1Module, state: bool) {
		block_irq(|| unsafe {
			let module = module as usize;
			let reg = &mut (*self.0).AHB1ENR as *mut usize;
			let val = reg.read_volatile() & !(1 << module);
			reg.write_volatile(val | if state { 1 << module } else { 0 });
		})
	}
	
	pub fn switch_apb1(&self, module: Apb1Module, state: bool) {
		block_irq(|| unsafe {
			let module = module as usize;
			let reg = &mut (*self.0).APB1ENR as *mut usize;
			let val = reg.read_volatile() & !(1 << module);
			reg.write_volatile(val | if state { 1 << module } else { 0 });
		})
	}

	pub fn switch(&self, module: Module, state: bool) {
		use Module::*;
		match module {
			Ahb1(module) => self.switch_ahb1(module, state),
			Apb1(module) => self.switch_apb1(module, state)
		}
	}

	pub fn enable_hse(&self, state: bool) {
		block_irq(|| unsafe {
			let reg = &mut (*self.0).CR as *mut usize;
			let val = reg.read_volatile() & !(1 << 16);
			reg.write_volatile(val | if state { 1 << 16 } else { 0 });
			if state {
				while reg.read_volatile() & (1 << 17) == 0 {}
			}
		});
	}

	pub fn enable_pll(&self, state: bool) {
		block_irq(|| unsafe {
			let reg = &mut (*self.0).CR as *mut usize;
			let val = reg.read_volatile() & !(1 << 24);
			reg.write_volatile(val | if state { 1 << 24 } else { 0 });
			if state {
				while reg.read_volatile() & (1 << 25) == 0 {}
			}
		});
	}

	pub fn pll_cfg(&self, q: usize, p: usize, n: usize, m: usize, src: PllSource) {
		block_irq(|| unsafe {
			let reg = &mut (*self.0).PLLCFGR as *mut usize;
			let val = (q & 0b1111) << 24
				| (p & 0b11) << 16
				| (n & 0b111111111) << 6
				| (m & 0b111111)
				| (src as usize) << 22;
			reg.write_volatile(val);
		});
	}

	pub fn set_system_clock_source(&self, src: SystemClockSource) {
		block_irq(|| unsafe {
			let reg = &mut (*self.0).CFGR as *mut usize;
			let val = reg.read_volatile() & !3;
			reg.write_volatile(val | (src as usize));
		});
	}

	#[cfg(feature = "f401")]
	pub fn set_bus_psc(&self, ahb: usize, apb1: usize, apb2: usize) {
		block_irq(|| unsafe {
			let reg = &mut (*self.0).CFGR as *mut usize;
			let val = reg.read_volatile()
				& !0b1111110011110000;
			let val = val
				| (ahb & 0b1111) << 4
				| (apb1 & 0b111) << 10
				| (apb2 & 0b111) << 13;
			reg.write_volatile(val);
		});
	}
}

#[derive(Copy, Clone)]
pub enum Ahb1Module {
	GpioA = 0,
	GpioB = 1,
	GpioC = 2,
	GpioD = 3,
	GpioE = 4,
}

#[derive(Copy, Clone)]
pub enum Apb1Module {
	TIM3 = 1,
	TIM4 = 2
}

#[derive(Copy, Clone)]
pub enum Module {
	Ahb1(Ahb1Module),
	Apb1(Apb1Module)
}

pub enum PllSource {
	HSI = 0,
	HSE = 1
}

pub enum SystemClockSource {
	HSI = 0b00,
	HSE = 0b01,
	PLL = 0b10
}

pub const RCC: Rcc = Rcc(0x4002_3800 as *mut _);
