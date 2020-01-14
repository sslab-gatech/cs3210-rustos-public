/// Trait implemented by all of the wrapper types in this crate.
///
/// The inner type of wrapper is specified as an associated constant `Inner`.
/// This allows for generic implementations over all of the wrapper types.
pub trait Wrapper {
    /// The type of the wrapped value.
    type Inner;

    /// Returns a pointer to the wrapped item.
    fn ptr(&self) -> *const Self::Inner;
}

/// Trait implemented by **readable** volatile wrappers.
pub trait Readable<T> {
    /// Returns the inner pointer.
    #[inline(always)]
    fn inner(&self) -> *const T;

    /// Reads and returns the value pointed to by `self`. The read is always
    /// done using volatile semantics.
    #[inline(always)]
    fn read(&self) -> T {
        unsafe { ::core::ptr::read_volatile(self.inner()) }
    }

    /// Returns `true` if the value pointed to by `self` has the mask `mask`.
    /// This is equivalent to `(self.read() & mask) == mask`.
    #[inline(always)]
    fn has_mask(&self, mask: T) -> bool
        where T: ::core::ops::BitAnd<Output = T>,
              T: PartialEq + Copy
    {
        (self.read() & mask) == mask
    }
}

/// Trait implemented by **writeable** volatile wrappers.
pub trait Writeable<T> {
    /// Returns the inner pointer.
    #[inline(always)]
    fn inner(&mut self) -> *mut T;

    /// Writes the value `val` to the inner address of `self`. The write is
    /// always done using volatile semantics.
    #[inline(always)]
    fn write(&mut self, val: T) {
        unsafe { ::core::ptr::write_volatile(self.inner(), val) }
    }
}

/// Trait implemented by **readable _and_ writeable** volatile wrappers.
pub trait ReadableWriteable<T>: Readable<T> + Writeable<T>
    where T: ::core::ops::BitAnd<Output = T>,
          T: ::core::ops::BitOr<Output = T>
{
    /// Applies the mask `mask` using `&` to the value referred to by `self`.
    /// This is equivalent to `self.write(self.read() & mask)`.
    fn and_mask(&mut self, mask: T) {
        let init_val = self.read();
        self.write(init_val & mask);
    }

    /// Applies the mask `mask` using `|` to the value referred to by `self`.
    /// This is equivalent to `self.write(self.read() | mask)`.
    fn or_mask(&mut self, mask: T) {
        let init_val = self.read();
        self.write(init_val | mask);
    }
}

