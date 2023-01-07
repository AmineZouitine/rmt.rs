use std::{fs, path::Path};

use crate::{argument_errors::RmtArgumentErrors, structure_manager::relative_path_to_absolute};
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Amine Zouitine", version, about)]
pub struct ArgumentsManager {
    pub elements: Vec<String>,

    /// remove the element without placing it in the trash
    #[arg(long = "destroy")]
    pub is_destroy: bool,

    /// ignore nonexistent files and arguments, never prompt
    #[arg(short = 'f', long = "force")]
    pub is_force: bool,

    /// prompt before every removal
    #[arg(short = 'i')]
    pub confirmation_always: bool,

    ///  prompt once before removing more than three files, or  when removing  recursively;
    #[arg(short = 'I')]
    pub confirmation_once: bool,

    /// remove directories and their contents recursively
    #[arg(short = 'r', long = "recursive")]
    pub is_recursive: bool, // -r -R --recursive

    /// remove empty directories
    #[arg(short = 'd', long = "dir")]
    pub is_empty_dir: bool, // -d --dir

    /// Explain what is being done
    #[arg(short = 'v', long = "verbose")]
    pub is_verbose: bool,

    /// Open trash manager CLI, use to restore or delete element from the trash
    #[arg(long = "td")]
    pub is_trash_display: bool, // rmt trash_display or rmt td

    /// Show some informations about the trash
    #[arg(long = "ti")]
    pub is_trash_info: bool, // rmt trash_info or rmt ti

    /// Flush all the elements present in the trash
    #[arg(long = "tf")]
    pub is_trash_flush: bool, // rmt trash_flush or rmt tf

    /// Switch to test mode
    #[arg(long = "test")]
    pub is_test: bool, // rmt --test
}

impl ArgumentsManager {
    pub fn filter_all_errors(&mut self) -> i32 {
        let mut exit_code = 0;
        let mut result: Vec<String> = Vec::new();
        self.elements
            .iter()
            .for_each(|path| match self.filter_error(path) {
                Ok(absolute_path_opt) => {
                    if let Some(absolute_path) = absolute_path_opt {
                        result.push(absolute_path);
                    }
                }
                Err(arg_error) => {
                    println!("{}", arg_error);
                    exit_code = 1;
                }
            });
        self.elements = result;
        exit_code
    }

    fn filter_error(&self, path: &str) -> Result<Option<String>, RmtArgumentErrors> {
        match relative_path_to_absolute(path) {
            Ok(path) => {
                if Path::new(&path).is_dir() {
                    let element_in_folder = fs::read_dir(&path).unwrap().count();
                    if element_in_folder == 0 && !self.is_empty_dir && !self.is_recursive {
                        return Err(RmtArgumentErrors::InvalidEmptyFolderFlags {
                            folder_name: path,
                        });
                    } else if element_in_folder > 0 && self.is_empty_dir {
                        return Err(RmtArgumentErrors::InvalidDirFlags {
                            folder_name: path,
                            element_in_folder,
                        });
                    } else if element_in_folder > 0 && !self.is_recursive {
                        return Err(RmtArgumentErrors::InvalidFillFolderFlags {
                            folder_name: path,
                        });
                    }
                }
                Ok(Some(path))
            }
            Err(invalid_path) => {
                if self.is_force {
                    Ok(None)
                } else {
                    Err(invalid_path)
                }
            }
        }
    }
}
