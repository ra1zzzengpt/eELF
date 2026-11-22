use crate::core::elfclass::ELFClass;
use crate::core::elfendianness::ELFEndianness;
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
        write!(
            f,
            "ELF: {:^5} Class: {:^5} Endianness: {:^5}\n\
            Version: {:^5} ABI: {:^5} ABI version: {:^5}\n\
            Padding: {:^5}",
            term_ansi::green!("{}", self.ident),
            term_ansi::green!("{}", self.class),
            term_ansi::green!("{}", self.endianness),
            term_ansi::green!("{}", self.version),
            term_ansi::green!("{}", self.abi),
            term_ansi::green!("{}", self.abi_version),
            term_ansi::green!("{:?}", self.padding)
        )
    }
}
