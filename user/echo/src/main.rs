#![feature(asm)]
#![feature(never_type)]
#![no_std]
#![no_main]

mod cr0;

use core::time::Duration;

use kernel_api::syscall::*;
use kernel_api::{print, println, OsResult};

fn main() {
    let result = main_inner();
    if let Err(error) = result {
        println!("Terminating with error: {:?}", error);
    }
}

fn main_inner() -> OsResult<!> {
    // Lab 5 3
    unimplemented!("main_inner()")
}
