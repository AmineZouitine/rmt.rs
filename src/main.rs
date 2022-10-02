pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod display_manager;
pub mod structure_manager;
pub mod trash_item;
pub mod trash_manager;

fn main() {
    let (config, connection) = structure_manager::setup_structure(true);
    trash_manager::add_element_to_trash(&connection, &config, "test", true);
    display_manager::display_trash(&connection, true);
}
