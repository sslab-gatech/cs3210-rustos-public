use crate::common::IO_BASE;

use volatile::prelude::*;
use volatile::{ReadVolatile, Volatile};

const INT_BASE: usize = IO_BASE + 0xB000 + 0x200;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Interrupt {
    Timer1 = 1,
    Timer3 = 3,
    Usb = 9,
    Gpio0 = 49,
    Gpio1 = 50,
    Gpio2 = 51,
    Gpio3 = 52,
    Uart = 57,
}

impl Interrupt {
    pub const MAX: usize = 8;

    pub fn iter() -> impl Iterator<Item = Interrupt> {
        use Interrupt::*;
        [Timer1, Timer3, Usb, Gpio0, Gpio1, Gpio2, Gpio3, Uart]
            .iter()
            .map(|int| *int)
    }
}

impl From<usize> for Interrupt {
    fn from(irq: usize) -> Interrupt {
        use Interrupt::*;
        match irq {
            1 => Timer1,
            3 => Timer3,
            9 => Usb,
            49 => Gpio0,
            50 => Gpio1,
            51 => Gpio2,
            52 => Gpio3,
            57 => Uart,
            _ => panic!("Unknown irq: {}", irq),
        }
    }
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    // FIXME: Fill me in.
}

/// An interrupt controller. Used to enable and disable interrupts as well as to
/// check if an interrupt is pending.
pub struct Controller {
    registers: &'static mut Registers,
}

impl Controller {
    /// Returns a new handle to the interrupt controller.
    pub fn new() -> Controller {
        Controller {
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    /// Enables the interrupt `int`.
    pub fn enable(&mut self, int: Interrupt) {
        unimplemented!()
    }

    /// Disables the interrupt `int`.
    pub fn disable(&mut self, int: Interrupt) {
        unimplemented!()
    }

    /// Returns `true` if `int` is pending. Otherwise, returns `false`.
    pub fn is_pending(&self, int: Interrupt) -> bool {
        unimplemented!()
    }

    /// Enables the interrupt as FIQ interrupt
    pub fn enable_fiq(&mut self, int: Interrupt) {
        // Lab 5 2.B
        unimplemented!("enable_fiq")
    }
}
