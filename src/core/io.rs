use crate::core::error::{Error, ErrorType};

pub fn get_params_without_first() -> Result<Vec<String>, Error> {
    std::env::args_os()
        .skip(1)
        .into_iter()
        .map(|arg| {
            arg.into_string().map_err(|e| {
                Error::new(
                    ErrorType::ArgumentsError,
                    e.to_str().unwrap_or("Unknown error").to_string(),
                )
            })
        })
        .collect()
}

pub fn print_error(error: &Error) {
    println!("{}", term_ansi::red!("{}", error.to_string()));
}
