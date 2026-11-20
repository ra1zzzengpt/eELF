pub mod core;

use crate::core::elf_reader::read_header;
use crate::core::io::{get_params_without_first, print_error};
fn main() {
    let args = match get_params_without_first() {
        Ok(args) => args,
        Err(error) => {
            print_error(&error);
            return;
        }
    };
    match read_header(args[0].clone()) {
        Ok(header) => {
            println!("{}", header)
        }
        Err(e) => {
            print_error(&e);
        }
    };
}
