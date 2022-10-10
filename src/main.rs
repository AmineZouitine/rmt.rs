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

const USAGE: &'static str = "

Usage:
    rmt [FILES]  -> use to remove a file or a folder
    rmt trash_display  -> use to restore and delete an file or a folder
    rmt trash_info     -> use to have information about the trash
    rmt trash_flush    -> use to clear all the trash
Options:
  --h     Show this screen.
  -f      remove all warnings
Exemple:
    rmt test.txt 
    rmt test.*
";

fn main() {
    let is_test = true;
    let (config, connection) = structure_manager::setup_structure(is_test);
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "{}\nYou should use {}",
            "The arguments are not valid.".red().bold(),
            "rmt --h".green().bold()
        );
        return;
    }
    if args.contains(&String::from("--h")) {
        println!("{}", USAGE);
        return;
    }
    if args.contains(&String::from("trash_display")) {
        input_manager::start_display(&connection, is_test);
        return;
    }
    if args.contains(&String::from("trash_flush")) {
        input_manager::start_display(&connection, is_test);
        return;
    }
    if args.contains(&String::from("trash_info")) {
        input_manager::start_display(&connection, is_test);
        return;
    }
    if args.len() >= 2 {
        let mut user_input = String::new();
        println!("{} will be add to the trash, are you sure ? [y/n] (add {} option to get no more warnings", args.len().to_string().green().bold(), "-f".green().bold());
        std::io::stdin().read_line(&mut user_input).unwrap();
        if user_input != "y" || user_input != "yes" {
            return;
        }
    }
    trash_manager::add_all_elements_to_trash(&connection, &config, &args[1..], true);
}
