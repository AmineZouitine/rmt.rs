pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod trash_item;

fn main() {
    let current_config = config_manager::config_setup();
}
