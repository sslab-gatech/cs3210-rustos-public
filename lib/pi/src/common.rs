/// The address where I/O peripherals are mapped to.
pub const IO_BASE: usize = 0x3F000000;
pub const IO_BASE_END: usize = 0x40000000 + 0x20000000;

/// The base address of the `GPIO` registers
pub const GPIO_BASE: usize = IO_BASE + 0x200000;

/// The number of cores in Rpi3
pub const NCORES: usize = 4;

/// The base of physical addresses that each core is spinning on
pub const SPINNING_BASE: *mut usize = 0xd8 as *mut usize;

/// Generates `pub enums` with no variants for each `ident` passed in.
pub macro states($($name:ident),*) {
    $(
        /// A possible state.
        #[doc(hidden)]
        pub enum $name {  }
    )*
}
