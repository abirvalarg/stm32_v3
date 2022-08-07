pub fn block_irq<F: FnOnce() -> R, R>(func: F) -> R {
	unsafe {
		let primask: usize;
		core::arch::asm!("mrs {}, PRIMASK", out(reg) primask);
		core::arch::asm!("cpsid i");
		let res = func();
		if primask == 0 {
			core::arch::asm!("cpsie i");
		}
		res
	}
}
