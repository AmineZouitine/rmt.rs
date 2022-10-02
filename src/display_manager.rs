use rusqlite::Connection;

use crate::data_manager;

pub fn display_trash(connection: &Connection, is_test: bool) {
    let trash_items = data_manager::find_all_trash_items(connection, is_test);
    for (i, trash_item) in trash_items.iter().enumerate() {
        println!("{}➜ {:?}", i, trash_item);
    }
}
