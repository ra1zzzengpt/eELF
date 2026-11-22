use crate::core::elfidentity::ELFIdentity;
use crate::core::elfinfo::ELFInfo;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ELFHeader {
    pub identity: ELFIdentity,
    pub info: ELFInfo,
}
impl ELFHeader {
    pub fn new() -> Self {
        ELFHeader {
            identity: ELFIdentity::new(),
            info: ELFInfo::new(),
        }
    }
}

impl Display for ELFHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.identity, self.info)
    }
}
