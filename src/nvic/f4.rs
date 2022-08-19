use core::ptr::write_volatile as volatile_write;

#[cold]
pub fn enable_irq(pos: Irq) {
    let pos = pos as usize;
    let block = pos / 32;
    let pos = pos % 32;
    unsafe {
        volatile_write((0xe000_e100 + 4 * block) as *mut usize, 1 << pos);
    }
}


#[non_exhaustive]
#[allow(non_camel_case_types)]
pub enum Irq {
    EXTI0 = 6,
    EXTI1 = 7,
    EXTI2 = 8,
    EXTI3 = 9,
    EXTI4 = 10,
    EXTI9_5 = 23,
    TIM3 = 29,
    TIM4 = 30,
    USART1 = 37,
    USART3 = 39,
    EXTI15_10 = 40,
    TIM6_DAC = 54,
    TIM7 = 55
}
