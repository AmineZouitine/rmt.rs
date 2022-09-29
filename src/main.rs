pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod structure_manager;
pub mod trash_item;
pub mod trash_manager;

fn main() {
    let (config, _) = structure_manager::setup_structure(true);
    trash_manager::convert_element_to_trash_item(&config, "test", true);
    // println!("{}", trash_manager::abspath("../rmt.rs").unwrap());
}
