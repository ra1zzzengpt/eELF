use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ErrorType {
    CantOpenFile,
    ArgumentsError,
    ReadFailed,
    NotELFFile,
    UnknownELFClass,
    UnknownELFEndianness,
    UnknownELFType,
    UnsupportedBinaryFormat,
    CannotFindStrtab,
    NetworkError,
    StorageError,
}

#[derive(Debug)]
pub struct Error {
    error_type: ErrorType,
    message: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Error {
            error_type,
            message,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}
