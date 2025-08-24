use std::path::Path;

use clap::{Args, Subcommand, command};

#[derive(clap::Parser, Debug)]
#[command(name = "rcli", version = "1.0", author = "codecow")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[command(
        name = "csv",
        version = "1.0",
        about = "Show csv,or convert csv to other formats!"
    )]
    Csv(CsvOpts),
}

#[derive(Args, Debug)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if Path::new(file).exists() {
        Ok(file.to_string())
    } else {
        Err(format!("The file '{}' does not exist.", file))
    }
}
