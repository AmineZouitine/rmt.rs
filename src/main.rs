pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod display_manager;
pub mod structure_manager;
pub mod trash_item;
pub mod trash_manager;
use std::env;

use glob::glob;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        panic!("Add program args");
    }

    println!("{:?}", args);

    let (config, connection) = structure_manager::setup_structure(true);
    trash_manager::add_all_elements_to_trash(&connection, &config, &args[1..], true);
    display_manager::display_trash(&connection, true);

}
