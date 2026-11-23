use crate::core::elfmachine::ELFMachine;
use crate::core::elftype::ELFType;
use crate::core::program_header::ProgramHeader;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ELFInfo {
    pub e_type: ELFType,
    pub e_machine: ELFMachine,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
    pub program_headers: Vec<ProgramHeader>,
}

impl ELFInfo {
    pub fn new() -> Self {
        ELFInfo {
            e_type: ELFType::EtRel,
            e_machine: ELFMachine::Intel80386,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
            program_headers: Vec::new(),
        }
    }
}

impl Display for ELFInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "e_type:      {:<12}  e_machine: {:<12}  e_version: {}",
            self.e_type, self.e_machine, self.e_version
        )?;
        writeln!(
            f,
            "e_entry:     {}  e_flags:   {:#010x}  e_ehsize: {}",
            term_ansi::green!("{:#018x}", self.e_entry),
            self.e_flags,
            self.e_ehsize
        )?;
        writeln!(f, "Program Header Table:")?;
        writeln!(
            f,
            "   e_phoff:   {}  e_phnum:   {}  e_phentsize: {}",
            term_ansi::green!("{:#018x}", self.e_phoff),
            self.e_phnum,
            self.e_phentsize
        )?;
        writeln!(f, "Section Header Table:")?;
        writeln!(
            f,
            "   e_shoff:   {}  e_shnum:   {}  e_shentsize: {}",
            term_ansi::green!("{:#018x}", self.e_shoff),
            self.e_shnum,
            self.e_shentsize
        )?;
        writeln!(f, "e_shstrndx: {}", self.e_shstrndx)?;
        writeln!(f, "Program Headers:")?;
        for header in &self.program_headers {
            writeln!(f, "  {}", header)?;
        }
        Ok(())
    }
}
