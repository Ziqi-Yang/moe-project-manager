use std::fs;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry((PathBuf, Option<String>, Option<String>, Option<String>, Vec<String>));

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    path: PathBuf,
    entries: Vec<Entry>
}

impl Database{
    fn new() -> Result<Self> {
        let path = config::get_data_dir()?.join(config::DB_NAME);
        Ok(Database {
            path,
            entries: Vec::new()
        })
    }

    pub fn get_or_create_db() -> Result<Self> {
        let path = config::get_data_dir()?.join(config::DB_NAME);
        match fs::read(path) {
            Ok(content) => {
                let decoded: Database = bincode::deserialize(&content).unwrap();
                Ok(decoded)
            },
            Err(e) if e.kind() == ErrorKind::NotFound => {
                Database::new()
            },
            Err(e) => Err(e.into())
        }
    }

    pub fn add(&mut self, project_path: PathBuf, nick_name: Option<String>, language: Option<String>, category: Option<String>, tags: Vec<String>) {
        self.entries.push(Entry((project_path, nick_name, language, category, tags)))
    }

    pub fn save_to_disk(&self) -> Result<()> {
        let encoded: Vec<u8> = bincode::serialize(self).unwrap();
        let mut file = fs::File::create(self.path.to_str().unwrap())?;
        file.write_all(&encoded)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn db_initialize_test() {
        let _ = Database::new().unwrap();
    }

    #[test]
    fn db_add_test() {
        let mut db = Database::new().unwrap();
        db.add(PathBuf::from("/tmp"), None, None, None, vec!());
    }
}
