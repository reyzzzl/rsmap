use clap::Parser;
use std::path::PathBuf;

// argument cli for network scanner
#[derive(Parser, Debug, Clone)]
#[command(author, version, about = "rsmap - High-performance network scanner")]
pub struct Cli {
    #[arg(short, long, value_delimiter = ',', required = false)]
    pub targets: Vec<String>,

    #[arg(short, long, default_value = "1-1000")]
    pub ports: String,

// technique scanning used 
    #[arg(short = 'T', long, default_value = "syn")]
    pub technique: String,

    #[arg(short = 'o', long, default_value = "json")]
    pub output: String,

    #[arg(long)]
    pub output_file: Option<PathBuf>,

// limit speed packet
    #[arg(long, default_value = "10000")]
    pub rate_limit: u32,

    #[arg(long, default_value = "2000")]
    pub timeout_ms: u64,

    #[arg(long, default_value = "1")]
    pub retries: u8,

// randomize  port order 
    #[arg(long)]
    pub randomize: bool,

    #[arg(long)]
    pub service: bool,

    #[arg(long)]
    pub fingerprint: bool,

    #[arg(long)]
    pub resume: Option<PathBuf>,

    #[arg(long)]
    pub save_state: Option<PathBuf>,

    #[arg(long)]
    pub ipv6: bool,

    #[arg(long)]
    pub demo: bool,
}

pub fn parse() -> Cli {
// TODO: add validation costum next mybe for parsing cidr or format port
    Cli::parse()
}