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
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ErrorType::CantOpenFile => {
                write!(f, "CantOpenFile")
            }
            ErrorType::ArgumentsError => {
                write!(f, "ArgumentsError")
            }
            ErrorType::ReadFailed => {
                write!(f, "ReadFailed")
            }
            ErrorType::NotELFFile => {
                write!(f, "NotELFFile")
            }
            ErrorType::UnknownELFClass => {
                write!(f, "UnknownELFClass")
            }
            ErrorType::UnknownELFEndianness => {
                write!(f, "UnknownELFEndianness")
            }
            ErrorType::UnknownELFType => {
                write!(f, "UnknownELFType")
            }
        }
    }
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
        write!(f, "{}: {}", self.error_type, self.message)
    }
}
