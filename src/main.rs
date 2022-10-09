pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod display_manager;
pub mod input_manager;
pub mod structure_manager;
pub mod trash_item;
pub mod trash_manager;
use std::env;

use colored::Colorize;

fn main(){
    let is_test = true;
    let (config, connection) = structure_manager::setup_structure(is_test);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}\nYou should use {}", "The arguments are not valid.".red().bold(), "rmt --h".green().bold());
        return;
    }
    if args.contains(&String::from("--h"))
    {
        println!("");
        return;
    }
    if args.contains(&String::from("display"))
    {
        input_manager::start_display(&connection, is_test);
        return;
    }
    trash_manager::add_all_elements_to_trash(&connection, &config, &args[1..], true);
}
