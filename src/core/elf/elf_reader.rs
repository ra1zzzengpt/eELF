use crate::core::elf::dynamic_header::{DynamicHeader, DynamicType};
use crate::core::elf::elfclass::ELFClass;
use crate::core::elf::elfendianness::ELFEndianness;
use crate::core::elf::elffile::ELFFile;
use crate::core::elf::elfheader::ELFHeader;
use crate::core::elf::elfidentity::ELFIdentity;
use crate::core::elf::elfinfo::ELFInfo;
use crate::core::elf::elfmachine::ELFMachine;
use crate::core::elf::elftype::ELFType;
use crate::core::elf::program_header::{PType, ProgramHeader};
use crate::core::error::ErrorType::{CannotFindStrtab, ReadFailed, UnsupportedBinaryFormat};
use crate::core::error::{Error, ErrorType};
use std::io::{Read, Seek, SeekFrom};

fn read_bytes(file: &mut std::fs::File, count: u64) -> Result<Vec<u8>, Error> {
    let mut buffer = vec![0u8; count as usize];
    file.read_exact(buffer.as_mut_slice())
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    Ok(buffer)
}

fn read_string(file: &mut std::fs::File) -> Result<String, Error> {
    let mut buffer: Vec<u8> = Vec::new();
    loop {
        let byte = read_bytes(file, 1)?
            .first()
            .ok_or(Error::new(
                ReadFailed,
                "can't read string bytes".to_string(),
            ))?
            .clone();
        buffer.push(byte);
        if byte == 0x00 {
            break;
        }
    }
    Ok(String::from_utf8(buffer).map_err(|e| Error::new(ReadFailed, e.to_string()))?)
}
fn read_dynamic_strtab(
    file: &mut std::fs::File,
    strtab: u64,
    dynamic_values: Vec<u64>,
    program_headers: Vec<&ProgramHeader>,
) -> Result<Vec<String>, Error> {
    let program_header = program_headers
        .iter()
        .find(|h| (strtab > h.p_vaddr) && (strtab < h.p_vaddr + h.p_filesz))
        .ok_or(Error::new(
            CannotFindStrtab,
            "can't find strtab in program headers".to_string(),
        ))?;
    let mut strings: Vec<String> = Vec::new();
    for dynamic in &dynamic_values {
        let offset = program_header.p_offset + (strtab - program_header.p_vaddr) + dynamic;
        file.seek(SeekFrom::Start(offset))
            .map_err(|e| Error::new(ReadFailed, e.to_string()))?;
        strings.push(read_string(file)?);
    }
    Ok(strings)
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

fn read_program_headers(
    file: &mut std::fs::File,
    e_phnum: u16,
    e_phoff: u64,
) -> Result<Vec<ProgramHeader>, Error> {
    let mut program_headers: Vec<ProgramHeader> = Vec::new();
    file.seek(SeekFrom::Start(e_phoff))
        .map_err(|_| Error::new(ReadFailed, "can't seek file".to_string()))?;
    program_headers.reserve(e_phnum as usize);
    for _ in 0..e_phnum {
        program_headers.push(read_program_header(file)?);
    }
    Ok(program_headers)
}

fn read_dynamic_header(file: &mut std::fs::File) -> Result<DynamicHeader, Error> {
    let mut header = DynamicHeader::new();
    header.tag = DynamicType::from_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take tag".to_string()))?,
    );
    header.value = u64::from_le_bytes(
        read_bytes(file, 8)?
            .try_into()
            .map_err(|_| Error::new(ReadFailed, "type take value".to_string()))?,
    );
    Ok(header)
}

fn read_dynamic_strtable(
    file: &mut std::fs::File,
    program_headers: &Vec<ProgramHeader>,
    dynamic_headers: &Vec<DynamicHeader>,
) -> Result<Vec<String>, Error> {
    let dynamic_values: Vec<u64> = dynamic_headers
        .iter()
        .filter(|header| {
            if header.tag == DynamicType::DtNeeded {
                return true;
            }
            false
        })
        .map(|h| h.value)
        .collect();
    let pt_load_headers: Vec<&ProgramHeader> = program_headers
        .iter()
        .filter(|header| header.p_type == PType::PtLoad)
        .collect();
    let strtable = dynamic_headers
        .iter()
        .find(|header| header.tag == DynamicType::DtStrtab)
        .ok_or(Error::new(
            CannotFindStrtab,
            "dynamic header not found".to_string(),
        ))?
        .value;
    read_dynamic_strtab(file, strtable, dynamic_values, pt_load_headers)
}

fn read_dynamic_headers(
    file: &mut std::fs::File,
    p_offset: u64,
) -> Result<Vec<DynamicHeader>, Error> {
    file.seek(SeekFrom::Start(p_offset))
        .map_err(|_| Error::new(ReadFailed, "can't seek file".to_string()))?;
    let mut dynamic_headers: Vec<DynamicHeader> = Vec::new();
    let mut will_break = false;
    loop {
        let dynamic_header_ = read_dynamic_header(file)?;
        if dynamic_header_.tag == DynamicType::DtNull {
            will_break = true;
        }
        dynamic_headers.push(dynamic_header_);
        if will_break {
            break;
        }
    }
    Ok(dynamic_headers)
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
    if info.class == ELFClass::Bit32 {
        return Err(Error::new(
            UnsupportedBinaryFormat,
            "elf32 not supported".to_string(),
        ));
    }
    info.endianness = ELFEndianness::from_byte(read_bytes(file, 1)?.first().ok_or(Error::new(
        ReadFailed,
        "can't take elf endianness byte".to_string(),
    ))?)?;
    if info.endianness == ELFEndianness::Big {
        return Err(Error::new(
            UnsupportedBinaryFormat,
            "big endianness not supported".to_string(),
        ));
    }
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
    Ok(info)
}

pub fn read_header(file: &mut std::fs::File) -> Result<ELFHeader, Error> {
    let mut header = ELFHeader::new();
    header.identity = read_identity(file)?;
    Ok(header)
}

pub fn read_elf_file(path: String) -> Result<ELFFile, Error> {
    let mut file = std::fs::File::open(path)
        .map_err(|err| Error::new(ErrorType::CantOpenFile, err.to_string()))?;
    let mut elf = ELFFile::new();
    elf.elf_header = read_header(&mut file)?;
    elf.elf_info = read_info(&mut file)?;
    elf.program_headers =
        read_program_headers(&mut file, elf.elf_info.e_phnum, elf.elf_info.e_phoff)?;
    elf.dynamic_headers = read_dynamic_headers(
        &mut file,
        elf.program_headers
            .iter()
            .find(|h| h.p_type == PType::PtDynamic)
            .ok_or(Error::new(
                CannotFindStrtab,
                "can't find dynamic program header".to_string(),
            ))?
            .p_offset,
    )?;
    elf.dependencies =
        read_dynamic_strtable(&mut file, &elf.program_headers, &elf.dynamic_headers)?;
    Ok(elf)
}
