use crate::arguments_manager::ArgumentsManager;
use crate::display_manager;
use crate::argument_errors::RmtArgumentErrors;
use crate::structure_manager::{self, get_element_name, get_element_path, get_home_directory_path};
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
use std::io::{stdout, Write};
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
    arguments_manager: &ArgumentsManager,
) -> Result<(), RmtArgumentErrors> {
    let mut element_path = match abspath(element_name) {
        Some(path) => {
            if Path::new(&path).is_dir() {
                let element_in_dir = fs::read_dir(&path).unwrap().count();
                if element_in_dir == 0
                    && !arguments_manager.is_empty_dir
                    && !arguments_manager.is_recursive
                {
                    return Err(RmtArgumentErrors::InvalidEmptyFolderFlags {
                        folder_name: element_name.to_string(),
                    });
                } else if element_in_dir > 0 && arguments_manager.is_empty_dir {
                    return Err(RmtArgumentErrors::InvalidDirFlags {
                        folder_name: element_name.to_string(),
                        element_in_folder: element_in_dir,
                    });
                } else if element_in_dir > 0 && !arguments_manager.is_recursive {
                    return Err(RmtArgumentErrors::InvalidFillFolderFlags {
                        folder_name: element_name.to_string(),
                    });
                }
            }
            path
        }
        None => {
            if arguments_manager.is_force {
                return Ok(());
            }
            return Err(RmtArgumentErrors::InvalidElementName {
                element_name: element_name.to_string(),
            });
        }
    };

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

    let new_name = format!("{}/{}", get_trash_directory_path(is_test), hash);

    if config.compression {
        // TODO
    } else {
        let symbolic_path = format!(
            "{}/{}",
            get_element_path(&element_path),
            get_element_name(element_name)
        );
        if symbolic_path != element_path {
            element_path = symbolic_path;
        }
        fs::rename(&element_path, &new_name).unwrap();
    }

    let element_is_directory = Path::new(&new_name).is_dir();

    let trash_item = TrashItem::new(
        structure_manager::get_element_name(element_name),
        hash,
        structure_manager::get_element_path(&element_path),
        date.to_string(),
        element_size,
        compression_size,
        element_is_directory,
    );

    if !arguments_manager.is_destroy {
        data_manager::insert_trash_item(connection, &trash_item, is_test);
    }

    if arguments_manager.is_verbose {
        println!(
            "this {} {} has been added to the trash.",
            if element_is_directory {
                "directory".bold().white()
            } else {
                "file".bold().white()
            },
            element_name.green().bold()
        );
    }
    Ok(())
}

pub fn add_all_elements_to_trash(
    connection: &Connection,
    config: &Config,
    element_name: &[String],
    is_test: bool,
    arguments_manager: &ArgumentsManager,
) {
    if arguments_manager.confirmation_once && element_name.len() > 3 {
        let message = format!(
            "Sure you want to delete all {} files ?",
            element_name.len().to_string().bold().green()
        );
        if !display_manager::get_user_validation(&message) {
            return;
        }
    }
    for path in element_name {
        let message = format!("Are you sure to delete {} ?", path.bold().green());
        if !arguments_manager.confirmation_always || display_manager::get_user_validation(&message)
        {
            if let Err(rmt_error) =
                add_element_to_trash(connection, config, path, is_test, arguments_manager)
            {
                println!("{}", rmt_error);
            }
        }
    }
}

pub fn remove_all_elements(connection: &Connection, is_test: bool, trash_items_ids: &[i32]) {
    trash_items_ids.iter().for_each(|trash_item_id| {
        let trash_item = data_manager::find_trash_item_by_id(connection, is_test, *trash_item_id);
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
    if Path::new(&element_path).is_dir() {
        std::fs::remove_dir_all(&element_path).unwrap();
    } else {
        std::fs::remove_file(&element_path).unwrap();
    }

    println!(
        "{} {}\r",
        trash_item.name.red().bold(),
        "deleted !".red().bold()
    );
}

pub fn restore_all_elements(connection: &Connection, is_test: bool, trash_items_ids: &[i32]) {
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
            println!(
                "{} has been restored ! :D\r",
                trash_item.name.green().bold()
            );
            println!(
                "You can find it at this path: {}\r",
                element_path_name.green().bold()
            );
            fs::rename(&path_in_trash, &element_path_name).unwrap();
            return;
        } else if !Path::new(&element_path_restored).exists() {
            println!(
                "{} has been restored ! :D\r",
                trash_item.name.green().bold()
            );
            println!(
                "You can find it at this path: {}\r",
                element_path_restored.green().bold()
            );
            fs::rename(&path_in_trash, &element_path_restored).unwrap();
            return;
        }
    }
    println!("Unfortunately Path {} doesn't exist anymore or there is a file with the same name inside, so we can't restore your element to the original path :c\r\n{}\r",
     &trash_item.path.green().bold(), "Please enter a new absolute path to restore your element".bold());

    let mut new_path = get_home_directory_path();
    print!("{} {}", ">>".green().bold(), new_path.bold());
    stdout().flush().unwrap();
    std::io::stdin().read_line(&mut new_path).unwrap();
    new_path.pop();
    while !Path::new(&new_path).is_dir()
        || Path::new(&format!("{}/{}", &new_path, &trash_item.name)).exists()
    {
        if !Path::new(&new_path).exists() {
            println!(
                "{} doesn't exist ! You have to give a valid {} path of a {}\r",
                new_path.green().bold(),
                "absolute path".green().bold(),
                "directory".green().bold()
            );
        } else if !Path::new(&new_path).is_dir() {
            println!(
                "{} exist but it's not a {} ! \r",
                new_path.green().bold(),
                "directory".green().bold()
            );
        } else {
            println!(
                "{} exist and it's a {}, but it's already contain a element with the same name {}!\r",
                new_path.green().bold(),
                "directory".green().bold(),
                trash_item.name.green().bold()
            );
        }
        new_path.clear();
        new_path = get_home_directory_path();
        print!("{} {}", ">>".green().bold(), new_path.bold());
        stdout().flush().unwrap();
        std::io::stdin().read_line(&mut new_path).unwrap();
        new_path.pop();
    }
    fs::rename(
        &path_in_trash,
        &format!("{}/{}", &new_path, &trash_item.name),
    )
    .unwrap();

    println!(
        "{} has been restored ! :D\r",
        trash_item.name.green().bold()
    );
    if !new_path.is_empty() && new_path.as_bytes()[new_path.len() - 1] as char == '/' {
        new_path.pop();
    }
    println!(
        "You can find it at this path: {}\r",
        format!("{}/{}", &new_path, &trash_item.name).green().bold()
    );
}

pub fn display_trash_information(connection: &Connection, is_test: bool) {
    let trash_items = data_manager::find_all_trash_items(connection, is_test);
    let total_size = if let Some(size) = trash_items
        .iter()
        .map(|trash_item| trash_item.real_size)
        .reduce(|a, b| a + b)
    {
        size
    } else {
        0
    };
    println!(
        "{} elements are stored in the trash.",
        trash_items.len().to_string().green().bold()
    );
    println!(
        "{} {} is the total size of the trash.",
        (total_size / 1000).to_string().green().bold(),
        "ko".bold().white()
    )
}

#[cfg(test)]
mod tests {}
