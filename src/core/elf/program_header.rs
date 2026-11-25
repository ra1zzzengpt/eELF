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
    PtGnuEhFrame,
    PtGnuStack,
    PtGnuRelro,
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
            0x6474e550 => PType::PtGnuEhFrame,
            0x6474e551 => PType::PtGnuStack,
            0x6474e552 => PType::PtGnuRelro,
            _ => PType::Unknown(u32::from_le_bytes(bytes)),
        }
    }
}

impl Default for PType {
    fn default() -> Self {
        Unknown(0)
    }
}

impl Display for PType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PType::PtNull => "PT_NULL".to_string(),
            PType::PtLoad => "PT_LOAD".to_string(),
            PType::PtDynamic => "PT_DYNAMIC".to_string(),
            PType::PtInterp => "PT_INTERP".to_string(),
            PType::PtNote => "PT_NOTE".to_string(),
            PType::PtPHDR => "PT_PHDR".to_string(),
            PType::PtTLS => "PT_TLS".to_string(),
            PType::PtGnuEhFrame => "PT_GNU_EH_FRAME".to_string(),
            PType::PtGnuStack => "PT_GNU_STACK".to_string(),
            PType::PtGnuRelro => "PT_GNU_RELRO".to_string(),
            PType::Unknown(value) => format!("UNKNOWN({:#x})", value),
        };
        f.pad(&name)
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
        let flags = format!(
            "{}{}{}",
            if self.p_flags & 4 != 0 { 'R' } else { '-' },
            if self.p_flags & 2 != 0 { 'W' } else { '-' },
            if self.p_flags & 1 != 0 { 'X' } else { '-' }
        );
        write!(
            f,
            "{:<20} {:^11} {:^11} {:^11} {:^11} {:^5} {:^12}",
            self.p_type,
            format!("{:#08x}", self.p_offset),
            format!("{:#08x}", self.p_vaddr),
            format!("{:#08x}", self.p_filesz),
            format!("{:#08x}", self.p_memsz),
            term_ansi::red!("{}", flags),
            format!("{:#010x}", self.p_align)
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
