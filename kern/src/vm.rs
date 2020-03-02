use crate::console::kprintln;
use crate::mutex::Mutex;

use aarch64::*;

mod address;
mod pagetable;

pub use self::address::{PhysicalAddr, VirtualAddr};
pub use self::pagetable::*;
use crate::param::{KERNEL_MASK_BITS, USER_MASK_BITS};

/// Thread-safe (locking) wrapper around a kernel page table.
pub struct VMManager(Mutex<Option<KernPageTable>>);

impl VMManager {
    /// Returns an uninitialized `VMManager`.
    ///
    /// The virtual memory manager must be initialized by calling `initialize()` and `setup()`
    /// before the first memory allocation. Failure to do will result in panics.
    pub const fn uninitialized() -> Self {
        VMManager(Mutex::new(None))
    }

    /// Initializes the virtual memory manager.
    /// The caller should assure that the method is invoked only once during the kernel
    /// initialization.
    pub fn initialize(&self) {
        unimplemented!();
    }

    /// Set up the virtual memory manager.
    /// The caller should assure that `initialize()` has been called before calling this function.
    /// Sets proper configuration bits to MAIR_EL1, TCR_EL1, TTBR0_EL1, and TTBR1_EL1 registers.
    ///
    /// # Panics
    ///
    /// Panics if the current system does not support 64KB memory translation granule size.
    pub fn setup(&self) {
        let kern_page_table = self.0.lock();
        let baddr = kern_page_table.as_ref().unwrap().get_baddr().as_u64();

        unsafe {
            assert!(ID_AA64MMFR0_EL1.get_value(ID_AA64MMFR0_EL1::TGran64) == 0);

            let ips = ID_AA64MMFR0_EL1.get_value(ID_AA64MMFR0_EL1::PARange);

            // (ref. D7.2.70: Memory Attribute Indirection Register)
            MAIR_EL1.set(
                (0xFF <<  0) |// AttrIdx=0: normal, IWBWA, OWBWA, NTR
                (0x04 <<  8) |// AttrIdx=1: device, nGnRE (must be OSH too)
                (0x44 << 16), // AttrIdx=2: non cacheable
            );
            // (ref. D7.2.91: Translation Control Register)
            TCR_EL1.set(
                (0b00 << 37) |// TBI=0, no tagging
                (ips  << 32) |// IPS
                (0b11 << 30) |// TG1=64k
                (0b11 << 28) |// SH1=3 inner
                (0b01 << 26) |// ORGN1=1 write back
                (0b01 << 24) |// IRGN1=1 write back
                (0b0  << 23) |// EPD1 enables higher half
                ((USER_MASK_BITS as u64) << 16) | // T1SZ=34 (1GB)
                (0b01 << 14) |// TG0=64k
                (0b11 << 12) |// SH0=3 inner
                (0b01 << 10) |// ORGN0=1 write back
                (0b01 <<  8) |// IRGN0=1 write back
                (0b0  <<  7) |// EPD0 enables lower half
                ((KERNEL_MASK_BITS as u64) << 0), // T0SZ=32 (4GB)
            );
            isb();

            TTBR0_EL1.set(baddr);
            TTBR1_EL1.set(baddr);

            asm!("dsb ish");
            isb();

            SCTLR_EL1.set(SCTLR_EL1.get() | SCTLR_EL1::I | SCTLR_EL1::C | SCTLR_EL1::M);
            asm!("dsb sy");
            isb();
        }
    }

    /// Returns the base address of the kernel page table as `PhysicalAddr`.
    pub fn get_baddr(&self) -> PhysicalAddr {
        unimplemented!();
    }
}
