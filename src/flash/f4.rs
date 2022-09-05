use crate::sync::block_irq;

#[allow(non_snake_case)]
#[repr(C)]
struct Reg {
	ACR: usize,
	KEYR: usize,
	OPTKEYR: usize,
	SR: usize,
	CR: usize,
	OPTCR: usize,
}

pub struct Flash(*mut Reg);

impl Flash {
	pub fn set_latency(&self, latency: usize) {
		block_irq(|| unsafe {
			let reg = &mut (*self.0).ACR as *mut usize;
			let val = reg.read_volatile() & !0xf;
			reg.write_volatile(val | (latency & 0xf));
		});
	}
}

pub const FLASH: Flash = Flash(0x4002_3c00 as *mut _);
