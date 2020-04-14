#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        let ms = 10000;
        let _error: u64;
        let _elapsed_ms: u64;

        unsafe {
            asm!("mov x0, $2
                  svc 1
                  mov $0, x0
                  mov $1, x7"
                 : "=r"(_elapsed_ms), "=r"(_error)
                 : "r"(ms)
                 : "x0", "x7"
                 : "volatile");
        }
    }
}
