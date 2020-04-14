use core::time::Duration;

use volatile::prelude::*;
use volatile::Volatile;

const INT_BASE: usize = 0x40000000;

/// Core interrupt sources (QA7: 4.10)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LocalInterrupt {
    // Lab 5 1.C
    // FIXME: please fill in the definition
}

impl LocalInterrupt {
    pub const MAX: usize = 12;

    pub fn iter() -> impl Iterator<Item = LocalInterrupt> {
        (0..LocalInterrupt::MAX).map(|n| LocalInterrupt::from(n))
    }
}

impl From<usize> for LocalInterrupt {
    fn from(irq: usize) -> LocalInterrupt {
        // Lab 5 1.C
        unimplemented!("LocalInterrupt")
    }
}

/// BCM2837 Local Peripheral Registers (QA7: Chapter 4)
#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    // Lab 5 1.C
    // FIXME: please fill in the definition
}

pub struct LocalController {
    core: usize,
    registers: &'static mut Registers,
}

impl LocalController {
    /// Returns a new handle to the interrupt controller.
    pub fn new(core: usize) -> LocalController {
        LocalController {
            core: core,
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    pub fn enable_local_timer(&mut self) {
        // Lab 5 1.C
        unimplemented!("LocalInterrupt")
    }

    pub fn is_pending(&self, int: LocalInterrupt) -> bool {
        // Lab 5 1.C
        unimplemented!("LocalInterrupt")
    }

    pub fn tick_in(&mut self, t: Duration) {
        // Lab 5 1.C
        // See timer: 3.1 to 3.3
        unimplemented!("LocalInterrupt")
    }
}

pub fn local_tick_in(core: usize, t: Duration) {
    LocalController::new(core).tick_in(t);
}
