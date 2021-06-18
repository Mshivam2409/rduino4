#![feature(stdsimd)]
#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(unknown_lints)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::empty_loop)]

mod port;
mod sim;
mod watchdog;

use port::Port;
use sim::Sim;
use watchdog::Watchdog;

pub extern "C" fn main() {
    // Disable watchdog timer
    //let(dogw,sim,pin)= unsafe { (Watchdog::new(),Sim::new(),) }.disable();
    let dogw = unsafe { Watchdog::new() };
    let sim = unsafe { Sim::new() };
    let pin = unsafe { Port::new(port::PortName::C).pin(5) };

    dogw.disable();
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
