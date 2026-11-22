use crate::core::error::Error;
use crate::core::error::ErrorType::UnknownELFType;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ELFMachine {
    Intel80386,
    ARM,
    AMDx86_64,
    AArch64,
    RISCV,
}

impl Display for ELFMachine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ELFMachine::Intel80386 => write!(f, "Intel80386"),
            ELFMachine::ARM => write!(f, "ARM"),
            ELFMachine::AMDx86_64 => write!(f, "AMDx86_64"),
            ELFMachine::AArch64 => write!(f, "AArch64"),
            ELFMachine::RISCV => write!(f, "RISCV"),
        }
    }
}

impl ELFMachine {
    pub fn from_bytes(bytes: &[u8]) -> Result<ELFMachine, Error> {
        let array: [u8; 2] = bytes
            .try_into()
            .map_err(|_| Error::new(UnknownELFType, "invalid ELF machine bytes".to_string()))?;
        let byte = u16::from_le_bytes(array);
        match byte {
            0x03 => Ok(ELFMachine::Intel80386),
            0x28 => Ok(ELFMachine::ARM),
            0x3E => Ok(ELFMachine::AMDx86_64),
            0xB7 => Ok(ELFMachine::AArch64),
            0xF3 => Ok(ELFMachine::RISCV),
            _ => Err(Error::new(
                UnknownELFType,
                format!("unknown ELF type: {byte}"),
            )),
        }
    }
}
