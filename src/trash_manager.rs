use crate::{
    config::Config, data_manager, structure_manager::get_trash_directory_path,
    trash_item::TrashItem,
};

use chrono;
use fs_extra::dir::get_size;
use rusqlite::Connection;
use sha256;
use std::fs;
use std::{ffi::OsStr, path::Path};

pub fn abspath(p: &str) -> Option<String> {
    shellexpand::full(p)
        .ok()
        .and_then(|x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok())
        .and_then(|p| p.into_os_string().into_string().ok())
}


pub fn add_element_to_trash(
    connection: &Connection,
    config: &Config,
    element_name: &str,
    is_test: bool,
) {
    let element_path = abspath(&element_name).unwrap();
    let element_size = get_size(&element_path).expect("Unable to get element size");

    let hash = sha256::digest(format!(
        "{}{}{}{}",
        element_name,
        &element_path,
        element_size,
        chrono::offset::Local::now().timestamp_nanos()
    ));

    let date = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");

    let compression_size: Option<u64> = None;

    if config.compression {
        // TODO
    } else {
        let new_name = format!("{}/{}", get_trash_directory_path(is_test), hash);
        fs::rename(&element_path, &new_name).unwrap();
    }

    let trash_item = TrashItem::new(
        element_name.to_string(),
        hash,
        element_path,
        date.to_string(),
        element_size,
        compression_size,
    );

    data_manager::insert_trash_item(connection, &trash_item, is_test);
}

pub fn add_all_elements_to_trash(
    connection: &Connection,
    config: &Config,
    element_name: &[String],
    is_test: bool,
) {
    for path in element_name {
        add_element_to_trash(connection, config, path, is_test);
    }
}

#[cfg(test)]
mod tests {}
