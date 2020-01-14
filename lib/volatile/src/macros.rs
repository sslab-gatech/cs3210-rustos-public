use crate::traits::*;

#[doc(hidden)]
pub(crate) macro ptr($type:ident, |$self:ident| $f:expr) {
    impl<T> Wrapper for $type<T> {
        type Inner = T;
        #[inline(always)] fn ptr(&$self) -> *const T { $f }
    }

    use core::fmt;

    impl<T> fmt::Debug for $type<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct(stringify!($type))
                .field("address", &(self.ptr()))
                .field("size", &::core::mem::size_of::<T>())
                .finish()
        }
    }
}

#[doc(hidden)]
pub(crate) macro readable($type:ident, |$self:ident| $f:expr) {
    impl<T> Readable<T> for $type<T> {
        #[inline(always)] fn inner(&$self) -> *const T { $f }
    }
}

#[doc(hidden)]
pub(crate) macro writeable($type:ident, |$self:ident| $f:expr) {
    impl<T> Writeable<T> for $type<T> {
        #[inline(always)] fn inner(&mut $self) -> *mut T { $f }
    }
}

#[doc(hidden)]
pub(crate) macro readable_writeable($type:ident) {
    impl<T> ReadableWriteable<T> for $type<T>
    where
        T: ::core::ops::BitAnd<Output = T>,
        T: ::core::ops::BitOr<Output = T>,
    {
    }
}
