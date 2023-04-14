use crate::cmd::Add;
use crate::cmd::Run;
use anyhow::{ Result};

impl Run for Add {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
