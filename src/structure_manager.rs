use rusqlite::Connection;

use crate::{argument_errors::RmtArgumentErrors, config::Config, config_manager, data_manager, secure_delete::delete_folder};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, MAIN_SEPARATOR},
};

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
        fs::create_dir(&trash_path).expect("Unable to create trash directory");
    }
}

// Delete trash directory, config file and database file if exists
pub fn clear_structure(is_test: bool) {
    let trash_path = get_trash_directory_path(is_test);
    println!("{}", trash_path);
    if Path::new(&trash_path).is_dir() {
        delete_folder(&trash_path)
            .unwrap_or_else(|_| panic!("Unable to delete {}", trash_path));
    }

    let data_base_path = get_data_base_path(is_test);
    if Path::new(&data_base_path).is_dir() {
        delete_folder(&data_base_path)
            .unwrap_or_else(|_| panic!("Unable to delete {}", data_base_path));
    }
}

// Create config file inside trash_directory if not exist
fn create_config_file(is_test: bool) -> Config {
    let config_path = get_config_path(is_test);
    config_manager::config_setup(&config_path)
}

pub fn get_home_directory_path() -> String {
    dirs::home_dir().unwrap().to_string_lossy().into_owned()
}

pub fn get_trash_directory_path(is_test: bool) -> String {
    format!(
        "{}{}{}",
        get_home_directory_path(),
        MAIN_SEPARATOR,
        get_trash_directory_name(is_test)
    )
}

pub fn get_config_path(is_test: bool) -> String {
    let config_name = if is_test { TEST_CONFIG } else { CONFIG };
    format!(
        "{}{}{}",
        get_trash_directory_path(is_test),
        MAIN_SEPARATOR,
        config_name
    )
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
    Connection::open(&data_base_path)
        .unwrap_or_else(|_| panic!("Unable to create {} file", &data_base_path))
}

pub fn get_data_base_path(is_test: bool) -> String {
    format!(
        "{}{}{}",
        get_trash_directory_path(is_test),
        MAIN_SEPARATOR,
        get_data_base_file_name(is_test)
    )
}

pub fn get_element_name(element_path_with_name: &str) -> String {
    let mut elements: Vec<&str> = element_path_with_name.split(MAIN_SEPARATOR).collect();
    elements.retain(|element| !element.is_empty());
    if elements.is_empty() {
        return "".to_string();
    }
    elements.last().unwrap().to_string()
}

pub fn get_element_path(element_path_with_name: &str) -> String {
    if element_path_with_name.matches(MAIN_SEPARATOR).count() == 1 {
        return MAIN_SEPARATOR.to_string();
    }

    let mut size = element_path_with_name.len();
    if !element_path_with_name.is_empty()
        && element_path_with_name.as_bytes()[element_path_with_name.len() - 1] as char
            == MAIN_SEPARATOR
    {
        size -= 1;
    }
    for i in (0..size).rev() {
        if element_path_with_name.as_bytes()[i] as char == MAIN_SEPARATOR {
            return element_path_with_name[0..i].to_string();
        }
    }
    element_path_with_name.to_string()
}

pub fn relative_path_to_absolute(relative_path: &str) -> Result<String, RmtArgumentErrors> {
    let original_path = relative_path;
    let path_result = shellexpand::full(relative_path)
        .ok()
        .and_then(|x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok())
        .and_then(|p| p.into_os_string().into_string().ok());

    if let Some(mut absolute_path) = path_result {
        if cfg!(target_os = "windows") {
            absolute_path = absolute_path.replace(r"\\?\", "");
        }

        if get_element_name(original_path) != get_element_name(&absolute_path) {
            Ok(format!(
                "{}{}{}",
                get_element_path(&absolute_path),
                MAIN_SEPARATOR,
                get_element_name(original_path)
            ))
        } else {
            Ok(absolute_path)
        }
    } else {
        Err(RmtArgumentErrors::InvalidPathWithoutForceFlags {
            element_name: original_path.to_string(),
        })
    }
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

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_get_element_name() {
        assert_eq!(get_element_name(""), "");
        assert_eq!(get_element_name("/"), "");
        assert_eq!(get_element_name("/home/test/oui.txt"), "oui.txt");
        assert_eq!(get_element_name("/home"), "home");
        assert_eq!(get_element_name("/home/test"), "test");
        assert_eq!(get_element_name("/home/test/"), "test");
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_get_element_path() {
        assert_eq!(get_element_path(""), "");
        assert_eq!(get_element_path("/home/test/oui.txt"), "/home/test");
        assert_eq!(get_element_path("/home"), "/");
        assert_eq!(get_element_path("/home/test"), "/home");
        assert_eq!(get_element_path("/home/test/"), "/home");
        assert_eq!(get_element_path("/home/test/oui/non"), "/home/test/oui");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_element_name() {
        assert_eq!(get_element_name(""), "");
        assert_eq!(get_element_name("\\"), "");
        assert_eq!(get_element_name("\\home\\test\\oui.txt"), "oui.txt");
        assert_eq!(get_element_name("\\home"), "home");
        assert_eq!(get_element_name("\\home\\test"), "test");
        assert_eq!(get_element_name("\\home\\test\\"), "test");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_get_element_path() {
        assert_eq!(get_element_path(""), "");
        assert_eq!(get_element_path("\\home\\test\\oui.txt"), "\\home\\test");
        assert_eq!(get_element_path("\\home"), "\\");
        assert_eq!(get_element_path("\\home\\test"), "\\home");
        assert_eq!(get_element_path("\\home\\test\\"), "\\home");
        assert_eq!(
            get_element_path("\\home\\test\\oui\\non"),
            "\\home\\test\\oui"
        );
    }

    #[test]
    fn aa() {
        clear_structure(true)
    }
}
