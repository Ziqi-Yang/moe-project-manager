use rusqlite::{Connection, Result};

fn init() -> Result<()> {
    // let conn = Connection::open_in_memory()?;
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;
}
