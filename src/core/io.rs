use crate::core::error::{Error, ErrorType};

pub fn get_params_without_first() -> Result<Vec<String>, Error> {
    let params: Vec<String> = std::env::args_os()
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
        .collect::<Result<Vec<String>, Error>>()?;
    if params.is_empty() {
        return Err(Error::new(
            ErrorType::ArgumentsError,
            "Empty arguments".to_string(),
        ));
    }
    Ok(params)
}

pub fn print_error(error: &Error) {
    println!("{}", term_ansi::red!("{}", error.to_string()));
}
