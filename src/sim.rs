use core;

#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}

#[repr(C, packed)]
pub struct Sim { // to understand read section 12.2 of teensy manual 
    sopt1: u32,// registers for sim 
    sopt1_cfg: u32,
    _pad0: [u32; 1023],// we had to introduce padding as the registers are not contigous in memory use _padi to introduce padding to skip memory blocks in between the registers for Sim
    sopt2: u32,
    _pad1: u32,
    sopt4: u32,
    sopt5: u32,
    _pad2: u32,
    sopt7: u32,
    _pad3: [u32; 2],
    sdid: u32,
    _pad4: [u32; 3],
    scgc4: u32,
    scgc5: u32,
    scgc6: u32,
    scgc7: u32,
    clkdiv1: u32,
    clkviv2: u32,
    fcfg1: u32,
    fcfg2: u32,
    uidh: u32,
    uidmh: u32,
    uidml: u32,
    uidl: u32,
}

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        // first register address of the register in Sim to start giving address in the Sim struct
        &mut *(0x40047000 as *mut Sim)
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    let mut scgc = core::ptr::read_volatile(&self.scgc5);//Read value at the "System Clock Gating Control Register 5" to make changes to enable Watch clock for Port C see 12.2 from manual
                    scgc |= 0x00000800;//make value at the 12th bit to 1 to enable the watch clock for Port C at scgc_5.... to check read 12.2 teensy manual 
                    core::ptr::write_volatile(&mut self.scgc5, scgc);//write value at "system Clock Gating Control Register 5" to finally enable the watch clock
                }
            }
        }
    }
}
