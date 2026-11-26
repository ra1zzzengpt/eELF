pub mod core;

use self::core::elf::elf_reader::read_elf_file;
use crate::core::ai::deepseek::DeepSeek;
use crate::core::cli::Cli;
use crate::core::error::Error;
use crate::core::storage::{load_token, save_token, token_path};
use clap::Parser;

fn print_error(error: &Error) {
    println!("{}", term_ansi::red!("{}", error.to_string()));
}
fn main() {
    let cli = Cli::parse();
    if let Some(token) = cli.token {
        match token_path().and_then(|path| save_token(path, token)) {
            Ok(_) => {}
            Err(e) => print_error(&e),
        };
    }
    if let Some(path) = cli.path {
        match read_elf_file(path) {
            Ok(elf) => {
                elf.display(
                    cli.header,
                    cli.program_headers,
                    cli.dynamic_headers,
                    cli.dependencies,
                );
                if let Some(distro) = cli.ai {
                    match token_path().and_then(load_token) {
                        Ok(token) => {
                            let deepseek = DeepSeek::new(
                                token,
                                "https://api.deepseek.com/responses".to_string(),
                            );
                            println!("DeepSeek thinking...");
                            match deepseek.analyze(&elf.dependencies, &distro) {
                                Ok(answer) => {
                                    println!(
                                        "Install for {distro}: {}",
                                        term_ansi::yellow!("{}", answer)
                                    );
                                }
                                Err(e) => print_error(&e),
                            }
                        }
                        Err(e) => print_error(&e),
                    }
                }
            }
            Err(e) => {
                print_error(&e);
            }
        };
    }
}
