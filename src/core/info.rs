use crate::core::error::Error;
use crate::core::error::ErrorType::{UnknownELFClass, UnknownELFEndianness};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ELFClass {
    Bit32,
    Bit64,
}

impl Display for ELFClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ELFClass::Bit32 => write!(f, "32 Bit"),
            ELFClass::Bit64 => write!(f, "64 Bit"),
        }
    }
}

pub fn byte_to_elfclass(byte: u8) -> Result<ELFClass, Error> {
    match byte {
        0x01 => Ok(ELFClass::Bit32),
        0x02 => Ok(ELFClass::Bit64),
        _ => Err(Error::new(
            UnknownELFClass,
            format!("unknown class byte: {byte}"),
        )),
    }
}

#[derive(Debug)]
pub enum ELFEndianness {
    Little,
    Big,
}

impl Display for ELFEndianness {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ELFEndianness::Little => write!(f, "Little endianness"),
            ELFEndianness::Big => write!(f, "Big endianness"),
        }
    }
}

pub fn byte_to_elfendian(byte: u8) -> Result<ELFEndianness, Error> {
    match byte {
        0x01 => Ok(ELFEndianness::Little),
        0x02 => Ok(ELFEndianness::Big),
        _ => Err(Error::new(
            UnknownELFEndianness,
            format!("unknown endianness byte: {byte}"),
        )),
    }
}

#[derive(Debug)]
pub struct ELFInfo {
    pub ident: bool,
    pub class: ELFClass,
    pub endianness: ELFEndianness,
    pub version: u8,
    pub abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
}

impl ELFInfo {
    pub fn new() -> ELFInfo {
        ELFInfo {
            ident: false,
            class: ELFClass::Bit32,
            endianness: ELFEndianness::Little,
            version: 0x00,
            abi: 0x00,
            abi_version: 0x00,
            padding: [0x00;7]
        }
    }
}

impl Display for ELFInfo {
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
