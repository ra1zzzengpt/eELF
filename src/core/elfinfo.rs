use crate::core::elfmachine::ELFMachine;
use crate::core::elftype::ELFType;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ELFInfo {
    pub e_type: ELFType,
    pub e_machine: ELFMachine,
}

impl ELFInfo {
    pub fn new() -> Self {
        ELFInfo {
            e_type: ELFType::EtRel,
            e_machine: ELFMachine::Intel80386,
        }
    }
}

impl Display for ELFInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "e_type: {} e_machine: {}", self.e_type, self.e_machine)
    }
}
