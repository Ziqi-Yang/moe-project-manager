use clap::Parser;
use anyhow::Result;

mod config;
mod utils;
mod cmd;
mod db;

use cmd::{Arg};

fn main() -> Result<()> {
    let cli = Arg::parse();
    // cli.check_arg()?;
    // cli.run()?;
    // let db = db::Database::get_or_create_db()?;
    // db.save_to_disk();
    Ok(())
}
