use crate::core::error::Error;
use crate::core::error::ErrorType::UnknownELFClass;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
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

impl ELFClass {
    pub fn from_byte(byte: &u8) -> Result<Self, Error> {
        match byte {
            0x01 => Ok(ELFClass::Bit32),
            0x02 => Ok(ELFClass::Bit64),
            _ => Err(Error::new(
                UnknownELFClass,
                format!("unknown class byte: {byte}"),
            )),
        }
    }
}
