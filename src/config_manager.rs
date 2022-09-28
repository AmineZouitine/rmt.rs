use home;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::config::Config;



pub fn config_setup() -> Config {
    let trash_path = create_trash_directory();

    let config_file_path = format!("{}/{}", trash_path, "config.yml");
    let mut current_config = Config::new_default_config();
    let default_config_str =
        serde_yaml::to_string(&current_config).expect("Unable to deserialize default config file");

    if !Path::new(&config_file_path).is_file() {
        write_default_config_file(&config_file_path, &default_config_str);
    } else {
        let config_content =
            fs::read_to_string(&config_file_path).expect("Unable to read config file");
        current_config = serde_yaml::from_str(&config_content).unwrap_or_else(|_| {
            write_default_config_file(&config_file_path, &default_config_str);
            current_config
        });
    }

    current_config
}



// Create the configuration file and write the basic configuration inside
fn write_default_config_file(config_file_path: &str, default_config_str: &str) {
    File::create(config_file_path)
        .expect("Unable to create config file")
        .write_all(default_config_str.as_bytes())
        .expect("Unable to write default config in config file");
}
