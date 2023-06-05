use crate::trash_item::TrashItem;
use crate::{database_errors::RmtDataBaseErrors, structure_manager};

use field_count::FieldCount;
use rusqlite::{params, types::FromSql, Connection, Row};
use std::path::{Path, MAIN_SEPARATOR};
use std::process::exit;

pub fn setup_data_base(is_test: bool) -> Connection {
    let connection = create_database(is_test);
    verification_database_item_exist_in_trash_folder(&connection, is_test);

    connection
}

// Create the database and the table to save information about deleted elements. (if database doesn't already exist)
pub fn create_database(is_test: bool) -> Connection {
    let connection = structure_manager::create_data_base_file(is_test);
    let table_name = structure_manager::get_data_base_table_name(is_test);

    let cols = get_column_count(&connection, is_test);
    if cols == 0 {
        let stmt_result = connection.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                 id INTEGER PRIMARY KEY,
                 name TEXT NOT NULL,
                 hash NOT NULL UNIQUE,
                 path TEXT NOT NULL,
                 date TEXT NOT NULL,
                 real_size INTEGER NOT NULL,
                 compression_size INTEGER,
                 is_folder INTEGER NOT NULL,
                 is_encrypted INTEGER NOT NULL,
                 is_compressed INTEGER NOT NULL
                )",
                table_name
            ),
            [],
        );

        match stmt_result {
            Ok(_) => connection,
            Err(_) => {
                println!("{}", RmtDataBaseErrors::DataBaseCreation);
                exit(1);
            }
        }
    } else if cols != TrashItem::field_count() {
        let stmt_result = connection.execute(
            &format!(
                "ALTER TABLE {} ADD COLUMN is_compressed INTEGER NOT NULL DEFAULT 0",
                table_name
            ),
            [],
        );
        match stmt_result {
            Ok(_) => connection,
            Err(_) => {
                println!(
                    "{}",
                    "Failed to add 'is_compressed' column to the existing trash database"
                );
                exit(1);
            }
        }
    } else {
        connection
    }
}

fn get_column_count(connection: &Connection, is_test: bool) -> usize {
    let table_name = structure_manager::get_data_base_table_name(is_test);
    let mut cols = 0;
    let stmt_result = connection.pragma(None, "table_info", table_name, |_| {
        cols += 1;
        Ok(())
    });
    match stmt_result {
        Ok(_) => cols,
        Err(_) => {
            println!("{}", RmtDataBaseErrors::CountAllColumns);
            exit(1);
        }
    }
}

// Check if every element from the database exist on the trash
// It's prevent error if user delete (without using rmt --td or rm --tf) element directly on the trash folder
pub fn verification_database_item_exist_in_trash_folder(connection: &Connection, is_test: bool) {
    let trash_items = find_all_trash_items(connection, is_test);

    trash_items.iter().for_each(|item| {
        let path = format!(
            "{}{}{}",
            structure_manager::get_trash_directory_path(is_test),
            MAIN_SEPARATOR,
            item.hash
        );
        if !Path::new(&path).exists() {
            delete_trash_item(connection, item.id, is_test);
        }
    })
}

fn get<T: FromSql>(row: &Row, index: usize) -> T {
    match row.get(index) {
        Ok(element) => element,
        Err(_) => {
            println!("{}", RmtDataBaseErrors::GetCellElement(index));
            exit(1);
        }
    }
}

// Find all elements on the table and convert them to TrashItems
pub fn find_all_trash_items(connection: &Connection, is_test: bool) -> Vec<TrashItem> {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    let mut stmt = match connection.prepare(&format!("SELECT * FROM {}", table_name)) {
        Ok(stmt) => stmt,
        Err(_) => {
            println!("{}", RmtDataBaseErrors::SelectAllElements);
            exit(1);
        }
    };

    let mut trash_items = Vec::<TrashItem>::new();

    let items = stmt.query_map((), |row| {
        Ok(TrashItem {
            id: get(row, 0),
            name: get(row, 1),
            hash: get(row, 2),
            path: get(row, 3),
            date: get(row, 4),
            real_size: get(row, 5),
            compression_size: get(row, 6),
            is_folder: get(row, 7),
            is_encrypted: get(row, 8),
            is_compressed: get(row, 9),
        })
    });

    for item in items.unwrap() {
        trash_items.push(item.unwrap());
    }

    // sort by adding date
    trash_items.into_iter().rev().collect()
}

// Get a trash item by id, need to refactor because it's not the best way to do it
pub fn find_trash_item_by_id(
    connection: &Connection,
    is_test: bool,
    id: i32,
) -> Result<TrashItem, RmtDataBaseErrors> {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    let stmt_result = connection.query_row(
        &format!("SELECT * FROM {} where id = ?1", table_name),
        [id],
        |row| {
            Ok(TrashItem {
                id: get(row, 0),
                name: get(row, 1),
                hash: get(row, 2),
                path: get(row, 3),
                date: get(row, 4),
                real_size: get(row, 5),
                compression_size: get(row, 6),
                is_folder: get(row, 7),
                is_encrypted: get(row, 8),
                is_compressed: get(row, 9),
            })
        },
    );

    match stmt_result {
        Ok(row) => Ok(row),
        Err(_) => Err(RmtDataBaseErrors::GetCellElement(id as usize)),
    }
}

pub fn get_element_count(
    connection: &Connection,
    is_test: bool,
) -> Result<usize, RmtDataBaseErrors> {
    let table_name = structure_manager::get_data_base_table_name(is_test);
    let stmt_result = connection.query_row(
        &format!("SELECT COUNT(*) FROM {} ", table_name),
        (),
        |row| row.get(0),
    );

    match stmt_result {
        Ok(row) => Ok(row),
        Err(_) => Err(RmtDataBaseErrors::CountAllElements),
    }
}

pub fn delete_trash_item_by_id(connection: &Connection, is_test: bool, id: i32) {
    let table_name = structure_manager::get_data_base_table_name(is_test);
    connection
        .execute(
            &format!("DELETE FROM {} WHERE id = {}", table_name, id),
            params![],
        )
        .unwrap();
}

pub fn insert_trash_item(connection: &Connection, trash_item: &TrashItem, is_test: bool) {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    let stmt_result = connection
        .execute(
            &format!("INSERT INTO {} (name, hash, path, date, real_size, compression_size, is_folder, is_encrypted, is_compressed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", table_name),
            params![
                trash_item.name,
                trash_item.hash,
                trash_item.path,
                trash_item.date,
                trash_item.real_size,
                trash_item.compression_size,
                trash_item.is_folder,
                trash_item.is_encrypted,
                trash_item.is_compressed
            ],
        );

    match stmt_result {
        Ok(_) => (),
        Err(_) => println!("{}", RmtDataBaseErrors::InsertTrashItem),
    };
}

pub fn delete_trash_item(connection: &Connection, trash_item_id: i32, is_test: bool) {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    let stmt_result = connection.execute(
        &format!("DELETE FROM {} WHERE id = (?1)", table_name),
        params![trash_item_id],
    );

    match stmt_result {
        Ok(_) => (),
        Err(_) => {
            println!("{}", RmtDataBaseErrors::DeleteElementById(trash_item_id));
            exit(1);
        }
    }
}

pub fn delete_all_trash_item(connection: &Connection, is_test: bool) {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    connection
        .execute(&format!("DELETE FROM {}", table_name), params![])
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_without_compression() {
        let is_test = true;

        let (_, connection) = structure_manager::setup_structure(is_test);

        let mut trash_item = TrashItem::new(
            "Amine".to_string(),
            "test".to_string(),
            "home/user".to_string(),
            "00::00::01".to_string(),
            10,
            None,
            false,
            false,
            false,
        );

        insert_trash_item(&connection, &trash_item, is_test);

        let trash_items = find_all_trash_items(&connection, is_test);

        assert_eq!(trash_items.len(), 1);

        trash_item.id = trash_items[0].id;
        assert_eq!(trash_items[0], trash_item);

        connection
            .close()
            .expect("Unable to close sqlite connection");
        structure_manager::clear_structure(is_test);
    }

    #[test]
    fn test_insert_compression() {
        let is_test = true;
        let (_, connection) = structure_manager::setup_structure(is_test);

        let mut trash_item = TrashItem::new(
            "Amine".to_string(),
            "Unique".to_string(),
            "home/user".to_string(),
            "00::00::01".to_string(),
            10,
            Some(4),
            false,
            false,
            true,
        );
        insert_trash_item(&connection, &trash_item, is_test);

        let trash_items = find_all_trash_items(&connection, is_test);

        assert_eq!(trash_items.len(), 1);

        trash_item.id = trash_items[0].id;
        assert_eq!(trash_items[0], trash_item);

        connection
            .close()
            .expect("Unable to close sqlite connection");
        structure_manager::clear_structure(is_test);
    }

    #[test]
    fn test_insert_multiple() {
        let is_test = true;
        let (_, connection) = structure_manager::setup_structure(is_test);

        let mut trash_item1 = TrashItem::new(
            "Amine".to_string(),
            "Unique1".to_string(),
            "home/user".to_string(),
            "00::00::01".to_string(),
            10,
            None,
            false,
            false,
            false,
        );

        let mut trash_item2 = TrashItem::new(
            "Amine".to_string(),
            "Unique2".to_string(),
            "home/user".to_string(),
            "00::00::01".to_string(),
            10,
            Some(4),
            false,
            false,
            true,
        );

        insert_trash_item(&connection, &trash_item1, is_test);
        insert_trash_item(&connection, &trash_item2, is_test);

        let trash_items = find_all_trash_items(&connection, is_test);

        assert_eq!(trash_items.len(), 2);

        trash_item1.id = trash_items[1].id;
        trash_item2.id = trash_items[0].id;
        assert!(trash_items.contains(&trash_item1));
        assert!(trash_items.contains(&trash_item2));

        connection
            .close()
            .expect("Unable to close sqlite connection");
        structure_manager::clear_structure(is_test);
    }

    #[test]
    fn test_delete_trash_item() {
        let is_test = true;
        let (_, connection) = structure_manager::setup_structure(is_test);

        let trash_item = TrashItem::new(
            "Amine".to_string(),
            "Unique1".to_string(),
            "home/user".to_string(),
            "00::00::01".to_string(),
            10,
            None,
            false,
            false,
            false,
        );

        insert_trash_item(&connection, &trash_item, is_test);
        let mut trash_items = find_all_trash_items(&connection, is_test);
        assert_eq!(trash_items.len(), 1);

        delete_trash_item(&connection, trash_items[0].id, is_test);
        trash_items = find_all_trash_items(&connection, is_test);
        assert_eq!(trash_items.len(), 0);

        connection
            .close()
            .expect("Unable to close sqlite connection");
        structure_manager::clear_structure(is_test);
    }

    #[test]
    fn test_add_is_compressed_column_to_existing_database() {
        let is_test = false;
        let (_, connection) = structure_manager::setup_structure(is_test);

        assert_eq!(
            get_column_count(&connection, is_test),
            TrashItem::field_count()
        );

        connection
            .close()
            .expect("Unable to close sqlite connection");
        structure_manager::clear_structure(is_test)
    }
}
