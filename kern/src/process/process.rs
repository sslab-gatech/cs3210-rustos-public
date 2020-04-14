use alloc::boxed::Box;
use alloc::vec::Vec;
use shim::io;
use shim::path::Path;

use aarch64;
use smoltcp::socket::SocketHandle;

use crate::param::*;
use crate::process::{Stack, State};
use crate::traps::TrapFrame;
use crate::vm::*;
use kernel_api::{OsError, OsResult};

/// Type alias for the type of a process ID.
pub type Id = u64;

/// A structure that represents the complete state of a process.
#[derive(Debug)]
pub struct Process {
    /// The saved trap frame of a process.
    pub context: Box<TrapFrame>,
    /// The memory allocation used for the process's stack.
    pub stack: Stack,
    /// The page table describing the Virtual Memory of the process
    // pub vmap: Box<UserPageTable>,
    /// The scheduling state of the process.
    pub state: State,
    // Lab 5 2.C
    /// Socket handles held by the current process
    // pub sockets: Vec<SocketHandle>,
}

impl Process {
    /// Creates a new process with a zeroed `TrapFrame` (the default), a zeroed
    /// stack of the default size, and a state of `Ready`.
    ///
    /// If enough memory could not be allocated to start the process, returns
    /// `None`. Otherwise returns `Some` of the new `Process`.
    pub fn new() -> OsResult<Process> {
        unimplemented!("Process::new()")
    }

    /// Loads a program stored in the given path by calling `do_load()` method.
    /// Sets trapframe `context` corresponding to its page table.
    /// `sp` - the address of stack top
    /// `elr` - the address of image base.
    /// `ttbr0` - the base address of kernel page table
    /// `ttbr1` - the base address of user page table
    /// `spsr` - `F`, `A`, `D` bit should be set.
    ///
    /// Returns Os Error if do_load fails.
    pub fn load<P: AsRef<Path>>(pn: P) -> OsResult<Process> {
        use crate::VMM;

        let mut p = Process::do_load(pn)?;

        // FIXME: Set trapframe for the process.

        Ok(p)
    }

    /// Creates a process and open a file with given path.
    /// Allocates one page for stack with read/write permission, and N pages with read/write/execute
    /// permission to load file's contents.
    fn do_load<P: AsRef<Path>>(pn: P) -> OsResult<Process> {
        unimplemented!();
    }

    /// Returns the highest `VirtualAddr` that is supported by this system.
    pub fn get_max_va() -> VirtualAddr {
        unimplemented!();
    }

    /// Returns the `VirtualAddr` represents the base address of the user
    /// memory space.
    pub fn get_image_base() -> VirtualAddr {
        unimplemented!();
    }

    /// Returns the `VirtualAddr` represents the base address of the user
    /// process's stack.
    pub fn get_stack_base() -> VirtualAddr {
        unimplemented!();
    }

    /// Returns the `VirtualAddr` represents the top of the user process's
    /// stack.
    pub fn get_stack_top() -> VirtualAddr {
        unimplemented!();
    }

    /// Returns `true` if this process is ready to be scheduled.
    ///
    /// This functions returns `true` only if one of the following holds:
    ///
    ///   * The state is currently `Ready`.
    ///
    ///   * An event being waited for has arrived.
    ///
    ///     If the process is currently waiting, the corresponding event
    ///     function is polled to determine if the event being waiting for has
    ///     occured. If it has, the state is switched to `Ready` and this
    ///     function returns `true`.
    ///
    /// Returns `false` in all other cases.
    pub fn is_ready(&mut self) -> bool {
        unimplemented!("Process::is_ready()")
    }
}
