use anyhow::Result;
use std::path::PathBuf;
use crate::config;

pub struct Entry((PathBuf, Option<String>, Option<String>, Option<String>, Vec<String>));

pub struct DataBase{
    path: PathBuf,
    entries: Vec<Entry>
}

impl DataBase{
    pub fn new() -> Result<Self> {
        let path = config::get_data_dir()?.join(config::DB_NAME);
        Ok(DataBase{
            path,
            entries: Vec::new()
        })
    }

    pub fn add(&mut self, projectPath: PathBuf, nickName: Option<String>, language: Option<String>, category: Option<String>, tags: Vec<String>) {
        self.entries.push(Entry((projectPath, nickName, language, category, tags)));
    }
}

// #[cfg(test)]
// pub mod test {
// }
