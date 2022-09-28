use crate::{config::Config, config_manager, trash_item::TrashItem};

use chrono;
use fs_extra::dir::get_size;
use sha256;
use std::{fs, path, process::Command};

fn get_element_path(element_name: &str) -> String {
    let current_directory_path = std::env::current_dir().unwrap();
    let current_directory_str = current_directory_path.to_str().unwrap();
    let selected_element_path = format!("{}/{}", &current_directory_str, element_name);
    selected_element_path
}

fn make_zip(element_src_path: &str, element_dst_path: &str, zip_name: &str) -> u64 {
    let mut path_elements: Vec<&str> = element_src_path.split("/").collect();
    if path_elements.len() >= 1 && path_elements[0].is_empty() {
        path_elements.remove(0);
    }

    if element_src_path.is_empty() {
        return 0;
    }

    let compression_name = format!("{}/{}.{}", &element_dst_path, &zip_name, "zip");

    let mut last_directory_before_target = String::new();
    for i in 0..path_elements.len() - 1 {
        last_directory_before_target =
            format!("{}/{}", last_directory_before_target, path_elements[i]);
    }

    Command::new("cd").arg(last_directory_before_target);
    Command::new("zip")
        .arg("-r")
        .arg(&compression_name)
        .arg(path_elements.last().unwrap());
    Command::new("rm")
        .arg("-rf")
        .arg(path_elements.last().unwrap());
    Command::new("cd").arg("-");

    get_size(&compression_name).expect(&format!(
        "Unable to get element size of {}",
        compression_name
    ))
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

    let mut compression_size: Option<u64> = None;

    if config.compression {
        compression_size = Some(make_zip(&element_path, &destination_path, &hash));
    }

    TrashItem::new(
        element_name.to_string(),
        hash,
        element_path,
        date.to_string(),
        element_size,
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
