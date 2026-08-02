use clap::Parser;

use myagentcontrol::cli;
use myagentcontrol::cli::args::Cli;

fn main() -> myagentcontrol::core::errors::Result<()> {
    let cli = Cli::parse();
    cli::run(cli)
}
