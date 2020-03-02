use core::alloc::{GlobalAlloc, Layout};
use core::fmt;
use core::ptr::Unique;

use crate::vm::PhysicalAddr;
use crate::ALLOCATOR;

/// A process stack. The default size is 1MiB with an alignment of 16 bytes.
pub struct Stack {
    ptr: Unique<[u8; Stack::SIZE]>,
}

impl Stack {
    /// The default stack size is 1MiB.
    pub const SIZE: usize = 1 << 20;

    /// The default stack alignment is 16 bytes.
    pub const ALIGN: usize = 16;

    /// The default layout for a stack.
    fn layout() -> Layout {
        unsafe { Layout::from_size_align_unchecked(Self::SIZE, Self::ALIGN) }
    }

    /// Returns a newly allocated process stack, zeroed out, if one could be
    /// successfully allocated. If there is no memory, or memory allocation
    /// fails for some other reason, returns `None`.
    pub fn new() -> Option<Stack> {
        let raw_ptr = unsafe {
            let raw_ptr: *mut u8 = ALLOCATOR.alloc(Stack::layout());
            assert!(!raw_ptr.is_null());
            raw_ptr.write_bytes(0, Self::SIZE);
            raw_ptr
        };

        let ptr = Unique::new(raw_ptr as *mut _).expect("non-null");
        Some(Stack { ptr })
    }

    /// Internal method to cast to a `*mut u8`.
    unsafe fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr() as _
    }

    /// Returns the physical address of top of the stack.
    pub fn top(&self) -> PhysicalAddr {
        unsafe { self.as_mut_ptr().add(Self::SIZE).into() }
    }

    /// Returns the physical address of bottom of the stack.
    pub fn bottom(&self) -> PhysicalAddr {
        unsafe { self.as_mut_ptr().into() }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { ALLOCATOR.dealloc(self.as_mut_ptr(), Self::layout()) }
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Stack")
            .field("top", &self.top())
            .field("bottom", &self.bottom())
            .field("size", &Self::SIZE)
            .finish()
    }
}
