#[macro_export]
macro_rules! define_mask {
    ($end:expr, $beg:expr) => {((1 << $end) - (1 << $beg) + (1 << $end))};
}

#[macro_export]
macro_rules! define_bitfield {
    ($field:ident, [$($end:tt - $beg:tt)|*]) => {
        #[allow(non_upper_case_globals)]
        pub const $field: u64 = $( define_mask!($end, $beg) )|*;
    };
}

#[macro_export]
macro_rules! defreg {
    ($regname:ident) => { defreg!($regname, []); };
    ($regname:ident, [$($field:ident $bits:tt,)*]) => {
        #[allow(non_snake_case)]
        pub mod $regname {
            pub struct Register;
            impl Register {
                #[inline(always)]
                pub unsafe fn get(&self) -> u64 {
                    let rtn;
                    asm!(concat!("mrs $0, ", stringify!($regname))
                         : "=r"(rtn) ::: "volatile");
                    rtn
                }

                #[inline(always)]
                pub unsafe fn get_masked(&self, mask: u64) -> u64 {
                    let rtn: u64;
                    asm!(concat!("mrs $0, ", stringify!($regname))
                         : "=r"(rtn) ::: "volatile");
                    rtn & mask
                }

                #[inline(always)]
                pub unsafe fn get_value(&self, mask: u64) -> u64 {
                    let rtn: u64;
                    asm!(concat!("mrs $0, ", stringify!($regname))
                         : "=r"(rtn) ::: "volatile");
                    (rtn & mask) >> (mask.trailing_zeros())
                }

                #[inline(always)]
                pub unsafe fn set(&self, val: u64) {
                    asm!(concat!("msr ", stringify!($regname), ", $0")
                         :: "r"(val) :: "volatile");
                }
            }

            #[inline(always)]
            pub fn get_masked(val: u64, mask: u64) -> u64 {
                val & mask
            }

            #[inline(always)]
            pub fn get_value(val: u64, mask: u64) -> u64 {
                (val & mask) >> (mask.trailing_zeros())
            }

            $( define_bitfield!($field, $bits); )*
        }

        #[allow(non_upper_case_globals)]
        pub static $regname: $regname::Register = $regname::Register {};
    }
}

#[macro_export]
macro_rules! defbit {
    ($regname:ident) => { defbit!($regname, []); };
    ($regname:ident, [$($field:ident $bits:tt,)*]) => {
        #[allow(non_snake_case)]
        #[derive(Copy, Clone)]
        pub struct $regname (u64);
        
        impl $regname {
            #[inline(always)]
            pub fn new(data: u64) -> $regname {
                $regname(data)
            }

            #[inline(always)]
            pub fn get(&self) -> u64 {
                self.0
            }

            #[inline(always)]
            pub fn get_masked(&self, mask: u64) -> u64 {
                self.0 & mask
            }

            #[inline(always)]
            pub fn get_value(&self, mask: u64) -> u64 {
                (self.0 & mask) >> (mask.trailing_zeros())
            }

            #[inline(always)]
            pub fn set(&mut self, val: u64) -> &mut Self {
                self.0 = val;
                self
            }

            #[inline(always)]
            pub fn set_masked(&mut self, val: u64, mask: u64) -> &mut Self {
                self.0 = (self.0 & !mask) | (val & mask);
                self
            }

            #[inline(always)]
            pub fn set_value(&mut self, val: u64, mask: u64) -> &mut Self {
                self.0 = (self.0 & !mask)
                    | ((val << (mask.trailing_zeros())) & mask);
                self
            }

            #[inline(always)]
            pub fn set_bit(&mut self, mask: u64) -> &mut Self {
                self.0 |= mask;
                self
            }

            #[inline(always)]
            pub fn clear_bit(&mut self, mask: u64) -> &mut Self {
                self.0 &= !mask;
                self
            }

            $( define_bitfield!($field, $bits); )*
        }
    }
}