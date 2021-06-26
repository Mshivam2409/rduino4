use core;

#[derive(Clone,Copy)]
pub enum PortName {
    C
}

#[repr(C,packed)]
pub struct Port { //registers in port c
    pcr : [u32;32], //32 Pin Control Register n
    gpclr : u32, // Global Pin Control Low Register (PORTC_GPCLR)
    gpchr : u32, // Global Pin Control High Register (PORTC_GPCHR) 
    isfr : u32, // Interrupt Status Flag Register(PORTC_ISFR)
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
        &mut *match name {
            PortName::C => 0x4004B000 as *mut Port,// return address only if the port is c
        }
    }
    pub unsafe fn set_pin_mode(&mut self, p: usize, mut mode: u32) {
        let mut pcr= core::ptr::read_volatile(&self.pcr[p]);

        pcr &= 0xFFFFF8FF;// to make the value at 8-10 to be 0 
        mode &= 0x00000007;//to make all other value except 0-2 in mode to zero
        mode <<= 8;//shift mode's value to left by 8
        pcr |= mode;//take or to get the value of mode in pcr  

        core::ptr::write_volatile(&mut self.pcr[p],pcr);
    }
}

pub struct Pin {
    port: *mut Port,
    pin: usize
}

impl Port {
    pub unsafe fn pin(&mut self, p: usize) -> Pin {
        Pin{
            port : self,
            pin : p
        }
    }
}

#[repr(C,packed)]
struct GpioBitBand {
    pdor: u32,// gpio registers for Port C
    psor: u32,
    pcor: u32,
    ptor: u32,
    pdir: u32,
    pddr: u32,
}

pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}

impl Port {
    pub fn name(&self) -> PortName {
        let addr = (self as *const Port) as u32;
        match addr {
            0x4004B000 => PortName::C,//if address matches that of Port C
            _ => unreachable!(),//if address does not match that of Port C
        }
    }
}

impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
            // Consume the pin into a gpio struct i.e. instantitate a gpio struct using the new function below.
            let port: &mut Port=&mut*self.port;//make port to be taken as mutable reference of Port
            port.set_pin_mode(self.pin, 1);// to make a pin into gpio mode 
            Gpio::new(port.name(), self.pin)//making the new Gpio struct for the port given and pin given 
        }
    }
}

impl Gpio {
    pub unsafe fn new(port: PortName, pin: usize) -> Gpio {-++++
        let gpio = match port {
            PortName::C => 0x43FE1000 as *mut GpioBitband
        };

        // Initialize and return a gpio struct.
        Gpio { gpio, pin }
    }

    pub fn output(&mut self) {
        unsafe {
            //  WRITTEN Port Data Direction Register register of GPIO to 1 to enable this pin as output type.
            core::ptr::write_volatile(&mut (*self.gpio).pddr,1);
        }
    }

    pub fn high(&mut self) {
        unsafe {
           //  WRITTEN Port Set Output Register register of GPIO to 1 to set this pin as high.
           core::ptr::write_volatile(&mut (*self.gpio).psor, 1);
        }
    }
}