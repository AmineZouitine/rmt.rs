use crate::data_manager;
use colored::Colorize;
use rusqlite::Connection;

pub const MAX_ELEMENT_PER_PAGE: usize = 4;

pub fn display_trash(connection: &Connection, is_test: bool, current_element: usize) {
    let trash_items = data_manager::find_all_trash_items(connection, is_test);

    for (i, trash_item) in trash_items.iter().enumerate() {
        let display_element = format!("{} ➜ {}", i, trash_item);
        if i == current_element {
            print!("{}    ", ">".green().bold());
            println!("{}\r", display_element);
            continue;
        }
        println!("{}\r", display_element);
    }
}

fn set_cursor(current_index: &mut usize, max_element_per_page: usize, top: bool) {
    if !top && (*current_index as i32) % (max_element_per_page as i32) < max_element_per_page as i32
    {
        *current_index += 1;
    } else if top && (*current_index as i32) % (max_element_per_page as i32) > 0 {
        *current_index -= 1;
    }
}

fn get_page(current_index: usize, max_element_per_page: usize) -> usize {
    return ((current_index as f64 / max_element_per_page as f64).ceil()) as usize;
}
