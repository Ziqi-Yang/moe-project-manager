use std::env;
use std::ffi::OsString;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

pub fn get_data_dir() -> Result<PathBuf> {
    let proj_dir = ProjectDirs::from("com", "MeowKing", "mpm").expect("no valid home directory path could be retrieved from the operating system");
    let path = match env::var_os("_MPM_DATA_DIR") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(proj_dir.cache_dir()),
    };
    Ok(path)
}

pub fn get_fzf_opts() -> Option<OsString> {
    env::var_os("_ZO_FZF_OPTS")
}

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn test_get_data_dir() {
        let expected_path = PathBuf::from("/home/zarkli/.cache/mpm"); // TODO don't include username
        let actual_path = get_data_dir().unwrap(); 
        assert_eq!(actual_path, expected_path);
    }
}
