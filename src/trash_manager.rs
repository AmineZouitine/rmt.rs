use crate::{config::Config, config_manager, trash_item::TrashItem};

use chrono;
use fs_extra::dir::get_size;
use sha256;
use std::process::Command;

fn get_element_path(element_name: &str) -> String {
    let current_directory_path = std::env::current_dir().unwrap();
    let current_directory_str = current_directory_path.to_str().unwrap();
    let selected_element_path = format!("{}/{}", &current_directory_str, element_name);
    selected_element_path
}

pub fn convert_element_to_trash_item(config: &Config, element_name: &str) -> TrashItem {
    let element_path = get_element_path(element_name);
    let element_size = get_size(&element_path).expect("Unable to get element size");

    let hash = sha256::digest(format!(
        "{}{}{}{}",
        element_name,
        &element_path,
        element_size,
        chrono::offset::Local::now().timestamp_nanos()
    ));

    let date = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");

    let destination_path = config_manager::get_trash_directory_path();

    let mut compression_method: Option<String> = None;
    let mut compression_size: Option<u64> = None;

    if let Some(compression) = &config.compression {
        let compress_name = format!("{}/{}.{}", &destination_path, &hash, "zip");
        Command::new(&compression.method)
            .arg(&compress_name)
            .arg(&element_path);
        Command::new("rm").arg("-rf").arg(&element_path);

        compression_method = Some(compression.method.clone());
        compression_size = Some(get_size(&compress_name).expect("Unable to get element size"));
    } else {
        Command::new("mv")
            .arg(&element_path)
            .arg(format!("{}/{}", &destination_path, &hash));
    }

    TrashItem::new(
        element_name.to_string(),
        hash,
        element_path,
        date.to_string(),
        element_size,
        compression_method,
        compression_size,
    )
}

// pub fn move_element(config: &Config, element: &str, ) -> Result<>
// {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_element_to_trash_item() {}
}
