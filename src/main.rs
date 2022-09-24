pub mod config;
pub mod config_manager;

fn main() {
    let current_config = config_manager::config_setup();
}
