use alloc::boxed::Box;

#[repr(C)]
#[allow(non_snake_case)]
struct Reg {
    CR1: usize,
    CR2: usize,
    SMCR: usize,
    DIER: usize,
    SR: usize,
    EGR: usize,
    CCMR1: usize,
    CCMR2: usize,
    CCER: usize,
    CNT: usize,
    PSC: usize,
    ARR: usize,
    _reserved0: usize,
    CCR1: usize,
    CCR2: usize,
    CCR3: usize,
    CCR4: usize,
    _reserved1: usize,
    DCR: usize,
    DMAR: usize,
}

pub struct Gp34 {
    hw: *mut Reg,
    on_reload: Option<Box<dyn FnMut()>>
}

impl Gp34 {
    pub const fn new(addr: usize) -> Self {
        Gp34 {
            hw: addr as *mut Reg,
            on_reload: None
        }
    }
}

impl crate::timer::Timer for Gp34 {
    fn start(&mut self) {
        unsafe {
            let reg = &mut (*self.hw).CR1 as *mut usize;
            let val = reg.read_volatile();
            reg.write_volatile(val | 1);
        }
    }

    fn stop(&mut self) {
        unsafe {
            let reg = &mut (*self.hw).CR1 as *mut usize;
            let val = reg.read_volatile();
            reg.write_volatile(val & !1);
        }
    }

    fn prescaller(&mut self, psc: u32) {
        unsafe {
            let reg = &mut (*self.hw).PSC as *mut usize;
            reg.write_volatile(psc as usize);
        }
    }

    fn reload(&mut self, arr: u32) {
        unsafe {
            let reg = &mut (*self.hw).ARR as *mut usize;
            reg.write_volatile(arr as usize);
        }
    }

    fn one_pulse(&mut self, opm: bool) {
        unsafe {
            let reg = &mut (*self.hw).CR1 as *mut usize;
            let val = reg.read_volatile() & !(1 << 3);
            reg.write_volatile(val | if opm { 1 << 3 } else { 0 });
        }
    }

    fn irq(&mut self, irq: bool) {
        unsafe {
            let reg = &mut (*self.hw).DIER as *mut usize;
            let val = reg.read_volatile() & !1;
            reg.write_volatile(val | if irq { 1 } else { 0 });
        }
    }

    fn trigger(&mut self) {
        unsafe {
            let reg = &mut (*self.hw).EGR as *mut usize;
            reg.write_volatile(1);
            let sr = &mut (*self.hw).SR as *mut usize;
            while sr.read_volatile() & 1 == 0 {}
            let val = sr.read_volatile() & !1;
            sr.write_volatile(val);
        }
    }

    fn on_reload<F: FnMut() + Send + 'static>(&mut self, func: F) {
        self.on_reload = Some(Box::new(func));
    }

    fn update(&mut self) {
        unsafe {
            let sr = &mut (*self.hw).SR as *mut usize;
            let val = sr.read_volatile();
            if val & 1 != 0 {
                if let Some(func) = &mut self.on_reload {
                    func();
                }

                sr.write_volatile(val & !1);
            }
        }
    }
}
