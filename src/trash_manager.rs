use crate::{
    config::Config,
    structure_manager::{self, get_trash_directory_path},
    trash_item::TrashItem,
};

use chrono;
use fs_extra::dir::get_size;
use sha256;
use std::{ffi::OsStr, path::Path, process::Command};

pub fn abspath(p: &str) -> Option<String> {
    shellexpand::full(p)
        .ok()
        .and_then(|x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok())
        .and_then(|p| p.into_os_string().into_string().ok())
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

    // println!(
    //     "1- compresse ->\ncommand->\ncd {}",
    //     &last_directory_before_target
    // );
    Command::new("cd").arg(last_directory_before_target);
    // println!(
    //     "2- compresse ->\ncommand->\nzip -r {} {}",
    //     &compression_name,
    //     &path_elements.last().unwrap()
    // );
    Command::new("zip")
        .arg("-r")
        .arg(&compression_name)
        .arg(path_elements.last().unwrap());
    println!(
        "3- compresse ->\ncommand->\nrm -rf {}",
        &path_elements.last().unwrap()
    );
    Command::new("rm")
        .arg("-rf")
        .arg(path_elements.last().unwrap());
    println!("4- compresse ->\ncommand->\ncd -");
    Command::new("cd").arg("-");

    get_size(&compression_name).expect(&format!(
        "Unable to get element size of {}",
        compression_name
    ))
}

pub fn convert_element_to_trash_item(
    config: &Config,
    element_name: &str,
    is_test: bool,
) -> TrashItem {
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

    let destination_path = structure_manager::get_trash_directory_path(is_test);

    let mut compression_size: Option<u64> = None;

    if config.compression {
        compression_size = Some(make_zip(&element_path, &destination_path, &hash));
    } else {
        let destination_path = format!("{}/{}", &get_trash_directory_path(is_test), &hash);
        println!(
            "Not compresse ->\ncommand->\nmv {} {}",
            element_path, destination_path
        );
        // Command::new("mv").arg(&element_path).arg(&destination_path);
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

#[cfg(test)]
mod tests {}
