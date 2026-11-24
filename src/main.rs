pub mod core;

use crate::core::ai::deepseek::DeepSeek;
use self::core::elf::elf_reader::read_elf_file;
use crate::core::io::{get_params_without_first, print_error};
fn main() {
    let args = match get_params_without_first() {
        Ok(args) => args,
        Err(error) => {
            print_error(&error);
            return;
        }
    };
    match read_elf_file(args[0].clone()) {
        Ok(elf) => {
            println!("{}", elf);
            let deepseek = DeepSeek::new(API_KEY_WIP,"https://api.deepseek.com/responses".to_string());
            match deepseek.analyze(elf.dependencies.clone(),"debian".to_string()) {
                Ok(answer) => {println!("{}", answer);}
                Err(error) => {print_error(&error)}
            }
        }
        Err(e) => {
            print_error(&e);
        }
    };
}
