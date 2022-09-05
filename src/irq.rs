use crate::timer::Timer;

#[cfg(feature = "f4")]
#[no_mangle]
extern "C" fn TIM3() {
    let mut tim = crate::timer::TIM3.get();
    tim.update();
}

#[cfg(feature = "f4")]
#[no_mangle]
extern "C" fn TIM4() {
    let mut tim = crate::timer::TIM4.get();
    tim.update();
}
