pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod trash_item;
pub mod trash_manager;

fn main() {
    let current_config = config_manager::config_setup();
    trash_manager::convert_element_to_trash_item(&current_config, "");
    // trash_manager::convert_element_to_trash_item("test.txt");
}
