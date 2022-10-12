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


fn main() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_description = env!("CARGO_PKG_DESCRIPTION");
    let author = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    let usage = format!(r###"
{pkg_name} - {pkg_description}
author: {author}
version {version}

SYNOPSIS
       {pkg_name} [OPTION]... [FILE]...   -> Use to remove an element and save it
       {pkg_name} trash_display   -> Use to open trash CLI to restore or delete elements in the trash
       {pkg_name} trash_info    -> Use print some informations about the trash
       {pkg_name} trash_flush    -> Use to delete every element in the trash

SHORTCUTS
       {pkg_name} trash_display ->   {pkg_name} td
       {pkg_name} trash_info    ->   {pkg_name} ti
       {pkg_name} trash_flush   ->   {pkg_name} tf
OPTIONS

       -f, --force
              ignore nonexistent files and arguments, never prompt

       -i     prompt before every removal

       -I     prompt once before removing more than three files, or  when
              removing  recursively;  less intrusive than -i, while still
              giving protection against most mistakes

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
"###);

    let is_test = false;
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
        println!("{}", usage);
    } else if arguments_manager.is_trash_display {
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
            &args[1..],
            is_test,
            &arguments_manager,
        );
    }
}
