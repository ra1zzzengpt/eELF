use crate::core::error::Error;
use crate::core::error::ErrorType::UnknownELFEndianness;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
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

impl ELFEndianness {
    pub fn from_byte(byte: &u8) -> Result<Self, Error> {
        match byte {
            0x01 => Ok(ELFEndianness::Little),
            0x02 => Ok(ELFEndianness::Big),
            _ => Err(Error::new(
                UnknownELFEndianness,
                format!("unknown endianness byte: {byte}"),
            )),
        }
    }
}
