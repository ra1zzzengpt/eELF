use crate::core::error::{Error, ErrorType};
use crate::core::info::ELFInfo;
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

pub fn read_header(path: String) -> Result<ELFInfo, Error> {
    let mut file = std::fs::File::open(path)
        .map_err(|err| Error::new(ErrorType::CantOpenFile, err.to_string()))?;
    let mut info = ELFInfo::new();
    read_sign(&mut file, &mut info)?;
    Ok(info)
}
