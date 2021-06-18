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
    // Create watchdog
    let dogw = unsafe { Watchdog::new() };
    // Create sim
    let sim = unsafe { Sim::new() };
    // Grab pin 5 from port
    let pin = unsafe { Port::new(port::PortName::C).pin(5) };

    // Disable watchdog timer
    dogw.disable();
    // Enable clock gating for port C
    sim.enable_clock(sim::Clock::PortC);
    // Make the pin 5 GPIO
    let mut gpio = pin.make_gpio();

    // Set GPIO as output
    gpio.output();

    // Blink the LED periodically
    loop {
        // Turn on the LED
        gpio.high();

        // Turn off the LED
        gpio.low();
    }
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
