use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Easy-ELF binary inspector with AI.")]
pub struct Cli {
    #[arg(short = 'p', long = "path", help = "Path to ELF file to inspect")]
    pub path: Option<String>,

    #[arg(
        short = 'a',
        long,
        value_name = "Distro",
        help = "Distro for find packages with AI (required token)"
    )]
    pub ai: Option<String>,

    #[arg(
        short = 't',
        long,
        value_name = "Token",
        help = "Token for DeepSeekAPI"
    )]
    pub token: Option<String>,

    #[arg(long = "head", help = "Show ELF header")]
    pub header: bool,

    #[arg(long = "ph", help = "Show ELF program headers")]
    pub program_headers: bool,

    #[arg(long = "sh", help = "Show ELF dynamic headers")]
    pub dynamic_headers: bool,

    #[arg(short = 'd', long)]
    pub dependencies: bool,

    #[arg(short = 'e', long)]
    pub easy: bool,
}
