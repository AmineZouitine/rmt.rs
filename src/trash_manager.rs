use crate::structure_manager;
use crate::{
    config::Config, data_manager, structure_manager::get_trash_directory_path,
    trash_item::TrashItem,
};

use chrono;
use colored::Colorize;
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
        structure_manager::get_element_name(element_name),
        hash,
        structure_manager::get_element_path(&element_path),
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

pub fn remove_all_elements(connection: &Connection, is_test: bool, trash_items_ids: &Vec<i8>) {
    trash_items_ids.iter().for_each(|trash_item_id| {
        let trash_item = data_manager::find_trash_item_by_id(connection, is_test, *trash_item_id);
        println!("trash item = {}\r", trash_item);
        remove_element(&trash_item, is_test);
        data_manager::delete_trash_item_by_id(connection, is_test, *trash_item_id);
    });
}

fn remove_element(trash_item: &TrashItem, is_test: bool) {
    let element_path = format!(
        "{}/{}",
        structure_manager::get_trash_directory_path(is_test),
       trash_item.hash 
    );
    println!("element path = {}\r", element_path);
    if Path::new(&element_path).is_dir() {
        std::fs::remove_dir_all(&element_path).unwrap();
    }
    else {
        std::fs::remove_file(&element_path).unwrap();
    }

    println!("{} {}\r", trash_item.name.red().bold(), "deleted !".red().bold());
}

pub fn restore_all_elements(connection: &Connection, is_test: bool, trash_items_ids: &Vec<i8>) {
    trash_items_ids.iter().for_each(|trash_item_id| {
        let trash_item = data_manager::find_trash_item_by_id(connection, is_test, *trash_item_id);
        restore_element(&trash_item, is_test);
        data_manager::delete_trash_item_by_id(connection, is_test, *trash_item_id);
    });
}

fn restore_element(trash_item: &TrashItem, is_test: bool) {
    let path_in_trash = format!(
        "{}/{}",
        structure_manager::get_trash_directory_path(is_test),
        trash_item.hash
    );
    if Path::new(&trash_item.path).is_dir() {
        let element_path_name = format!("{}/{}", &trash_item.path, &trash_item.name);
        let element_path_restored = format!("{}/{}", &trash_item.path, "restored_item");
        if !Path::new(&element_path_name).exists() {
            println!("{} has been restored ! :D", trash_item.name.green().bold());
            println!(
                "You can find it at this path: {}",
                element_path_name.green().bold()
            );
            fs::rename(&path_in_trash, &element_path_name).unwrap();
        } else if !Path::new(&element_path_restored).exists() {
            println!("{} has been restored ! :D", trash_item.name.green().bold());
            println!(
                "You can find it at this path: {}",
                element_path_restored.green().bold()
            );
            fs::rename(&path_in_trash, &element_path_restored).unwrap();
        }
        return;
    }
    println!("Unfortunately Path {} doesn't exist anymore, so we can't restore your element to the original path :c\n{}",
     &trash_item.path.green().bold(), "Please enter a new absolute path to restore your element".bold());

    let mut new_path = String::new();
    std::io::stdin().read_line(&mut new_path).unwrap();
    while !Path::new(&new_path).is_dir()
        || Path::new(&format!("{}/{}", &new_path, &trash_item.name)).exists()
    {
        if !Path::new(&new_path).exists() {
            println!(
                "{} doesn't exist ! You have to give a valid {} path of a {}",
                new_path.green().bold(),
                "absolute path".green().bold(),
                "directory".green().bold()
            );
        } else if !Path::new(&new_path).is_dir() {
            println!(
                "{} exist but it's not a {} ! ",
                new_path.green().bold(),
                "directory".green().bold()
            );
        } else {
            println!(
                "{} exist and it's a {}, but it's already contain a element with the same name {}!",
                new_path.green().bold(),
                "directory".green().bold(),
                trash_item.name.green().bold()
            );
        }
    }
    fs::rename(
        &path_in_trash,
        &format!("{}/{}", &new_path, &trash_item.name),
    )
    .unwrap();
    println!("{} has been restored ! :D", trash_item.name.green().bold());
    println!(
        "You can find it at this path: {}",
        format!("{}/{}", &new_path, &trash_item.name).green().bold()
    );
}

#[cfg(test)]
mod tests {}
