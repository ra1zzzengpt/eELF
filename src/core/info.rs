use crate::core::error::Error;
use crate::core::error::ErrorType::{UnknownELFClass, UnknownELFEndianness, UnknownELFType};
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

#[derive(Debug)]
pub enum ELFType{
    EtRel,
    EtExec,
    EtDyn,
    EtCore,
}

impl Display for ELFType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ELFType::EtRel => write!(f, "ET_REL"),
            ELFType::EtExec => write!(f, "ET_EXEC"),
            ELFType::EtDyn => write!(f, "ET_DYN"),
            ELFType::EtCore => write!(f, "ET_CORE"),
        }
    }
}

pub fn bytes_to_elftype(bytes: [u8; 2]) -> Result<ELFType, Error> {
    let byte = u16::from_le_bytes(bytes);
    match byte {
        0x01 => Ok(ELFType::EtRel),
        0x02 => Ok(ELFType::EtExec),
        0x03 => Ok(ELFType::EtDyn),
        0x04 => Ok(ELFType::EtCore),
        _ => Err(Error::new(UnknownELFType, format!("unknown ELF type: {byte}"))),
    }
}

#[derive(Debug)]
pub enum ELFMachine{
    Intel80386,
    ARM,
    AMDx86_64,
    AArch64,
    RISCV
}

impl Display for ELFMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ELFMachine::Intel80386 => write!(f, "Intel80386"),
            ELFMachine::ARM => write!(f, "ARM"),
            ELFMachine::AMDx86_64 => write!(f, "AMDx86_64"),
            ELFMachine::AArch64 => write!(f, "AArch64"),
            ELFMachine::RISCV => write!(f, "RISCV")
        }
    }
}

pub fn bytes_to_elfmachine(bytes: [u8; 2]) -> Result<ELFMachine, Error> {
    let byte = u16::from_le_bytes(bytes);
    match byte {
        0x03 => Ok(ELFMachine::Intel80386),
        0x28 => Ok(ELFMachine::ARM),
        0x3E => Ok(ELFMachine::AMDx86_64),
        0xB7 => Ok(ELFMachine::AArch64),
        0xF3 => Ok(ELFMachine::RISCV),
        _ => Err(Error::new(UnknownELFType, format!("unknown ELF type: {byte}"))),
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
pub struct ELFIdentity {
    pub ident: bool,
    pub class: ELFClass,
    pub endianness: ELFEndianness,
    pub version: u8,
    pub abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
}

#[derive(Debug)]
pub struct ELFInfo{
    pub e_type : ELFType,
    pub e_machine : ELFMachine,
}

impl ELFInfo {
    pub fn new() -> Self{
        ELFInfo{e_type : ELFType::EtRel, e_machine: ELFMachine::Intel80386}
    }
}
#[derive(Debug)]
pub struct ELFHeader {
    pub identity: ELFIdentity,
    pub info : ELFInfo,
}

impl ELFHeader {
    pub fn new() -> Self{
        ELFHeader{identity : ELFIdentity::new(), info : ELFInfo::new()}
    }
}

impl ELFIdentity {
    pub fn new() -> ELFIdentity {
        ELFIdentity {
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

impl Display for ELFHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}\n{}", self.identity, self.info)
    }
}

impl Display for ELFInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"e_type: {} e_machine: {}",self.e_type, self.e_machine)
    }
}