use clap::{ArgGroup, Parser, Subcommand, CommandFactory};
use anyhow::Result;
use std::path::PathBuf;

mod config;
mod utils;
mod cmd;
mod db;

use cmd::cmd::{Arg, Commands};

fn main() -> Result<()> {
    let args = Arg::parse();
    args.check_arg()?;
    // custom checks for Args
    println!("{:#?}", args);
    Ok(())
}
