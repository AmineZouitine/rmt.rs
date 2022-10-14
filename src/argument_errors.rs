use core::fmt;
use std::error::Error;

use colored::Colorize;

#[derive(Debug)]
pub enum RmtArgumentErrors {
    InvalidNumberOfArguments(usize),
    InvalidDirFlags {
        folder_name: String,
        element_in_folder: usize,
    },
    InvalidFillFolderFlags {
        folder_name: String,
    },
    InvalidEmptyFolderFlags {
        folder_name: String,
    },
    InvalidPath {
        element_name: String,
    },
}

impl RmtArgumentErrors {
    fn error_message(&self) -> String {
        match self {
            RmtArgumentErrors::InvalidNumberOfArguments(args_number) => format!(
                "The number of arguments ({}) is not valid.",
                args_number.to_string().red().bold(),
            ),
            RmtArgumentErrors::InvalidDirFlags { folder_name, element_in_folder } => format!("You cannot delete {} folder with the {} flags because there is {} elements inside, you should use {} flags instead.", folder_name.red().bold(), "-d".red().bold().green(), element_in_folder.to_string().red().bold(), "-r".green().bold()),
            RmtArgumentErrors::InvalidEmptyFolderFlags { folder_name } => format!("You cannot delete {} folder without using {} or {} flags.", folder_name.red().bold(), "-r".green().bold(), "-d".green().bold()),
            RmtArgumentErrors::InvalidFillFolderFlags { folder_name } => format!("You cannot delete {} folder without using {} flags (do no use {} flags because your directory isn't empty).", folder_name.red().bold(), "-r".green().bold(), "-d".green().bold()),
            RmtArgumentErrors::InvalidPath { element_name} => format!("you cannot destroy your {} because it doesn't exist (use the {} option to stop getting this warning).",  element_name.red().bold(), "-f".green().bold())
        }
    }

    fn default_help_message() -> String {
        format!(
            "Use the {} option to get more details on how to use {}.",
            "rmt --help".bold().green(),
            "rmt".bold()
        )
    }
}

impl fmt::Display for RmtArgumentErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}\n{}\n",
            "Error".red().bold(),
            self.error_message(),
            RmtArgumentErrors::default_help_message().italic()
        )
    }
}

impl Error for RmtArgumentErrors {}
