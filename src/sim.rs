use core;

#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}

#[repr(C, packed)]
pub struct Sim {
    // Complete code here
    // See section 12.2 of the teensy manual for the register sizes and memory locations and do similar to the watchdog struct.
    // Note that there are some empty bits between some registers and they are not continous, how do we resolve that ? Padding, eh ?
    sopt1: u32,
    sopt1_cfg: u32,
    _pad0: [u32; 1023],
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
        // Complete code here (similar to watchdog), see memory location from section 12.2
        &mut *(0x40047000 as *mut Sim)
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    // Use the teensy manual to find out which register controls Port C. Then implement this function to enable port C. Scroll through section 12.2 to find which bit of which register needs to be changed to enable clock gate to Port C. Note that all other bits of that register must remain unchanged.
                    let mut scgc = core::ptr::read_volatile(&self.scgc5);
                    scgc |= 0x00000800;
                    core::ptr::write_volatile(&mut self.scgc5, scgc);
                }
            }
        }
    }
}
