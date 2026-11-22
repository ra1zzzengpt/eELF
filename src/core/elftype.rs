use crate::core::error::Error;
use crate::core::error::ErrorType::UnknownELFType;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ELFType {
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

impl ELFType {
    pub fn from_bytes(bytes: [u8; 2]) -> Result<Self, Error> {
        let byte = u16::from_le_bytes(bytes);
        match byte {
            0x01 => Ok(ELFType::EtRel),
            0x02 => Ok(ELFType::EtExec),
            0x03 => Ok(ELFType::EtDyn),
            0x04 => Ok(ELFType::EtCore),
            _ => Err(Error::new(
                UnknownELFType,
                format!("unknown ELF type: {byte}"),
            )),
        }
    }
}
