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

	pub fn switch(&self, module: Module, state: bool) {
		use Module::*;
		match module {
			Ahb1(module) => self.switch_ahb1(module, state)
		}
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
pub enum Module {
	Ahb1(Ahb1Module)
}

pub const RCC: Rcc = Rcc(0x4002_3800 as *mut _);
