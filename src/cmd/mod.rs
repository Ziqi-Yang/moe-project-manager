mod cmd;
mod add;

use anyhow::Result;
pub use cmd::*;

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for Arg {
    fn run(&self) -> Result<()> {
        match &self.command {
            Some(Commands::Add(cmd)) => cmd.run(),
            // Some(Commands::Scan(cmd)) => cmd.run(),
            // Some(Commands::Cd(cmd)) => cmd.run(),
            // Some(Commands::Init(cmd)) => cmd.run(),
            // Some(Commands::Cp(cmd)) => cmd.run(),
            // Some(Commands::List(cmd)) => cmd.run(),
            // Some(Commands::Jump(cmd)) => cmd.run(),
            // Some(Commands::Edit(cmd)) => cmd.run(),
            // None => Ok(()),
            _ => Ok(())
        }
    }
}
