/// Wait for event not to burn CPU.
#[inline(always)]
pub fn wfe() {
    unsafe { asm!("wfe" :::: "volatile") };
}

/// Wait for interrupt not to burn CPU.
#[inline(always)]
pub fn wfi() {
    unsafe { asm!("wfi" :::: "volatile") };
}

/// A NOOP that won't be optimized out.
#[inline(always)]
pub fn nop() {
    unsafe { asm!("nop" :::: "volatile") };
}

/// Transition to a lower level
#[inline(always)]
pub unsafe fn eret() {
    asm!("eret" :::: "volatile");
}

/// Instruction Synchronization Barrier
#[inline(always)]
pub fn isb() {
    unsafe { asm!("isb" :::: "volatile") };
}

/// Set Event
#[inline(always)]
pub fn sev() {
    unsafe { asm!("sev" ::::"volatile") };
}

/// Enable (unmask) interrupts
#[inline(always)]
pub fn enable_irq_interrupt() {
    unsafe {
        asm!("msr DAIFClr, 0b0010"
         :
         :
         :
         : "volatile");
    }
}

/// Disable (mask) interrupt
#[inline(always)]
pub fn disable_irq_interrupt() {
    unsafe {
        asm!("msr DAIFSet, 0b0010"
         :
         :
         :
         : "volatile");
    }
}

/// Enable (unmask) FIQ
#[inline(always)]
pub fn enable_fiq_interrupt() {
    unsafe {
        asm!("msr DAIFClr, 0b0001"
         :
         :
         :
         : "volatile");
    }
}

/// Disable (mask) FIQ
#[inline(always)]
pub fn disable_fiq_interrupt() {
    unsafe {
        asm!("msr DAIFSet, 0b0001"
         :
         :
         :
         : "volatile");
    }
}

pub fn get_interrupt_mask() -> u64 {
    unsafe {
        let mut mask: u64;
        asm!("mrs $0, DAIF"
         : "=r"(mask)
         :
         :
         : "volatile");
        mask
    }
}

pub fn set_interrupt_mask(mask: u64) {
    unsafe {
        asm!("msr DAIF, $0"
         :
         : "r"(mask)
         :
         : "volatile");
    }
}
