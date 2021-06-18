use core::{
    self,
    ptr::addr_of_mut,
    u8,
};

#[derive(Clone, Copy)]
pub enum PortName {
    C,
}

#[repr(C, packed)]
pub struct Port {
    // Array to store the 32 Pin Control Registers
    pcr: [u32; 32],
    gpclr: u32,
    gpchr: u32,
    isfr: u32,
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        // the matchcase returns the address only when portname is C
        &mut *match name {
            PortName::C => 0x4004B000 as *mut Port,
        }
        // Complete the function below. Similar to watchdog. But use
        // a matchcase since we should only return when portname is C.
        // See the address in section 11.1.4.
    }

    pub unsafe fn set_pin_mode(&mut self, p: usize, mode: u32) {
        // Given the pin mode as a 32 bit value set the register bytes
        // to the same value for the corresponding pin. See the MUX(10-8)
        // bits in section 11.14.1. We need to set only those bits.
        // Again think of appropriate operations using AND,OR,XOR etc..
        // There are only 8 possible pin models so mode = 0 to 7. Reject if different.
        if mode > u8::MAX as u32 {
            return;
        }

        // Get the value of pcr
        let pcr = core::ptr::addr_of_mut!(self.pcr[p]);
        let mut pcr_val = core::ptr::read_volatile(pcr);

        // Clear bits 8-10
        pcr_val &= !(0x7 << 8);
        // Set bits 8-10 to value specified in mode
        pcr_val |= mode << 8;

        // Write pcr_val to register
        core::ptr::write_volatile(pcr, pcr_val);
    }
}

pub struct Pin {
    port: *mut Port,
    pin: usize,
}

impl Port {
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        Pin { port: self, pin: p } // Complete and return a pin struct
    }
}

#[repr(C, packed)]
struct GpioBitBand {
    pdor: [u32; 32],
    psor: [u32; 32],
    pcor: [u32; 32],
    ptor: [u32; 32],
    pdir: [u32; 32],
    pddr: [u32; 32], // Complete using section 49.2
}

pub struct Gpio {
    gpio: *mut GpioBitBand,
    pin: usize,
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            // Return PortName::C if the address matches the starting
            // address of port C as specified in section 11.1.4. Reject
            // if address is wrong and return error.
            0x4004B000 => PortName::C,
            _ => unreachable!(),
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            // Set pin mode to 1 to enable gpio mode (section 11.14.1 MUX bits).
            // Consume the pin into a gpio struct i.e. instantitate a gpio
            // struct using the new function below.
            let port = &mut *self.port;
            port.set_pin_mode(self.pin, 1);
            Gpio::new(port.name(), self.pin)
        }
    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {
        let gpio = match port {
            PortName::C => 0x43FE1000 as *mut GpioBitBand,
        };

        // Initialize and return a gpio struct.
        Gpio { gpio, pin }
    }

    // Sets GPIO to output
    pub fn output(&mut self) {
        unsafe {
            // The PDDR configures the individual port pins for input or output.
            // WRITE THE  XX register of GPIO to 1 to enable this pin as output type.
            // See section 49.2 of the teensy manual to find out what is XX.
            // Get the address of PDDR
            let pddr = addr_of_mut!((*self.gpio).pddr[self.pin]);

            // Write
            core::ptr::write_volatile(pddr, 1);
        }
    }

    // Sets GPIO to high
    pub fn high(&mut self) {
        unsafe {
            //PSOR configures whether to set the fields of the PDOR.
            // WRITE THE  XX register of GPIO to 1 to set this pin as high.
            // See section 49.2 of the teensy manual to find out what is XX.
            // Please not that it is not PDOR, since PDOR is never directly written.
            // Get the address of PSOR
            let psor = addr_of_mut!((*self.gpio).psor[self.pin]);

            // Write
            core::ptr::write_volatile(psor, 1);
        }
    }

    // Sets GPIO to low
    pub fn low(&mut self) {
        unsafe {
            // Get address of register
            let psor_mut = addr_of_mut!((*self.gpio).psor[self.pin]);

            // Write 0x0 byte to set the pin to low
            core::ptr::write_volatile(psor_mut, 0x0);
        }
    }
}
