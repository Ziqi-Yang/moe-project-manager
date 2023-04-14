use clap::Parser;
use anyhow::Result;

mod config;
mod utils;
mod cmd;
mod db;

use cmd::{Arg, Run};

fn main() -> Result<()> {
    let cli = Arg::parse();
    cli.check_arg()?;
    cli.run()?;
    Ok(())
}
