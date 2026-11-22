use crate::core::elfclass::ELFClass;
use crate::core::elfendianness::ELFEndianness;
use crate::core::elfheader::ELFHeader;
use crate::core::elfidentity::ELFIdentity;
use crate::core::elfinfo::ELFInfo;
use crate::core::elfmachine::ELFMachine;
use crate::core::elftype::ELFType;
use crate::core::error::ErrorType::ReadFailed;
use crate::core::error::{Error, ErrorType};
use std::io::Read;

fn read_bytes(file: &mut std::fs::File, count: u64) -> Result<Vec<u8>, Error> {
    let mut buffer = vec![0u8; count as usize];
    file.read_exact(buffer.as_mut_slice())
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    Ok(buffer)
}

fn read_identity(file: &mut std::fs::File) -> Result<ELFIdentity, Error> {
    let mut info = ELFIdentity::new();
    info.ident = match ELFIdentity::identity(read_bytes(file, 4)?.as_slice()) {
        Ok(_) => true,
        Err(e) => {
            return Err(e);
        }
    };
    info.class = ELFClass::from_byte(read_bytes(file, 1)?.first().ok_or(Error::new(
        ReadFailed,
        "can't take elf class byte".to_string(),
    ))?)?;
    info.endianness = ELFEndianness::from_byte(read_bytes(file, 1)?.first().ok_or(Error::new(
        ReadFailed,
        "can't take elf endianness byte".to_string(),
    ))?)?;
    info.version = read_bytes(file, 1)?
        .first()
        .ok_or(Error::new(ReadFailed, "can't take version".to_string()))?
        .clone();
    info.abi = read_bytes(file, 1)?
        .first()
        .ok_or(Error::new(ReadFailed, "can't take abi version".to_string()))?
        .clone();
    info.abi_version = read_bytes(file, 1)?
        .first()
        .ok_or(Error::new(ReadFailed, "can't take abi version".to_string()))?
        .clone();
    info.padding = read_bytes(file, 7)?
        .as_slice()
        .try_into()
        .map_err(|_| Error::new(ReadFailed, "can't take padding byte".to_string()))?;
    Ok(info)
}

fn read_info(file: &mut std::fs::File) -> Result<ELFInfo, Error> {
    let mut info = ELFInfo::new();
    info.e_type = ELFType::from_bytes(
        read_bytes(file, 2)?
            .as_slice()
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take ether type".to_string()))?,
    )?;
    info.e_machine = ELFMachine::from_bytes(
        read_bytes(file, 2)?
            .as_slice()
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take machine".to_string()))?,
    )?;
    Ok(info)
}

pub fn read_header(path: String) -> Result<ELFHeader, Error> {
    let mut file = std::fs::File::open(path)
        .map_err(|err| Error::new(ErrorType::CantOpenFile, err.to_string()))?;
    let mut header = ELFHeader::new();
    header.identity = read_identity(&mut file)?;
    header.info = read_info(&mut file)?;
    Ok(header)
}
