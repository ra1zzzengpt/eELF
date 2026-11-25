use crate::core::error::Error;
use crate::core::error::ErrorType::StorageError;
use std::io::{Read, Write};

pub fn save_token(path: String, token: String) -> Result<(), Error> {
    let mut file =
        std::fs::File::create(path).map_err(|e| Error::new(StorageError, e.to_string()))?;
    file.write_all(token.as_bytes())
        .map_err(|e| Error::new(StorageError, e.to_string()))?;
    Ok(())
}

pub fn load_token(path: String) -> Result<String, Error> {
    let mut file =
        std::fs::File::open(path).map_err(|e| Error::new(StorageError, e.to_string()))?;
    let mut token = String::new();
    file.read_to_string(&mut token)
        .map_err(|e| Error::new(StorageError, e.to_string()))?;
    Ok(token)
}
