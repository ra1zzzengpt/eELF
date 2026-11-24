use crate::core::elf::elfidentity::ELFIdentity;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ELFHeader {
    pub identity: ELFIdentity,
}
impl ELFHeader {
    pub fn new() -> Self {
        ELFHeader {
            identity: ELFIdentity::new(),
        }
    }
}

impl Display for ELFHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.identity)
    }
}
