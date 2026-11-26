use crate::core::error::Error;
use crate::core::error::ErrorType::StorageError;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn token_path() -> Result<PathBuf, Error> {
    let path = PathBuf::from(std::env::var_os("HOME").unwrap()).join(".config/eelf/eelf-token.txt");
    Ok(path)
}

pub fn save_token(path: PathBuf, token: String) -> Result<(), Error> {
    std::fs::create_dir_all(path.parent().expect("token path has a parent"))
        .map_err(|e| Error::new(StorageError, e.to_string()))?;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    let mut file = options
        .open(path)
        .map_err(|e| Error::new(StorageError, e.to_string()))?;
    file.write_all(token.as_bytes())
        .map_err(|e| Error::new(StorageError, e.to_string()))?;
    Ok(())
}

pub fn load_token(path: PathBuf) -> Result<String, Error> {
    let mut file =
        std::fs::File::open(path).map_err(|e| Error::new(StorageError, e.to_string()))?;
    let mut token = String::new();
    file.read_to_string(&mut token)
        .map_err(|e| Error::new(StorageError, e.to_string()))?;
    Ok(token)
}
