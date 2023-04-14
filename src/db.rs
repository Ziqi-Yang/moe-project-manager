use anyhow::Result;
use std::path::PathBuf;
use crate::config;

pub struct Entry((PathBuf, Option<String>, Option<String>, Option<String>, Vec<String>));

pub struct DataBase {
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

    pub fn add(&mut self, project_path: PathBuf, nick_name: Option<String>, language: Option<String>, category: Option<String>, tags: Vec<String>) {
        self.entries.push(Entry((project_path, nick_name, language, category, tags)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn db_initialize_test() {
        let _ = DataBase::new().unwrap();
    }

    #[test]
    fn db_add_test() {
        let mut db = DataBase::new().unwrap();
        db.add(PathBuf::from("/tmp"), None, None, None, vec!());
    }
}
