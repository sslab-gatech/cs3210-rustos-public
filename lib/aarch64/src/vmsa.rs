use core::fmt;

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EntryType {
    pub const Block: u64 = 0;
    pub const Table: u64 = 1;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod PageType {
    pub const Page: u64 = 1;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EntryValid {
    pub const Invalid: u64 = 0;
    pub const Valid: u64 = 1;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EntryPerm {
    pub const KERN_RW: u64 = 0b00;
    pub const USER_RW: u64 = 0b01;
    pub const KERN_RO: u64 = 0b10;
    pub const USER_RO: u64 = 0b11;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EntrySh {
    pub const ISh: u64 = 0b11;
    pub const OSh: u64 = 0b10;
}

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EntryAttr {
    pub const Mem: u64 = 0b000;
    pub const Dev: u64 = 0b001;
    pub const Nc: u64 = 0b010;
}

defbit!(
    RawL2Entry,
    [
        ADDR[47 - 16],
        AF[10 - 10],
        SH[09 - 08],
        AP[07 - 06],
        NS[05 - 05],
        ATTR[04 - 02],
        TYPE[01 - 01],
        VALID[00 - 00],
    ]
);

defbit!(
    RawL3Entry,
    [
        ADDR[47 - 16],
        AF[10 - 10],
        SH[09 - 08],
        AP[07 - 06],
        NS[05 - 05],
        ATTR[04 - 02],
        TYPE[01 - 01],
        VALID[00 - 00],
    ]
);

// (ref: D7.2.70: Memory Attribute Indirection Register)
defreg!(
    MAIR_EL1,
    [
        Attr7[63 - 56],
        Attr6[55 - 48],
        Attr5[47 - 40],
        Attr4[39 - 32],
        Attr3[31 - 24],
        Attr2[23 - 16],
        Attr1[15 - 08],
        Attr0[07 - 00],
    ]
);

// (ref: D7.2.91: Translation Control Register)
defreg!(TCR_EL1);

// (ref: D7.2.99: Translation Table Base Register 0)
defreg!(TTBR0_EL1, [TTBR_CNP[00 - 00],]);

// (ref: D7.2.102: Translation Table Base Register 1)
defreg!(TTBR1_EL1, [TTBR_CNP[00 - 00],]);

// (ref: D7.2.43: AArch64 Memory Model Feature Register 0)
defreg!(
    ID_AA64MMFR0_EL1,
    [
        TGran4[31 - 28],
        TGran64[27 - 24],
        TGran16[23 - 20],
        BigEndEL0[19 - 16],
        SNSMem[15 - 12],
        BigEnd[11 - 08],
        ASIDBits[07 - 04],
        PARange[03 - 00],
    ]
);

// For Phase5, (ref: 7.2.86: Implementation Defined Registers)
defreg!(S3_1_C15_C2_1);
// <<

impl fmt::Debug for RawL2Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.get_value(RawL2Entry::VALID) == EntryValid::Valid {
            write!(f, "V")?;
        } else {
            write!(f, "I")?;
        }

        if self.get_value(RawL2Entry::TYPE) == EntryType::Block {
            write!(f, "B")?;
        } else {
            write!(f, "T")?;
        }

        write!(
            f,
            "-> {:x} ({:x})",
            self.get_masked(RawL2Entry::ADDR),
            self.get()
        )
    }
}

impl fmt::Debug for RawL3Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.get_value(RawL3Entry::VALID) {
                EntryValid::Valid => "V",
                EntryValid::Invalid => "I",
                _ => "?",
            }
        )?;

        write!(
            f,
            "{}",
            match self.get_value(RawL3Entry::TYPE) {
                PageType::Page => "P",
                _ => "?",
            }
        )?;

        write!(
            f,
            "{}",
            match self.get_value(RawL3Entry::ATTR) {
                EntryAttr::Mem => "M",
                EntryAttr::Dev => "D",
                EntryAttr::Nc => "N",
                _ => "?",
            }
        )?;

        write!(
            f,
            "{}",
            match self.get_value(RawL3Entry::SH) {
                EntrySh::ISh => "I",
                EntrySh::OSh => "O",
                _ => "?",
            }
        )?;

        write!(
            f,
            "{}",
            match self.get_value(RawL3Entry::AF) {
                0 => "I",
                1 => "A",
                _ => "?",
            }
        )?;

        write!(
            f,
            "|{}",
            match self.get_value(RawL3Entry::AP) {
                EntryPerm::KERN_RW => "KERN-RW",
                EntryPerm::USER_RW => "USER-RW",
                EntryPerm::KERN_RO => "KERN-RO",
                EntryPerm::USER_RO => "USER-RO",
                _ => "????-??",
            }
        )?;

        // NS    [05-05],

        write!(
            f,
            "-> {:08x} ({:x})",
            self.get_masked(RawL3Entry::ADDR),
            self.get()
        )
    }
}
