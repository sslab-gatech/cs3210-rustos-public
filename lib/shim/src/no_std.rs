pub use core_io as io;

#[cfg(feature = "alloc")]
pub mod ffi;
#[cfg(feature = "alloc")]
pub mod path;
