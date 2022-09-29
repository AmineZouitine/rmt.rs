use rusqlite::Connection;

use crate::{config::Config, config_manager, data_manager};
use std::{fs, path::Path};

// TRASH DIRECTORY CONSTANT
const TRASH_DIRECTORY_NAME: &str = ".trash_rmt";
const TEST_TRASH_DIRECTORY_NAME: &str = ".test_trash_rmt";

// CONFIG CONSTANTE
const CONFIG: &str = "config_rmt.yml";
const TEST_CONFIG: &str = "test_config_rmt.yml";

//DATABASE FILE CONSTANTE
const DATA_BASE_FILE_NAME: &str = "trash.db";
const TEST_DATA_BASE_FILE_NAME: &str = "test_trash.db";

//DATABASE TABLE CONSTANTE
const DATA_BASE_TABLE_NAME: &str = "trash_table";
const TEST_DATA_BASE_TABLE_NAME: &str = "test_trash_table";

// Setup tash directory and config file inside it and return the current config
pub fn setup_structure(is_test: bool) -> (Config, Connection) {
    create_trash_directory(is_test);
    (
        create_config_file(is_test),
        data_manager::setup_data_base(is_test),
    )
}

// Create trash directory at the home if not exist
fn create_trash_directory(is_test: bool) {
    let trash_path = get_trash_directory_path(is_test);
    if !Path::new(&trash_path).is_dir() {
        fs::create_dir(&trash_path).expect("Unable to create tash directory");
    }
}

// Delete trash directory, config file and database file if exists
pub fn clear_structure(is_test: bool) {
    let trash_path = get_trash_directory_path(is_test);
    if Path::new(&trash_path).is_dir() {
        fs::remove_dir_all(&trash_path).expect(&format!("Unable to delete {}", trash_path));
    }

    let data_base_path = get_data_base_path(is_test);
    if Path::new(&data_base_path).is_dir() {
        fs::remove_dir_all(&data_base_path).expect(&format!("Unable to delete {}", data_base_path));
    }
}

// Create config file inside trash_directory if not exist
fn create_config_file(is_test: bool) -> Config {
    let config_path = get_config_path(is_test);
    config_manager::config_setup(&config_path)
}

pub fn get_home_directory_path() -> String {
    home::home_dir()
        .expect("Unable to find home directory path")
        .to_str()
        .expect("Unable to convert home dir to str")
        .to_string()
}

pub fn get_trash_directory_path(is_test: bool) -> String {
    format!(
        "{}/{}",
        get_home_directory_path(),
        get_trash_directory_name(is_test)
    )
}

pub fn get_config_path(is_test: bool) -> String {
    let config_name = if is_test { TEST_CONFIG } else { CONFIG };
    format!("{}/{}", get_trash_directory_path(is_test), config_name)
}

fn get_trash_directory_name(is_test: bool) -> String {
    if is_test {
        TEST_TRASH_DIRECTORY_NAME.to_string()
    } else {
        TRASH_DIRECTORY_NAME.to_string()
    }
}

pub fn get_data_base_table_name(is_test: bool) -> String {
    if is_test {
        TEST_DATA_BASE_TABLE_NAME.to_string()
    } else {
        DATA_BASE_TABLE_NAME.to_string()
    }
}

fn get_data_base_file_name(is_test: bool) -> String {
    if is_test {
        TEST_DATA_BASE_FILE_NAME.to_string()
    } else {
        DATA_BASE_FILE_NAME.to_string()
    }
}

pub fn create_data_base_file(is_test: bool) -> Connection {
    let data_base_path = get_data_base_path(is_test);
    Connection::open(&data_base_path).expect(&format!("Unable to create {} file", &data_base_path))
}

pub fn get_data_base_path(is_test: bool) -> String {
    format!(
        "{}/{}",
        get_trash_directory_path(is_test),
        get_data_base_file_name(is_test)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_trash_directory() {
        let is_test = false;
        create_trash_directory(is_test);
        let path = get_trash_directory_path(is_test);
        assert!(fs::metadata(path).unwrap().is_dir());
        clear_structure(is_test);
    }

    #[test]
    fn test_create_trash_directory_test() {
        let is_test = true;
        create_trash_directory(is_test);
        let path = get_trash_directory_path(is_test);
        assert!(fs::metadata(path).unwrap().is_dir());
        clear_structure(is_test);
    }

    #[test]
    fn test_setup() {
        let is_test = false;
        setup_structure(is_test);
        let path_trash_folder = get_trash_directory_path(is_test);
        let path_config = get_config_path(is_test);
        let path_data_base = get_data_base_path(is_test);

        assert!(fs::metadata(&path_trash_folder).unwrap().is_dir());
        assert!(fs::metadata(&path_config).unwrap().is_file());
        assert!(fs::metadata(&path_data_base).unwrap().is_file());
        clear_structure(is_test);
    }
}
