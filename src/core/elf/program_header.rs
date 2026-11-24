use crate::core::elf::program_header::PType::Unknown;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum PType {
    PtNull,
    PtLoad,
    PtDynamic,
    PtInterp,
    PtNote,
    PtPHDR,
    PtTLS,
    Unknown(u32),
}

impl PType {
    pub fn from_bytes(bytes: [u8; 4]) -> Self {
        match u32::from_le_bytes(bytes) {
            0 => PType::PtNull,
            1 => PType::PtLoad,
            2 => PType::PtDynamic,
            3 => PType::PtInterp,
            4 => PType::PtNote,
            6 => PType::PtPHDR,
            7 => PType::PtTLS,
            _ => PType::Unknown(u32::from_le_bytes(bytes)),
        }
    }
}

impl Default for PType {
    fn default() -> Self {
        Unknown(0)
    }
}

#[derive(Debug, Default)]
pub struct ProgramHeader {
    pub p_type: PType,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Display for ProgramHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} off={:#x} vaddr={:#x} paddr={:#x} filesz={:#x} memsz={:#x} flags={} align={:#x}",
            self.p_type,
            self.p_offset,
            self.p_vaddr,
            self.p_paddr,
            self.p_filesz,
            self.p_memsz,
            self.p_flags,
            self.p_align,
        )
    }
}

impl ProgramHeader {
    pub const fn new(
        p_type: PType,
        p_flags: u32,
        p_offset: u64,
        p_vaddr: u64,
        p_paddr: u64,
        p_filesz: u64,
        p_memsz: u64,
        p_align: u64,
    ) -> Self {
        Self {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        }
    }
}
