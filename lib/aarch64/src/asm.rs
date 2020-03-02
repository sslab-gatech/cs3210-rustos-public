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
pub fn eret() {
    unsafe { asm!("eret" :::: "volatile") };
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
pub unsafe fn sti() {
    asm!("msr DAIFClr, 0b0010"
         :
         :
         :
         : "volatile");
}

/// Disable (mask) interrupt
#[inline(always)]
pub unsafe fn cli() {
    asm!("msr DAIFSet, 0b0010"
         :
         :
         :
         : "volatile");
}

/// Break with an immeidate
#[macro_export]
macro_rules! brk {
    ($num:tt) => {
        unsafe { asm!(concat!("brk ", stringify!($num)) :::: "volatile"); }
    }
}

/// Supervisor call with an immediate
#[macro_export]
macro_rules! svc {
    ($num:tt) => {
        unsafe { asm!(concat!("svc ", stringify!($num)) :::: "volatile"); }
    }
}
