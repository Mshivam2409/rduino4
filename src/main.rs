#![feature(stdsimd)]
#![no_std]
#![no_main]
#![allow(unknown_lints)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::empty_loop)]

mod port;
mod sim;
mod watchdog;

pub extern "C" fn main() {
    // Complete main function here
    let (wdog, sim, pin) = unsafe {
        (
            watchdog::Watchdog::new(),
            sim::Sim::new(),
            port::Port::new(port::PortName::C).pin(5),
        )
    };

    wdog.disable();
    sim.enable_clock(sim::Clock::PortC);

    let mut gpio = pin.make_gpio();

    gpio.output();
    gpio.high();

    loop {}
}

extern "C" {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern "C" fn(); 2] = [_stack_top, main];

#[link_section = ".flashconfig"]
#[no_mangle]
// Complete the code below
pub static _FLASHCONFIG: [u8; 16] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF,
];
#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    loop {}
}
