use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Watchdog {
    stctrlh: u16,//12 registers for Wacthdog controlling
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
        &mut *(0x40052000 as *mut Watchdog)//starting register of watchdog
    }

    pub fn disable(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.unlock, 0xC520);
            core::ptr::write_volatile(&mut self.unlock, 0xD928);//unlocking the watchdog by writing values for unlock registers can check secction 23.3.1
            __nop();
            __nop();//waiting for watchdog to unlock takes two cycle delay
            let mut ctrl = core::ptr::read_volatile(&self.stctrlh);//take value of stctrlh to disable watchdog see section 23.7.1 
            ctrl &= !(0x00000001);//making 0th bit to 0 of WDOG_STCTRLH to disable Watch dog
            core::ptr::write_volatile(&mut self.stctrlh, ctrl);//write the ctrl value to finally disable watchdog
        }
    }
}
