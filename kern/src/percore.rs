use core::sync::atomic::{AtomicBool, AtomicI64, Ordering};

use crate::param::NCORES;
use crate::traps::irq::LocalIrq;

/// A struct to track per-core data.
#[repr(align(512))]
pub struct PerCore {
    /// Number of locks held by this core
    preemption: AtomicI64,
    /// Is MMU initialized for this core?
    mmu_ready: AtomicBool,
    /// Local IRQ handler registry
    irq: LocalIrq,
}

static PER_CORE_DATA: [PerCore; NCORES] = [
    PerCore {
        preemption: AtomicI64::new(0),
        mmu_ready: AtomicBool::new(false),
        irq: LocalIrq::new(),
    },
    PerCore {
        preemption: AtomicI64::new(0),
        mmu_ready: AtomicBool::new(false),
        irq: LocalIrq::new(),
    },
    PerCore {
        preemption: AtomicI64::new(0),
        mmu_ready: AtomicBool::new(false),
        irq: LocalIrq::new(),
    },
    PerCore {
        preemption: AtomicI64::new(0),
        mmu_ready: AtomicBool::new(false),
        irq: LocalIrq::new(),
    },
];

/// Returns the current preemption counter of this core.
pub fn get_preemptive_counter() -> i64 {
    let cpu = aarch64::affinity();
    PER_CORE_DATA[cpu].preemption.load(Ordering::Relaxed)
}

/// Increases the preemption counter of this core and returns the current core number.
pub fn getcpu() -> usize {
    let cpu = aarch64::affinity();
    PER_CORE_DATA[cpu]
        .preemption
        .fetch_add(1, Ordering::Relaxed);
    cpu
}

/// Decreases the preemption counter of this core. This function asserts that
/// `cpu` parameter matches the current core number.
pub fn putcpu(cpu: usize) {
    assert!(aarch64::affinity() == cpu, "Incorrect putcpu()");
    let cnt = PER_CORE_DATA[cpu]
        .preemption
        .fetch_sub(1, Ordering::Relaxed);
    assert!(cnt > 0, "Preemption count goes to negative!")
}

/// Returns true if MMU is initialized on the current core.
pub fn is_mmu_ready() -> bool {
    let cpu = aarch64::affinity();
    PER_CORE_DATA[cpu].mmu_ready.load(Ordering::Relaxed)
}

/// Sets MMU-ready flag of the current core.
pub unsafe fn set_mmu_ready() {
    let cpu = aarch64::affinity();
    PER_CORE_DATA[cpu].mmu_ready.store(true, Ordering::Relaxed);
}

/// Returns a reference to the local IRQ handler registry of the current core.
pub fn local_irq() -> &'static LocalIrq {
    let cpu = aarch64::affinity();
    &PER_CORE_DATA[cpu].irq
}
