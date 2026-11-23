use crate::core::elfclass::ELFClass;
use crate::core::elfendianness::ELFEndianness;
use crate::core::elfheader::ELFHeader;
use crate::core::elfidentity::ELFIdentity;
use crate::core::elfinfo::ELFInfo;
use crate::core::elfmachine::ELFMachine;
use crate::core::elftype::ELFType;
use crate::core::error::ErrorType::ReadFailed;
use crate::core::error::{Error, ErrorType};
use crate::core::program_header::{PType, ProgramHeader};
use std::io::{Read, Seek, SeekFrom};

fn read_bytes(file: &mut std::fs::File, count: u64) -> Result<Vec<u8>, Error> {
    let mut buffer = vec![0u8; count as usize];
    file.read_exact(buffer.as_mut_slice())
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    Ok(buffer)
}

fn read_program_header(file: &mut std::fs::File) -> Result<ProgramHeader, Error> {
    let mut program_header: ProgramHeader = ProgramHeader::default();
    program_header.p_type = PType::from_bytes(
        read_bytes(file, 4)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take error".to_string()))?,
    );
    program_header.p_flags = u32::from_le_bytes(
        read_bytes(file, 4)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take flags".to_string()))?,
    );
    program_header.p_offset = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take offset".to_string()))?,
    );
    program_header.p_vaddr = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take vaddr".to_string()))?,
    );
    program_header.p_paddr = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take paddr".to_string()))?,
    );
    program_header.p_filesz = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take filesz".to_string()))?,
    );
    program_header.p_memsz = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take memsz".to_string()))?,
    );
    program_header.p_align = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take offset".to_string()))?,
    );
    Ok(program_header)
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
    info.e_version = u32::from_le_bytes(
        read_bytes(file, 4)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take version".to_string()))?,
    );
    info.e_entry = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take entry".to_string()))?,
    );
    info.e_phoff = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take phoff".to_string()))?,
    );
    info.e_shoff = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take shoff".to_string()))?,
    );
    info.e_flags = u32::from_le_bytes(
        read_bytes(file, 4)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take flags".to_string()))?,
    );
    info.e_ehsize = u16::from_le_bytes(
        read_bytes(file, 2)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take ehsize".to_string()))?,
    );
    info.e_phentsize = u16::from_le_bytes(
        read_bytes(file, 2)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take phentsize".to_string()))?,
    );
    info.e_phnum = u16::from_le_bytes(
        read_bytes(file, 2)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take phnum".to_string()))?,
    );
    info.e_shentsize = u16::from_le_bytes(
        read_bytes(file, 2)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take shentsize".to_string()))?,
    );
    info.e_shnum = u16::from_le_bytes(
        read_bytes(file, 2)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take shnum".to_string()))?,
    );
    info.e_shstrndx = u16::from_le_bytes(
        read_bytes(file, 2)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "can't take shstrndx".to_string()))?,
    );
    file.seek(SeekFrom::Start(info.e_phoff))
        .map_err(|_| Error::new(ReadFailed, "can't seek file".to_string()))?;
    info.program_headers.reserve(info.e_phnum as usize);
    for _ in 0..info.e_phnum {
        info.program_headers.push(read_program_header(file)?);
    }
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
