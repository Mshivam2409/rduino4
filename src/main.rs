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

pub extern "C" fn main() {
    // Complete main function here

    // Watchdog disabled by the program
    let wdog = watchdog::WatchDog::new();
    wdog.disable();

    // Enabling Clock Gating in the program
    let sim = sim::Sim::new();
    clock = sim::Clock::PortC;
    sim.enable_clock(clock);

    // setting up of Pin 5 as a GPIO pin
    p = port::PortName::C;
    let port = port::Port::new(p);
    let pin = port.pin(5);
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
// pub static _FLASHCONFIG:

// FLASH CONFIGURATION is a set of 16 registers of 1 byte = 8 bits each
pub static _FLASHCONFIG : [u8 ; 16] = [
    // All the other bytes except FSEC and FOPT are to be changed.
    // 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	// 0xFF, 0xFF, 0xFF, 0xFF, FSEC, FOPT, 0xFF, 0xFF
    
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF
];

#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    loop {}
}
