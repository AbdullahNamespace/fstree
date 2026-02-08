use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tree")]
#[command(about = "A blazing fast and beautiful tree command in Rust", long_about = None)]
pub struct TreeConfig {
    #[arg(short = 'p', long = "path", value_name = "PATH", default_value = ".")]
    pub path: PathBuf,

    #[arg(short = 'd', long = "depth")]
    pub depth: Option<usize>,

    #[arg(short = 'a', long = "all")]
    pub all: bool,
}
