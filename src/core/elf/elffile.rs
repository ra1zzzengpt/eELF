use crate::core::elf::dynamic_header::DynamicHeader;
use crate::core::elf::elfheader::ELFHeader;
use crate::core::elf::elfinfo::ELFInfo;
use crate::core::elf::program_header::ProgramHeader;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ELFFile {
    pub elf_header: ELFHeader,
    pub elf_info: ELFInfo,
    pub program_headers: Vec<ProgramHeader>,
    pub dynamic_headers: Vec<DynamicHeader>,
    pub dependencies: Vec<String>,
}

impl ELFFile {
    pub fn new() -> ELFFile {
        Self {
            elf_header: ELFHeader::new(),
            elf_info: ELFInfo::new(),
            program_headers: Vec::new(),
            dynamic_headers: Vec::new(),
            dependencies: Vec::new(),
        }
    }
}

impl Display for ELFFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", term_ansi::bold!("{}", term_ansi::cyan!("ELF FILE")))?;
        writeln!(f, "{}", term_ansi::cyan!("========================================================"))?;
        writeln!(f)?;

        writeln!(f, "{}", term_ansi::bold!("IDENTIFICATION"))?;
        writeln!(f, "{}", self.elf_header.identity)?;
        writeln!(f)?;

        writeln!(f, "{}", term_ansi::bold!("ELF HEADER"))?;
        writeln!(f, "{}", self.elf_info)?;

        writeln!(f, "{}", term_ansi::bold!("PROGRAM HEADERS"))?;
        if self.program_headers.is_empty() {
            writeln!(f, "  {}", term_ansi::yellow!("(none)"))?;
        } else {
            writeln!(f, "  {:<12} {:>18} {:>18} {:>12} {:>12} {:>8} {:>10}",
                "TYPE", "OFFSET", "VADDR", "FILE SIZE", "MEM SIZE", "FLAGS", "ALIGN")?;
            for header in &self.program_headers {
                writeln!(f, "  {:<12?} {:#018x} {:#018x} {:#012x} {:#012x} {:>8} {:#010x}",
                    header.p_type, header.p_offset, header.p_vaddr, header.p_filesz,
                    header.p_memsz, header.p_flags, header.p_align)?;
            }
        }
        writeln!(f)?;

        writeln!(f, "{}", term_ansi::bold!("DYNAMIC ENTRIES"))?;
        if self.dynamic_headers.is_empty() {
            writeln!(f, "  {}", term_ansi::yellow!("(none)"))?;
        } else {
            for (index, header) in self.dynamic_headers.iter().enumerate() {
                writeln!(f, "  {:>3}  {}", index, header)?;
            }
        }
        writeln!(f)?;

        writeln!(f, "{}", term_ansi::bold!("DEPENDENCIES"))?;
        if self.dependencies.is_empty() {
            writeln!(f, "  {}", term_ansi::yellow!("(none)"))?;
        } else {
            for dependency in &self.dependencies {
                writeln!(f, "  {} {}", term_ansi::green!("•"), dependency)?;
            }
        }
        Ok(())
    }
}
