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
use colored::Colorize;
use std::env;

const USAGE: &str = "
NAME
       rmt - remove files or directories and save it

SYNOPSIS
       rmt [OPTION]... [FILE]...   -> Use to remove an element and save it
       rmt trash_display   -> Use to open trash CLI to restore or delete elements in the trash
       rmt trash_info    -> Use print some informations about the trash
       rmt trash_flush    -> Use to delete every element in the trash

SHORTCUTS
       rmt trash_display ->   rmt td
       rmt trash_info    ->   rmt ti
       rmt trash_flush   ->   rmt tf
OPTIONS

       -f, --force
              ignore nonexistent files and arguments, never prompt

       -i     prompt before every removal

       -I     prompt once before removing more than three files, or  when
              removing  recursively;  less intrusive than -i, while still
              giving protection against most mistakes

       --interactive[=WHEN]
              prompt according to WHEN: never, once (-I), or always (-i);
              without WHEN, prompt always

       -r, -R, --recursive
              remove directories and their contents recursively

       -d, --dir
              remove empty directories

       -v, --verbose
              explain what is being done

       --help display this help and exit

       By default, rm does not remove directories.  Use  the  --recursive
       (-r or -R) option to remove each listed directory, too, along with
       all of its contents.
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

    let arguments_manager = ArgumentsManager::new(&args);
    ArgumentsManager::filter_args(&mut args);

    if arguments_manager.is_help {
        println!("{}", USAGE);
    } else if arguments_manager.is_trash_display {
        input_manager::start_display(&connection, is_test);
    } else if arguments_manager.is_trash_flush {
        let mut user_input = String::new();
        println!("Are you sure to flush all the elements of your trash ? [y/n]");
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
            &args[1..],
            is_test,
            &arguments_manager,
        );
    }
}
