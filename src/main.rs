use clap::Parser;
use anyhow::Result;

mod config;
mod utils;
mod cmd;
mod db;

use cmd::{Arg};

fn main() -> Result<()> {
    let _cli = Arg::parse();
    // cli.check_arg()?;
    // cli.run()?;
    Ok(())
}
