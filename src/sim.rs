use core;

#[derive(Clone, Copy)]
pub enum Clock {
    PortC,
}

// Represents a 32-bit unsigned int
struct DWORD(u32);

#[repr(C, packed)]
pub struct Sim {
    // Complete code here
    // See section 12.2 of the teensy manual for the register sizes
    // and memory locations and do similar to the watchdog struct.
    // Note that there are some empty bits between some registers and
    // they are not continous, how do we resolve that ? Padding, eh ?
    sopt1: u32,
    sopt1cfg: u32,
    // Add 0x3FF DWORD padding
    _pad0: [DWORD; 0x3FF],
    sopt2: u32,
    // Add 0x001 DWORD padding
    _pad1: [DWORD; 0x001],
    sopt4: u32,
    sopt5: u32,
    // Add 0x001 DWORD padding
    _pad2: [DWORD; 0x001],
    sopt7: u32,
    // Add 0x002 DWORD padding
    _pad3: [DWORD; 0x002],
    sdid: u32,
    scgc1: u32,
    scgc2: u32,
    scgc3: u32,
    scgc4: u32,
    scgc5: u32,
    scgc6: u32,
    scgc7: u32,
    clkdiv1: u32,
    clkdiv2: u32,
    fcfg1: u32,
    fcfg2: u32,
    uidh: u32,
    uidmh: u32,
    uidml: u32,
    uidl: u32,
}

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        // Complete code here (similar to watchdog),
        // see memory location from section 12.2
        &mut *(0x40047000 as *mut Sim)
    }

    pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {
            match clock {
                Clock::PortC => {
                    // Use the teensy manual to find out which register
                    // controls Port C. Then implement this function to
                    // enable port C. Scroll through section 12.2 to find
                    // which bit of which register needs to be changed to
                    // enable clock gate to Port C. Note that all other
                    // bits of that register must remain unchanged.
                    self.scgc5.update(|scgc|) {
                        scgc.set_bit(11,true);
                    });
                }
            }
        }
    }
}
