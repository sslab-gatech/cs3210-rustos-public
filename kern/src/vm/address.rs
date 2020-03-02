use core::fmt;
use core::ops::{Add, AddAssign, BitAnd, BitOr, Sub, SubAssign};

/// A virtual address.
#[derive(Copy, Clone, PartialEq)]
pub struct VirtualAddr(usize);

/// A physical address.
#[derive(Copy, Clone, PartialEq)]
pub struct PhysicalAddr(usize);

// FIXME: Implement `Add`, `AddAssign`, `Sub`, `SubAssign`, `BitAnd`,
// and `BitOr` for `VirtualAddr` and `PhysicalAddr`.

macro_rules! impl_for {
    ($T:tt) => {
        impl Add for $T {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                $T(self.0.wrapping_add(other.0))
            }
        }

        impl AddAssign for $T {
            fn add_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_add(other.0)
            }
        }

        impl Sub for $T {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                $T(self.0.wrapping_sub(other.0))
            }
        }

        impl SubAssign for $T {
            fn sub_assign(&mut self, other: Self) {
                self.0 = self.0.wrapping_sub(other.0)
            }
        }

        impl BitAnd for $T {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                $T(self.0 & rhs.0)
            }
        }

        impl BitOr for $T {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                $T(self.0 | rhs.0)
            }
        }

        impl<T: Sized> From<*mut T> for $T {
            fn from(raw_ptr: *mut T) -> $T {
                $T(raw_ptr as usize)
            }
        }

        impl<T: Sized> From<*const T> for $T {
            fn from(raw_ptr: *const T) -> $T {
                $T(raw_ptr as usize)
            }
        }

        impl From<usize> for $T {
            fn from(raw_ptr: usize) -> $T {
                $T(raw_ptr)
            }
        }

        impl From<u64> for $T {
            fn from(raw_ptr: u64) -> $T {
                $T(raw_ptr as usize)
            }
        }

        impl From<i32> for $T {
            fn from(raw_ptr: i32) -> $T {
                $T(raw_ptr as usize)
            }
        }

        impl $T {
            /// Returns the inner address of `self`.
            pub fn as_ptr(&self) -> *const u8 {
                self.0 as *const u8
            }

            /// Returns the inner address of `self`.
            pub fn as_mut_ptr(&mut self) -> *mut u8 {
                self.0 as *mut u8
            }

            /// Returns the inner address of `self` as a `usize`.
            pub fn as_usize(&self) -> usize {
                self.0
            }

            /// Returns the inner address of `self` as a `u64`.
            #[cfg(target_pointer_width = "64")]
            pub fn as_u64(&self) -> u64 {
                self.0 as u64
            }
        }

        impl fmt::Debug for $T {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}({:#016x})", stringify!($T), self.0)
            }
        }
    };
}

impl_for!(VirtualAddr);
impl_for!(PhysicalAddr);
