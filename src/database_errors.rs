use core::fmt;
use std::error::Error;

use colored::Colorize;

#[derive(Debug)]
pub enum RmtDataBaseErrors {
    DataBaseCreation,
    SelectAllElements,
    GetCellElement(usize),
    DeleteElementById(i32),
    InsertTrashItem,
    DeleteAllElement,
}

impl RmtDataBaseErrors {
    fn error_message(&self) -> String {
        match self {
            RmtDataBaseErrors::DataBaseCreation => "Impossible to create the database.".to_string(),
            RmtDataBaseErrors::SelectAllElements => {
                "Impossible to select all the elements.".to_string()
            }
            RmtDataBaseErrors::GetCellElement(index) => format!(
                "Impossible to retrieve the cell with index {}.",
                index.to_string().red().bold()
            ),
            RmtDataBaseErrors::DeleteElementById(id) => format!(
                "Impossible to delete the element at index {}.",
                id.to_string().red().bold()
            ),
            RmtDataBaseErrors::InsertTrashItem => "Impossible to insert the trashItem.".to_string(),
            RmtDataBaseErrors::DeleteAllElement => "Impossible to delete all elements.".to_string(),
        }
    }

    fn default_help_message() -> String {
        format!(
            "If you haven't touched the {} file yourself, please open an issue on: {} with as many details as possible.",
            ".trash_rmt".red().bold(),
            "https://github.com/AmineZouitine/rmt.rs".bold().green()
        )
    }
}

impl fmt::Display for RmtDataBaseErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}\n{}",
            "Error".red().bold(),
            self.error_message(),
            RmtDataBaseErrors::default_help_message().italic()
        )
    }
}

impl Error for RmtDataBaseErrors {}
