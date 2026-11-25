use crate::core::elf::elfmachine::ELFMachine;
use crate::core::elf::elftype::ELFType;
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
        }
    }
}

impl Display for ELFInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            term_ansi::bold!("{}", term_ansi::cyan!("ELF HEADER"))
        )?;
        writeln!(f, "  {:<22} {}", "Type", self.e_type)?;
        writeln!(f, "  {:<22} {}", "Machine", self.e_machine)?;
        writeln!(f, "  {:<22} {}", "Version", self.e_version)?;
        writeln!(
            f,
            "  {:<22} {}",
            "Entry point",
            term_ansi::green!("{:#018x}", self.e_entry)
        )?;
        writeln!(f, "  {:<22} {:#010x}", "Flags", self.e_flags)?;
        writeln!(f, "  {:<22} {} bytes", "ELF header size", self.e_ehsize)?;
        writeln!(f)?;
        writeln!(f, "{}", term_ansi::bold!("TABLE LOCATIONS"))?;
        writeln!(
            f,
            "  {:<22} {} entries at {:#018x} ({} bytes each)",
            "Program headers", self.e_phnum, self.e_phoff, self.e_phentsize
        )?;
        writeln!(
            f,
            "  {:<22} {} entries at {:#018x} ({} bytes each)",
            "Section headers", self.e_shnum, self.e_shoff, self.e_shentsize
        )?;
        write!(f, "  {:<22} {}", "Section names index", self.e_shstrndx)
    }
}
