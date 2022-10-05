use crate::data_manager;
use colored::{ColoredString, Colorize};
use rusqlite::Connection;

pub const MAX_ELEMENT_PER_PAGE: usize = 10;

pub struct DisplayInfos {
    pub current_cursor_index: usize,
    pub current_page: usize,
    pub total_elements: usize,
    pub max_element_per_page: usize,
    pub total_page: f64, // Should not be f64 but usize, but ceiling problems
    pub selected_trash_items: SelectedTrashItems,
}

pub struct SelectedTrashItems {
    pub restore: Vec<u8>, // store the id of the trash item
    pub delete: Vec<u8>,  // store the id of the trash item
}

impl DisplayInfos {
    pub fn new(total_elements: usize) -> Self {
        Self {
            current_cursor_index: 0,
            current_page: 1,
            total_elements,
            max_element_per_page: MAX_ELEMENT_PER_PAGE,
            total_page: (total_elements as f64 / MAX_ELEMENT_PER_PAGE as f64).ceil(),
            selected_trash_items: SelectedTrashItems {
                restore: Vec::<u8>::new(),
                delete: Vec::<u8>::new(),
            },
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

    println!("\r");
    display_number(display_infos);
    println!("\r");
    display_inputs_commands();
}

fn display_number(display_infos: &DisplayInfos) {
    for i in 1..=display_infos.total_page as usize {
        if i == display_infos.current_page {
            print!("{}", "•".green().bold());
        } else {
            print!("{}", "•".bold());
        }
    }
    println!("\r");
}

fn display_inputs_commands() {
    println!(
        "{} • {} • {} • {} • {} • {} • {} • {}\r",
        display_input("↑/k", "up"),
        display_input("↓/j", "down"),
        display_input("esc", "filter"),
        display_input("ctrl(d)", "clear filter"),
        display_input("space", "restore"),
        display_input("del", "flush"),
        display_input("enter", "validation"),
        display_input("q", "exist")
    );
}

fn display_input(inputs: &str, name: &str) -> String {
    format!("{} {}", inputs.green().bold(), name.truecolor(150,150, 150))
}