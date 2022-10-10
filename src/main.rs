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

const USAGE: &str = "
Usage:
    rmt [FILES]  -> use to remove a file or a folder
    rmt trash_displair | rmt td  -> use to restore and delete an file or a folder
    rmt trash_info | rmt ti    -> use to have information about the trash
    rmt trash_flush | rmt tf   -> use to clear all the trash
Options:
  -h --help     Show this screen.
  -f            remove all warnings
  -v            --verbose print deleted elements
  -d            remove element without add it on the trash
Exemple:
    rmt test.txt 
    rmt test.*
";

fn main() {
    let is_test = true;
    let (config, connection) = structure_manager::setup_structure(is_test);
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "{}\nYou should use {}",
            "The arguments are not valid.".red().bold(),
            "rmt --h".green().bold()
        );
        return;
    }
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("{}", USAGE);
        return;
    }
    if args.contains(&String::from("trash_display")) || args.contains(&String::from("td")) {
        input_manager::start_display(&connection, is_test);
        return;
    }
    if args.contains(&String::from("trash_flush")) || args.contains(&String::from("tf")) {
        let mut user_input = String::new();
        println!("Are you sure to flush all the elements of your trash ? [y/n]");
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.pop();
        if user_input != "y" && user_input != "yes" {
            return;
        }
        data_manager::delete_all_trash_item(&connection, is_test);
        return;
    }
    if args.contains(&String::from("trash_info")) || args.contains(&String::from("ti")) {
        trash_manager::display_trash_information(&connection, is_test);
        return;
    }
    let is_force = args.contains(&String::from("-f"));
    let is_destroy = args.contains(&String::from("-d"));
    let is_verbose =
        args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    args.retain(|arg| arg != "-f" && arg != "-v" && arg != "--verbose" && arg != "-d");

    if args.len() > 2 && !is_force {
        let mut user_input = String::new();
        println!("{} will be add to the trash, are you sure ? [y/n] (add {} option to get no more warnings)", (args.len() - 1).to_string().green().bold(), "-f".green().bold());
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.pop();
        if user_input != "y" && user_input != "yes" {
            return;
        }
    }

    trash_manager::add_all_elements_to_trash(
        &connection,
        &config,
        &args[1..],
        is_test,
        is_verbose,
        is_destroy,
    );
}
