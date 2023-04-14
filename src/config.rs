use std::env;
use std::fs;
use std::ffi::OsString;
use anyhow::Result;
use directories::{BaseDirs, ProjectDirs };
use std::path::PathBuf;

pub const DB_NAME: &str = "p.db3";

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "MeowKing", "mpm").expect("no valid home directory path could be retrieved from the operating system")
}

pub fn home_dir() -> Result<PathBuf> {
    let basedirs = BaseDirs::new().expect("no valid home directory path could be retrieved from the operating system");
    let home_dir = basedirs.home_dir();
    fs::create_dir_all(home_dir)?;
    Ok(PathBuf::from(home_dir))
}

pub fn get_data_dir() -> Result<PathBuf> {
    let proj_dirs = project_dirs();
    let path = match env::var_os("_MPM_DATA_DIR") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(proj_dirs.cache_dir()),
    };
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn get_config_dir() -> Result<PathBuf> {
    let proj_dirs = project_dirs();
    let path = match env::var_os("_MPM_CONFIG_DIR") {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(proj_dirs.config_dir()),
    };
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn get_fzf_opts() -> Option<OsString> {
    env::var_os("_ZO_FZF_OPTS")
}

#[cfg(test)]
mod test {
    use super::*; 

    #[test]
    fn test_get_data_dir_linux() {
        let actual_path = get_data_dir().unwrap(); 
        let mut expected_path = home_dir().unwrap();
        // TODO change expected_dir for mac and windows
        expected_path.push(".cache/mpm");
        assert_eq!(actual_path, expected_path);
    }

    #[test]
    fn test_get_config_dir_linux() {
        let actual_path = get_config_dir().unwrap();
        let mut expected_path = home_dir().unwrap();
        // TODO change expected_dir for mac and windows
        expected_path.push(".config/mpm");
        assert_eq!(actual_path, expected_path);
    }
}
