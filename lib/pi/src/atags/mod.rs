mod atag;
mod raw;

pub use self::atag::*;

/// The address at which the firmware loads the ATAGS.
const ATAG_BASE: usize = 0x100;

/// An iterator over the ATAGS on this system.
pub struct Atags {
    ptr: Option<&'static raw::Atag>,
}

impl Atags {
    /// Returns an instance of `Atags`, an iterator over ATAGS on this system.
    pub fn get() -> Atags {
        Atags {
            ptr: Some(unsafe { &*(ATAG_BASE as *const raw::Atag) }),
        }
    }
}

impl Iterator for Atags {
    type Item = Atag;

    // FIXME: Implement `Iterator` for `Atags`
    fn next(&mut self) -> Option<Atag> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::{raw, Atag, Atags};

    const MEM: [u32; 23] = [
        // CORE
        5,
        raw::Atag::CORE,
        1,
        2,
        3,
        // MEM
        4,
        raw::Atag::MEM,
        1234,
        5678,
        // UNKNOWN
        3,
        raw::Atag::RAMDISK,
        1010,
        // CMDLINE
        4,
        raw::Atag::CMDLINE,
        1819043176,
        111,
        // UNKNOWN
        5,
        raw::Atag::REVISION,
        123,
        456,
        789,
        // NONE
        2,
        raw::Atag::NONE,
    ];

    #[test]
    fn test_atags() {
        let mut atags = Atags {
            ptr: Some(unsafe { &*(&MEM as *const u32 as *const raw::Atag) }),
        };

        assert_eq!(
            atags.next(),
            Some(Atag::Core(raw::Core {
                flags: 1,
                page_size: 2,
                root_dev: 3
            }))
        );

        assert_eq!(
            atags.next(),
            Some(Atag::Mem(raw::Mem {
                size: 1234,
                start: 5678,
            }))
        );

        assert_eq!(atags.next(), Some(Atag::Unknown(raw::Atag::RAMDISK)));

        assert_eq!(atags.next(), Some(Atag::Cmd("hello")));

        assert_eq!(atags.next(), Some(Atag::Unknown(raw::Atag::REVISION)));

        assert_eq!(atags.next(), Some(Atag::None));

        assert_eq!(atags.next(), None);
        assert_eq!(atags.next(), None);
        assert_eq!(atags.next(), None);
    }
}
