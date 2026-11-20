use crate::core::error::{Error, ErrorType};
use crate::core::info::{ELFInfo, byte_to_elfclass, byte_to_elfendian};
use std::io::Read;

fn read_sign(file: &mut std::fs::File, info: &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 4] = [0; 4];
    file.read(&mut header_buff)
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

fn read_class(file: &mut std::fs::File, info: &mut ELFInfo) -> Result<(), Error> {
    let mut class_buff: [u8; 1] = [0; 1];
    file.read(&mut class_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.class = byte_to_elfclass(class_buff[0])?;
    Ok(())
}

fn read_endianness(file: &mut std::fs::File, info: &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.endianness = byte_to_elfendian(header_buff[0])?;
    Ok(())
}

fn read_version(file: &mut std::fs::File, info: &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.version = header_buff[0];
    Ok(())
}

fn read_abi(file: &mut std::fs::File, info: &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.abi = header_buff[0];
    Ok(())
}

fn read_abi_version(file: &mut std::fs::File, info: &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 1] = [0; 1];
    file.read(&mut header_buff)
        .map_err(|err| Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.abi_version = header_buff[0];
    Ok(())
}

fn read_padding(file: &mut std::fs::File, info: &mut ELFInfo) -> Result<(), Error> {
    let mut header_buff: [u8; 7] = [0; 7];
    file.read(&mut header_buff).map_err(|err|Error::new(ErrorType::ReadFailed, err.to_string()))?;
    info.padding = header_buff;
    Ok(())
}

pub fn read_header(path: String) -> Result<ELFInfo, Error> {
    let mut file = std::fs::File::open(path)
        .map_err(|err| Error::new(ErrorType::CantOpenFile, err.to_string()))?;
    let mut info = ELFInfo::new();
    read_sign(&mut file, &mut info)?;
    read_class(&mut file, &mut info)?;
    read_endianness(&mut file, &mut info)?;
    read_version(&mut file, &mut info)?;
    read_abi(&mut file, &mut info)?;
    read_abi_version(&mut file, &mut info)?;
    read_padding(&mut file, &mut info)?;
    Ok(info)
}
