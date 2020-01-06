use core::mem::zeroed;
use core::ptr::write_volatile;

mod panic;

use crate::kmain;

global_asm!(include_str!("init/init.s"));

unsafe fn zeros_bss() {
    extern "C" {
        static mut __bss_beg: u64;
        static mut __bss_end: u64;
    }

    let mut iter: *mut u64 = &mut __bss_beg;
    let end: *mut u64 = &mut __bss_end;

    while iter < end {
        write_volatile(iter, zeroed());
        iter = iter.add(1);
    }
}

#[no_mangle]
unsafe fn kinit() -> ! {
    zeros_bss();
    kmain();
}