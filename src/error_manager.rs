use core::fmt;
use std::{error::Error};

use colored::Colorize;

#[derive(Debug)]
pub enum RmtError {
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
    InvalidElementName {
        element_name: String,
    },
}

impl RmtError {
    fn error_message(&self) -> String {
        match self {
            RmtError::InvalidNumberOfArguments(args_number) => format!(
                "The number of arguments ({}) is not valid.",
                args_number.to_string().red().bold(),
            ),
            RmtError::InvalidDirFlags { folder_name, element_in_folder } => format!("You cannot delete {} folder with the {} flags because there is {} elements inside, you should use {} flags instead.", folder_name.red().bold(), "-d".red().bold().green(), element_in_folder.to_string().red().bold(), "-r".green().bold()),
            RmtError::InvalidEmptyFolderFlags { folder_name } => format!("You cannot delete {} folder without using {} or {} flags.", folder_name.red().bold(), "-r".green().bold(), "-d".green().bold()),
            RmtError::InvalidFillFolderFlags { folder_name } => format!("You cannot delete {} folder without using {} flags (do no use {} flags because your directory isn't empty).", folder_name.red().bold(), "-r".green().bold(), "-d".green().bold()),
            RmtError::InvalidElementName { element_name} => format!("you cannot destroy your {} because it doesn't exist (use the {} option to stop getting this warning).",  element_name.red().bold(), "-f".green().bold())
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

impl fmt::Display for RmtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}\n{}",
            "Error".red().bold(),
            self.error_message(),
            RmtError::default_help_message().italic()
        )
    }
}

impl Error for RmtError {}
