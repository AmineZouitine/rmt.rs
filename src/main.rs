pub mod arguments_manager;
pub mod config;
pub mod config_manager;
pub mod data_manager;
pub mod display_manager;
pub mod input_manager;
pub mod structure_manager;
pub mod trash_item;
pub mod trash_manager;
use arguments_manager::ArgumentsManager;
use clap::Parser;
use colored::Colorize;

fn main() {
    let is_test = false;
    let arguments_manager = ArgumentsManager::parse();
    let (config, connection) = structure_manager::setup_structure(is_test);

    if arguments_manager.elements.is_empty()
        && !arguments_manager.is_trash_flush
        && !arguments_manager.is_trash_display
        && !arguments_manager.is_trash_info
    {
        println!(
            "{}\nYou should use {}",
            "The arguments are not valid.".red().bold(),
            "rmt --help".green().bold()
        );
        return;
    }

    if arguments_manager.is_trash_display {
        input_manager::start_display(&connection, is_test);
    } else if arguments_manager.is_trash_flush {
        let mut user_input = String::new();
        println!(
            "Are you sure to {} all the elements of your trash ? {}",
            "flush".green().bold(),
            "[y/n]".green().bold()
        );
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.pop();
        if user_input == "y" || user_input == "yes" {
            data_manager::delete_all_trash_item(&connection, is_test);
        }
    } else if arguments_manager.is_trash_info {
        trash_manager::display_trash_information(&connection, is_test);
    } else {
        trash_manager::add_all_elements_to_trash(
            &connection,
            &config,
            &arguments_manager.elements,
            is_test,
            &arguments_manager,
        );
    }
}
