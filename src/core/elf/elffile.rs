use crate::core::elf::dynamic_header::DynamicHeader;
use crate::core::elf::elfheader::ELFHeader;
use crate::core::elf::elfinfo::ELFInfo;
use crate::core::elf::program_header::ProgramHeader;

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

impl ELFFile {
    pub fn display(
        &self,
        show_header: bool,
        show_program_headers: bool,
        show_dynamic_headers: bool,
        show_dependencies: bool,
    ) {
        let mut printed_section = false;
        let mut section = |title: &str| {
            if printed_section {
                println!();
            }
            println!("{}", term_ansi::bold!("{}", term_ansi::cyan!("{}", title)));
            printed_section = true;
        };

        if show_header {
            section("FILE HEADER");
            println!("{}", self.elf_header);
            println!();
            println!("{}", self.elf_info);
        }
        if show_program_headers {
            section("PROGRAM HEADERS");
            if self.program_headers.is_empty() {
                println!("  {}", term_ansi::yellow!("No program headers"));
            } else {
                println!(
                    "  {:<20} {:^11} {:^11} {:^11} {:^11} {:^5} {:^12}",
                    "TYPE", "OFFSET", "VADDR", "FILE SIZE", "MEM SIZE", "FLAGS", "ALIGN"
                );
                for header in &self.program_headers {
                    println!("  {}", header);
                }
            }
        }
        if show_dynamic_headers {
            section("DYNAMIC ENTRIES");
            if self.dynamic_headers.is_empty() {
                println!("  {}", term_ansi::yellow!("No dynamic entries"));
            } else {
                println!("  {:>4}  {:<20}  VALUE", "#", "TAG");
                for (index, header) in self.dynamic_headers.iter().enumerate() {
                    println!("  {:>4}  {}", index, header);
                }
            }
        }
        if show_dependencies {
            section("SHARED LIBRARIES");
            if self.dependencies.is_empty() {
                println!("  {}", term_ansi::yellow!("No shared-library dependencies"));
            } else {
                for (index, dependency) in self.dependencies.iter().enumerate() {
                    println!(
                        "  {}  {}",
                        term_ansi::green!("{:>3}.", index + 1),
                        dependency
                    );
                }
            }
        }
    }
}
