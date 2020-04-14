use core::iter::Chain;
use core::ops::{Deref, DerefMut};
use core::slice::Iter;

use alloc::boxed::Box;
use alloc::fmt;
use core::alloc::{GlobalAlloc, Layout};

use crate::allocator;
use crate::param::*;
use crate::vm::{PhysicalAddr, VirtualAddr};
use crate::ALLOCATOR;

use aarch64::vmsa::*;
use shim::const_assert_size;

#[repr(C)]
pub struct Page([u8; PAGE_SIZE]);
const_assert_size!(Page, PAGE_SIZE);

impl Page {
    pub const SIZE: usize = PAGE_SIZE;
    pub const ALIGN: usize = PAGE_SIZE;

    fn layout() -> Layout {
        unsafe { Layout::from_size_align_unchecked(Self::SIZE, Self::ALIGN) }
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct L2PageTable {
    pub entries: [RawL2Entry; 8192],
}
const_assert_size!(L2PageTable, PAGE_SIZE);

impl L2PageTable {
    /// Returns a new `L2PageTable`
    fn new() -> L2PageTable {
        unimplemented!("L2PageTable::new()")
    }

    /// Returns a `PhysicalAddr` of the pagetable.
    pub fn as_ptr(&self) -> PhysicalAddr {
        unimplemented!("L2PageTable::as_ptr()")
    }
}

#[derive(Copy, Clone)]
pub struct L3Entry(RawL3Entry);

impl L3Entry {
    /// Returns a new `L3Entry`.
    fn new() -> L3Entry {
        unimplemented!("L3Entry::new()")
    }

    /// Returns `true` if the L3Entry is valid and `false` otherwise.
    fn is_valid(&self) -> bool {
        unimplemented!("L3Entry::is_valid()")
    }

    /// Extracts `ADDR` field of the L3Entry and returns as a `PhysicalAddr`
    /// if valid. Otherwise, return `None`.
    fn get_page_addr(&self) -> Option<PhysicalAddr> {
        unimplemented!("LeEntry::get_page_add()")
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct L3PageTable {
    pub entries: [L3Entry; 8192],
}
const_assert_size!(L3PageTable, PAGE_SIZE);

impl L3PageTable {
    /// Returns a new `L3PageTable`.
    fn new() -> L3PageTable {
        unimplemented!("L3PageTable::new()")
    }

    /// Returns a `PhysicalAddr` of the pagetable.
    pub fn as_ptr(&self) -> PhysicalAddr {
        unimplemented!("L3PageTable::as_ptr()")
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct PageTable {
    pub l2: L2PageTable,
    pub l3: [L3PageTable; 3],
}

impl PageTable {
    /// Returns a new `Box` containing `PageTable`.
    /// Entries in L2PageTable should be initialized properly before return.
    fn new(perm: u64) -> Box<PageTable> {
        unimplemented!("PageTable::new()")
    }

    /// Returns the (L2index, L3index) extracted from the given virtual address.
    /// L2index should be smaller than the number of L3PageTable.
    ///
    /// # Panics
    ///
    /// Panics if the virtual address is not properly aligned to page size.
    /// Panics if extracted L2index exceeds the number of L3PageTable.
    fn locate(va: VirtualAddr) -> (usize, usize) {
        unimplemented!("PageTable::localte()")
    }

    /// Returns `true` if the L3entry indicated by the given virtual address is valid.
    /// Otherwise, `false` is returned.
    pub fn is_valid(&self, va: VirtualAddr) -> bool {
        unimplemented!("PageTable::is_valid()")
    }

    /// Returns `true` if the L3entry indicated by the given virtual address is invalid.
    /// Otherwise, `true` is returned.
    pub fn is_invalid(&self, va: VirtualAddr) -> bool {
        unimplemented!("PageTable::is_invalid()")
    }

    /// Set the given RawL3Entry `entry` to the L3Entry indicated by the given virtual
    /// address.
    pub fn set_entry(&mut self, va: VirtualAddr, entry: RawL3Entry) -> &mut Self {
        unimplemented!("PageTable::set_entry()")
    }

    /// Returns a base address of the pagetable. The returned `PhysicalAddr` value
    /// will point the start address of the L2PageTable.
    pub fn get_baddr(&self) -> PhysicalAddr {
        unimplemented!("PageTable::get_baddr()")
    }
}

// FIXME: Implement `IntoIterator` for `&PageTable`.

pub struct KernPageTable(Box<PageTable>);

impl KernPageTable {
    /// Returns a new `KernPageTable`. `KernPageTable` should have a `Pagetable`
    /// created with `KERN_RW` permission.
    ///
    /// Set L3entry of ARM physical address starting at 0x00000000 for RAM and
    /// physical address range from `IO_BASE` to `IO_BASE_END` for peripherals.
    /// Each L3 entry should have correct value for lower attributes[10:0] as well
    /// as address[47:16]. Refer to the definition of `RawL3Entry` in `vmsa.rs` for
    /// more details.
    pub fn new() -> KernPageTable {
        unimplemented!("KernPageTable::new()")
    }
}

pub enum PagePerm {
    RW,
    RO,
    RWX,
}

pub struct UserPageTable(Box<PageTable>);

impl UserPageTable {
    /// Returns a new `UserPageTable` containing a `PageTable` created with
    /// `USER_RW` permission.
    pub fn new() -> UserPageTable {
        unimplemented!("UserPageTable::new()")
    }

    /// Allocates a page and set an L3 entry translates given virtual address to the
    /// physical address of the allocated page. Returns the allocated page.
    ///
    /// # Panics
    /// Panics if the virtual address is lower than `USER_IMG_BASE`.
    /// Panics if the virtual address has already been allocated.
    /// Panics if allocator fails to allocate a page.
    ///
    /// TODO. use Result<T> and make it failurable
    /// TODO. use perm properly
    pub fn alloc(&mut self, va: VirtualAddr, _perm: PagePerm) -> &mut [u8] {
        unimplemented!("alloc()")
    }
}

impl Deref for KernPageTable {
    type Target = PageTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for UserPageTable {
    type Target = PageTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KernPageTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for UserPageTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// FIXME: Implement `Drop` for `UserPageTable`.
// FIXME: Implement `fmt::Debug` as you need.
