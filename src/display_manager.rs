use crate::data_manager;
use colored::Colorize;
use rusqlite::Connection;

pub const MAX_ELEMENT_PER_PAGE: usize = 6;

pub struct DisplayInfos {
    pub current_cursor_index: usize,
    pub current_page: usize,
    pub total_elements: usize,
    pub max_element_per_page: usize,
    pub total_page: f64, // Should not be f64 but usize, but ceiling problems
}

impl DisplayInfos {
    pub fn new(total_elements: usize) -> Self {
        Self {
            current_cursor_index: 0,
            current_page: 1,
            total_elements,
            max_element_per_page: MAX_ELEMENT_PER_PAGE,
            total_page: (total_elements as f64 / MAX_ELEMENT_PER_PAGE as f64).ceil(),
        }
    }
}

pub fn display_trash(connection: &Connection, is_test: bool, display_infos: &DisplayInfos) {
    let trash_items = data_manager::find_all_trash_items(connection, is_test);

    let starting_index = (display_infos.current_page - 1) * display_infos.max_element_per_page;
    let end_index = (display_infos.current_page) * display_infos.max_element_per_page;
    for i in starting_index..end_index {
        if i >= trash_items.len() {
            continue;
        }
        let display_element = format!("{} ➜ {}", i, trash_items[i]);
        if i == display_infos.current_cursor_index {
            print!("{}    ", ">".green().bold());
            println!("{}\r", display_element);
            continue;
        }
        println!("{}\r", display_element);
    }
}
