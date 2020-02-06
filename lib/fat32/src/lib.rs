#![feature(decl_macro)]
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(not(feature = "no_std"))]
extern crate core;

#[macro_use]
extern crate alloc;

#[cfg(not(target_endian = "little"))]
compile_error!("only little endian platforms supported");

mod mbr;
#[cfg(test)]
mod tests;
mod util;

pub mod traits;
pub mod vfat;

pub use crate::mbr::*;
