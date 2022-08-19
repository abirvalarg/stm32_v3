use crate::timer::Timer;

#[no_mangle]
extern "C" fn TIM3() {
    let mut tim = crate::timer::TIM3.get();
    tim.update();
}
