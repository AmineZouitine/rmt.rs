use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::config::Config;

// Create config file with default config if not exist and return it, or read current config file and the config
pub fn config_setup(config_path: &str) -> Config {
    let mut current_config = Config::new_default_config();
    let default_config_content =
        serde_yaml::to_string(&current_config).expect("Unable to deserialize default config file");

    // if the config files doesn't exist
    if !Path::new(config_path).is_file() {
        write_default_config_file(&config_path, &default_config_content);
    } else {
        // get the current content of the config file
        let config_content = fs::read_to_string(&config_path).expect("Unable to read config file");

        // try to convert it to Config structure
        current_config = serde_yaml::from_str(&config_content).unwrap_or_else(|_| {
            // if the current config file, is not correct (bad format, bad values etc...), replace the content by the default config
            write_default_config_file(&config_path, &default_config_content);
            current_config
        });
    }

    current_config
}

fn write_default_config_file(config_file_path: &str, default_config_str: &str) {
    File::create(config_file_path)
        .expect("Unable to create config file")
        .write_all(default_config_str.as_bytes())
        .expect("Unable to write default config in config file");
}
