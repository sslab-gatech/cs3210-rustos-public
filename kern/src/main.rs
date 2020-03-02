#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(ptr_internals)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;
pub mod param;
pub mod process;
pub mod traps;
pub mod vm;

use console::kprintln;

use allocator::Allocator;
use fs::FileSystem;
use process::GlobalScheduler;
use traps::irq::Irq;
use vm::VMManager;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();
pub static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();
pub static VMM: VMManager = VMManager::uninitialized();
pub static IRQ: Irq = Irq::uninitialized();

fn kmain() -> ! {
    unsafe {
        ALLOCATOR.initialize();
        FILESYSTEM.initialize();
    }

    kprintln!("Welcome to cs3210!");
    shell::shell("> ");
}
