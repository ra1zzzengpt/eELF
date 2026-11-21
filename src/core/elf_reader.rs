use crate::core::error::{Error, ErrorType};
use crate::core::info::{byte_to_elfclass, byte_to_elfendian, ELFHeader, ELFIdentity, ELFInfo, bytes_to_elftype, bytes_to_elfmachine};
use std::io::Read;

fn read_sign(file: &mut std::fs::File, info: &mut ELFIdentity) -> Result<(), Error> {
    let mut header_buff: [u8; 4] = [0; 4];
    file.read_exact(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    if (header_buff[0] == 0x7F)
        && (header_buff[1] == 0x45)
        && (header_buff[2] == 0x4C)
        && (header_buff[3] == 0x46)
    {
        info.ident = true;
        return Ok(());
    }
    Err(Error::new(
        ErrorType::NotELFFile,
        "file is not elf.".to_string(),
    ))
}

fn read_class(file: &mut std::fs::File, info: &mut ELFIdentity) -> Result<(), Error> {
    let mut class_buff: [u8; 1] = [0; 1];
    file.read_exact(&mut class_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.class = byte_to_elfclass(class_buff[0])?;
    Ok(())
}

fn read_endianness(file: &mut std::fs::File, info: &mut ELFIdentity) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read_exact(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.endianness = byte_to_elfendian(header_buff[0])?;
    Ok(())
}

fn read_version(file: &mut std::fs::File, info: &mut ELFIdentity) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read_exact(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.version = header_buff[0];
    Ok(())
}

fn read_abi(file: &mut std::fs::File, info: &mut ELFIdentity) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read_exact(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.abi = header_buff[0];
    Ok(())
}

fn read_abi_version(file: &mut std::fs::File, info: &mut ELFIdentity) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read_exact(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.abi_version = header_buff[0];
    Ok(())
}

fn read_padding(file: &mut std::fs::File, info: &mut ELFIdentity) -> Result<(), Error> {
    let mut header_buff: [u8; 7] = [0; 7];
    file.read(&mut header_buff).map_err(|err|Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.padding = header_buff;
    Ok(())
}

fn read_identity(file: &mut std::fs::File) -> Result<ELFIdentity, Error> {
    let mut info = ELFIdentity::new();
    read_sign(file,&mut info)?;
    read_class(file,&mut info)?;
    read_endianness(file,&mut info)?;
    read_version(file,&mut info)?;
    read_abi(file,&mut info)?;
    read_abi_version(file,&mut info)?;
    read_padding(file,&mut info)?;
    Ok(info)
}

fn read_type(file: &mut std::fs::File, info : &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 2] = [0; 2];
    file.read_exact(&mut header_buff).map_err(|err|{Error::new(ErrorType::ReadFailed, err.to_string())})?;
    info.e_type = bytes_to_elftype(header_buff)?;
    Ok(())
}

fn read_machine(file: &mut std::fs::File, info : &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 2] = [0; 2];
    file.read_exact(&mut header_buff).map_err(|err|{Error::new(ErrorType::ReadFailed, err.to_string())})?;
    info.e_machine = bytes_to_elfmachine(header_buff)?;
    Ok(())
}

fn read_info(file: &mut std::fs::File) -> Result<ELFInfo, Error> {
    let mut info = ELFInfo::new();
    read_type(file,&mut info)?;
    read_machine(file,&mut info)?;
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
