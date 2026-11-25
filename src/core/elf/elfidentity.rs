use crate::core::elf::elfclass::ELFClass;
use crate::core::elf::elfendianness::ELFEndianness;
use crate::core::error::{Error, ErrorType};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ELFIdentity {
    pub ident: bool,
    pub class: ELFClass,
    pub endianness: ELFEndianness,
    pub version: u8,
    pub abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
}

impl ELFIdentity {
    pub fn identity(bytes: &[u8]) -> Result<(), Error> {
        let ident_bytes: [u8; 4] = bytes.try_into().map_err(|_| {
            Error::new(
                ErrorType::ReadFailed,
                "can't read 4 header bytes".to_string(),
            )
        })?;
        if (ident_bytes[0] == 0x7F)
            && (ident_bytes[1] == 0x45)
            && (ident_bytes[2] == 0x4C)
            && (ident_bytes[3] == 0x46)
        {
            return Ok(());
        }
        Err(Error::new(
            ErrorType::NotELFFile,
            "file is not elf.".to_string(),
        ))
    }
    pub fn new() -> ELFIdentity {
        ELFIdentity {
            ident: false,
            class: ELFClass::Bit32,
            endianness: ELFEndianness::Little,
            version: 0x00,
            abi: 0x00,
            abi_version: 0x00,
            padding: [0x00; 7],
        }
    }
}

impl Display for ELFIdentity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            term_ansi::bold!("{}", term_ansi::cyan!("ELF IDENTIFICATION"))
        )?;
        writeln!(
            f,
            "  {:<18} {}",
            "Magic",
            if self.ident {
                term_ansi::green!("ELF (7f 45 4c 46)")
            } else {
                term_ansi::red!("invalid")
            }
        )?;
        writeln!(
            f,
            "  {:<18} {}",
            "Class",
            term_ansi::green!("{}", self.class)
        )?;
        writeln!(
            f,
            "  {:<18} {}",
            "Byte order",
            term_ansi::green!("{}", self.endianness)
        )?;
        writeln!(
            f,
            "  {:<18} {}",
            "Version",
            term_ansi::green!("{}", self.version)
        )?;
        writeln!(f, "  {:<18} {:#04x}", "OS/ABI", self.abi)?;
        writeln!(f, "  {:<18} {:#04x}", "ABI version", self.abi_version)?;
        write!(f, "  {:<18} {:02x?}", "Padding", self.padding)
    }
}
