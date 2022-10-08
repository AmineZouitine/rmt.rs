pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod display_manager;
pub mod input_manager;
pub mod structure_manager;
pub mod trash_item;
pub mod trash_manager;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Add program args");
    }

    println!("{:?}", args);
    let is_test = true;

    let (config, connection) = structure_manager::setup_structure(is_test);
    // trash_manager::add_all_elements_to_trash(&connection, &config, &args[1..], true);
    input_manager::handle_input(&connection, is_test);
}
