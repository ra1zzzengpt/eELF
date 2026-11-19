pub mod core;

use crate::core::elf_reader::read_header;
use crate::core::io::{get_params_without_first, print_error};
fn main() {
    let args = get_params_without_first().unwrap();
    match read_header(args[0].clone()) {
        Ok(header) => {
            println!("{:?}", header.ident)
        }
        Err(e) => {
            print_error(&e);
        }
    };
}
