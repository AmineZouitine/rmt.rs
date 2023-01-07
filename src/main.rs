use rmt_lib::*;

use arguments_manager::ArgumentsManager;
use clap::Parser;
use colored::Colorize;

use crate::argument_errors::RmtArgumentErrors;

fn main() {
    std::env::set_var("CLICOLOR_FORCE", "true");
    let mut arguments_manager = ArgumentsManager::parse();
    let (config, connection) = structure_manager::setup_structure(arguments_manager.is_test);

    if arguments_manager.elements.is_empty()
        && !arguments_manager.is_trash_flush
        && !arguments_manager.is_trash_display
        && !arguments_manager.is_trash_info
    {
        println!("{}", RmtArgumentErrors::InvalidNumberOfArguments(0));
        std::process::exit(1);
    }

    let exit_code = arguments_manager.filter_all_errors();

    if arguments_manager.is_trash_display {
        input_manager::start_display(&connection, arguments_manager.is_test);
    } else if arguments_manager.is_trash_flush {
        let message = format!(
            "Are you sure to {} all the elements of your trash ?",
            "flush".green().bold()
        );
        if display_manager::get_user_validation(&message) {
            trash_manager::remove_all_elements(&connection, arguments_manager.is_test);
        }
    } else if arguments_manager.is_trash_info {
        trash_manager::display_trash_information(&connection, arguments_manager.is_test);
    } else {
        trash_manager::add_all_elements_to_trash(
            &connection,
            &config,
            &arguments_manager.elements,
            &arguments_manager,
        );
    }

    std::process::exit(exit_code)
}
