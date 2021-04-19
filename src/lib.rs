use stm32f4::stm32f401::Interrupt;
use cortex_m::interrupt::Nr;

#[no_mangle]
pub extern fn rtic_scope_func() -> u8 {
    Interrupt::EXTI0.nr()
}
