pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod structure_manager;
pub mod trash_item;
pub mod trash_manager;
use std::{os::unix::process::CommandExt, process::Command};

fn main() {
    // let (config, _) = structure_manager::setup_structure(true);
    // trash_manager::convert_element_to_trash_item(&config, "test2", true);
    fs_extra::copy_items(
        &vec!["test"],
        "/home/amine/perso",
        &fs_extra::dir::CopyOptions::new(),
    )
    .unwrap();
}
