use clap::Parser;
use rcli::{
    opts::{Cli, SubCommand},
    process,
};

fn main() -> anyhow::Result<()> {
    match Cli::parse().cmd {
        SubCommand::Csv(opts) => {
            process::process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
