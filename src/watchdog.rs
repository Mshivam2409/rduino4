use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Watchdog {
    stctrlh: u16,
    stctrll: u16,
    tovalh: u16,
    tovall: u16,
    winh: u16,
    winl: u16,
    refresh: u16,
    unlock: u16,
    tmrouth: u16,
    tmroutl: u16,
    rstcnt: u16,
    presc: u16,
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        // You can see the starting address in section 23.7 of the manual i.e. 4005_2000.
        &mut *(0x40052000 as *mut Watchdog)
    }

    pub fn disable(&mut self) {
        unsafe {
            // access volatile object  and unlocking the watchdog for modification
            core::ptr::write_volatile(&mut self.unlock, 0xC520);
            core::ptr::write_volatile(&mut self.unlock, 0xD928);
            // 2 cycle delay for every unlock
            __nop();
            __nop();
            // reading into the Wathdog Status and Control Register High
            let mut ctrl = core::ptr::read_volatile(&self.stctrlh);
            // changing the 0th bit
            ctrl &= !(0x00000001);
            // writing into the register
            core::ptr::write_volatile(&mut self.stctrlh, ctrl);

        }
    }
}
