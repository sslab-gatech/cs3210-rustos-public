#![feature(decl_macro)]
#![feature(optin_builtin_traits)]

#![no_std]

mod traits;
mod macros;

pub use traits::*;
use macros::*;

/// Reexports all of the traits in this crate.
///
/// The purpose of this module is to alleviate imports of the wrapper traits by
/// adding a glob import to the top of wrapper-heavy modules:
///
/// ```rust
/// use volatile::prelude::*;
/// ```
pub mod prelude {
	#[doc(no_inline)]
    pub use super::{Readable, Writeable, ReadableWriteable, Wrapper};
}

/// A wrapper type that enforces **read-only** _volatile_ accesses to a raw
/// pointer.
#[repr(C)]
pub struct ReadVolatile<T>(T);

/// A wrapper type that enforces _volatile_ (read **or** write) accesses to a
/// raw pointer.
#[repr(C)]
pub struct Volatile<T>(T);

/// A wrapper type that enforces **write-only** _volatile_ accesses to a raw
/// pointer.
#[repr(C)]
pub struct WriteVolatile<T>(T);

/// A wrapper type that prevents read or writes to its value.
///
/// This type implements no methods. It is meant to make the inner type
/// inaccessible to prevent accidental reads or writes.
#[repr(C)]
pub struct Reserved<T>(T);

/// A wrapper over all other wrapper types that implements `Sync`.
///
/// `Sync` is implemented if the wrapper wrapper type's generic is `T: Sync`.
/// For instance, a type of `Unique<Volatile<T>>` is `Sync` if `T` is `Sync`.
#[repr(C)]
#[derive(Debug)]
pub struct Unique<T>(T);

// Implementations for `ReadVolatile`.
ptr!(ReadVolatile, |self| &self.0);
readable!(ReadVolatile, |self| &self.0);
unsafe impl<T: Send> Send for ReadVolatile<T> {  }
impl<T> !Sync for ReadVolatile<T> {  }

// Implementations for `Volatile`.
ptr!(Volatile, |self| &self.0);
readable!(Volatile, |self| &self.0);
writeable!(Volatile, |self| &mut self.0);
readable_writeable!(Volatile);
unsafe impl<T: Send> Send for Volatile<T> {  }
impl<T> !Sync for Volatile<T> {  }

// Implementations for `WriteVolatile`.
writeable!(WriteVolatile, |self| &mut self.0);
ptr!(WriteVolatile, |self| &self.0);
unsafe impl<T: Send> Send for WriteVolatile<T> {  }
impl<T> !Sync for WriteVolatile<T> {  }

// Implementations for `Reserved`.
ptr!(Reserved, |self| &self.0);

// Implementations for `Unique`.
unsafe impl<R: Wrapper> Send for Unique<R> where <R as Wrapper>::Inner: Send {  }
unsafe impl<R: Wrapper> Sync for Unique<R> where <R as Wrapper>::Inner: Sync {  }

impl<T, R: Readable<T>> Readable<T> for Unique<R> {
    #[inline(always)]
    fn inner(&self) -> *const T {
        self.0.inner()
    }
}

impl<T, R: Writeable<T>> Writeable<T> for Unique<R> {
    #[inline(always)]
    fn inner(&mut self) -> *mut T {
        self.0.inner()
    }
}

impl<T, R: ReadableWriteable<T>> ReadableWriteable<T> for Unique<R>
    where T: ::core::ops::BitAnd<Output = T>, T: ::core::ops::BitOr<Output = T> { }
