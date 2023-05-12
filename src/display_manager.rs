use crate::{data_manager, trash_item::TrashItem};
use colored::{ColoredString, Colorize};
use rusqlite::Connection;

pub const MAX_ELEMENT_PER_PAGE: usize = 10;

// All the informations about the display on for the "rmt display"
pub struct DisplayInfos {
    pub current_cursor_index: usize,
    pub current_page: usize,
    pub total_elements: usize,
    pub max_element_per_page: usize,
    pub total_page: f64, // Should not be f64 but usize, but ceiling problems
    pub filter: Filter,
    pub selected_trash_items: SelectedTrashItems,
}

impl DisplayInfos {
    pub fn new(total_elements: usize) -> Self {
        Self {
            current_cursor_index: 0,
            current_page: 1,
            total_elements,
            max_element_per_page: MAX_ELEMENT_PER_PAGE,
            total_page: (total_elements as f64 / MAX_ELEMENT_PER_PAGE as f64).ceil(),
            filter: Filter {
                is_filter: false,
                content: String::new(),
            },
            selected_trash_items: SelectedTrashItems {
                restore: Vec::<i32>::new(),
                delete: Vec::<i32>::new(),
            },
        }
    }
}

pub struct SelectedTrashItems {
    pub restore: Vec<i32>, // store the id of the trash item
    pub delete: Vec<i32>,  // store the id of the trash item
}

pub struct Filter {
    pub is_filter: bool,
    pub content: String,
}

impl Filter {
    pub fn is_valid_item(&self, trash_item: &TrashItem) -> bool {
        if !self.is_filter {
            true
        } else {
            trash_item.name.contains(&self.content)
                || trash_item.date.contains(&self.content)
                || trash_item.path.contains(&self.content)
        }
    }
}

pub fn display_trash(
    connection: &Connection,
    is_test: bool,
    display_infos: &mut DisplayInfos,
) -> i32 {
    println!("Which elements do you want to restore ?\n\r");

    // Getting all trash item from ddb
    let mut trash_items = data_manager::find_all_trash_items(connection, is_test);
    // filter item if the "filter mode" is activate
    trash_items.retain(|item| display_infos.filter.is_valid_item(item));
    display_infos.total_elements = trash_items.len();

    let starting_index = (display_infos.current_page - 1) * display_infos.max_element_per_page;
    let end_index = (display_infos.current_page) * display_infos.max_element_per_page;

    let mut current_selected_id = 0;

    for i in starting_index..end_index {
        if i >= trash_items.len() {
            continue;
        }
        let trash_item_str = format!("{} ➜ {}", i, trash_items[i]).white();
        let is_restore = display_infos
            .selected_trash_items
            .restore
            .contains(&trash_items[i].id);
        let is_delete = display_infos
            .selected_trash_items
            .delete
            .contains(&trash_items[i].id);
        let display_element = get_color_display_element(is_restore, is_delete, &trash_item_str);

        if i == display_infos.current_cursor_index || is_restore || is_delete {
            if i == display_infos.current_cursor_index {
                print!("{}", ">".green().bold());
                current_selected_id = trash_items[i].id;
            }
            print!("    ");
        }
        println!("{}\r", display_element);
    }

    println!("\r");
    display_pages_marks(display_infos);
    println!("\r");
    if display_infos.filter.is_filter {
        println!(
            "{}: {}",
            "Filter".green().bold(),
            display_infos.filter.content
        );
        println!("\r")
    }
    display_inputs_commands(display_infos.filter.is_filter);

    current_selected_id
}

// Use to display dot, to indicate the current page and the total numbers of pages
fn display_pages_marks(display_infos: &DisplayInfos) {
    for i in 1..=display_infos.total_page as usize {
        if i == display_infos.current_page {
            print!("{}", "•".green().bold());
        } else {
            print!("{}", "•".bold());
        }
    }
    println!("\r");
}

// display all the keybinding to interact with the cli
fn display_inputs_commands(is_filter: bool) {
    if !is_filter {
        println!(
            "{} • {} • {} • {} • {} • {} • {} • {}\r",
            display_input("↑/k", "up"),
            display_input("↓/j", "down"),
            display_input("esc", "filter"),
            display_input("ctrl(d)", "clear filter"),
            display_input("space", "restore"),
            display_input("del", "flush"),
            display_input("enter", "validation"),
            display_input("q", "exit")
        );
    } else {
        println!(
            "{} • {} • {}\r",
            display_input("esc", "filter"),
            display_input("ctrl(d)", "clear filter"),
            display_input("q", "exit")
        );
    }
}

fn display_input(inputs: &str, name: &str) -> String {
    format!(
        "{} {}",
        inputs.green().bold(),
        name.truecolor(150, 150, 150)
    )
}

fn get_color_display_element(is_restore: bool, is_delete: bool, text: &str) -> ColoredString {
    if is_restore {
        text.green().bold()
    } else if is_delete {
        text.red().bold()
    } else {
        text.white()
    }
}

pub fn get_user_validation(message: &str) -> bool {
    let mut user_input = String::new();
    println!("{} {}", message, "[y/n]".green().bold());
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input.pop();
    user_input == "y" || user_input == "yes"
}
