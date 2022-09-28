use std::{path::Path, fs};

const TRASH_DIRECTORY_NAME: &str = ".trash_rmt";
const TEST_TRASH_DIRECTORY_NAME: &str = ".test_trash_rmt";

const CONFIG: &str = "config_rmt";
const TEST_CONFIG: &str = "test_config_rmt";


// Setup tash directory and config file inside it
pub fn setup_structure(is_test: bool)
{
    create_trash_directory(is_test);
    create_config_file(is_test);
}

// Create trash directory at the home if not exist
fn create_trash_directory(is_test: bool){
    let trash_path = get_trash_directory_path(is_test);
    if !Path::new(&trash_path).is_dir() {
        fs::create_dir(&trash_path).expect("Unable to create tash directory");
    }
}


// Create config file inside trash_directory if not exist
fn create_config_file(is_test: bool)
{

    let config_path = get_config_path(is_test);
    if !Path::new(&config_path).is_file() {
        fs::File::create(config_path);
    }
}

fn get_home_directory_path() -> String {
    home::home_dir()
        .expect("Unable to find home directory path")
        .to_str()
        .expect("Unable to convert home dir to str")
        .to_string()
}


pub fn get_trash_directory_path(is_test: bool) -> String {
    let trash_directory_name = if is_test {TEST_TRASH_DIRECTORY_NAME} else {TRASH_DIRECTORY_NAME};
    format!("{}/{}", get_home_directory_path(), trash_directory_name)
}


pub fn get_config_path(is_test: bool) -> String {
    let config_name = if is_test {TEST_CONFIG} else {CONFIG};
    format!("{}/{}", get_trash_directory_path(is_test), config_name)
}
