#![feature(asm)]
#![feature(global_asm)]
#![cfg_attr(not(test), no_std)]

#[macro_use]
pub mod macros;

pub mod asm;
pub mod regs;
pub mod sp;
pub mod vmsa;

pub use asm::*;
pub use regs::*;
pub use sp::SP;
pub use vmsa::*;

/// Returns the current exception level.
/// This is a privileged operation and will abort in EL0.
#[inline(always)]
pub fn current_el() -> u8 {
    ((unsafe { CurrentEL.get() } & 0b1100) >> 2) as u8
}

/// Returns the SPSel value.
#[inline(always)]
pub fn sp_sel() -> u8 {
    unsafe { SPSel.get_value(SPSel::SP) as u8 }
}

/// Returns the core currently executing.
/// This is a privileged operation and will abort in EL0.
pub fn affinity() -> usize {
    unsafe { MPIDR_EL1.get_value(MPIDR_EL1::Aff0) as usize }
}
