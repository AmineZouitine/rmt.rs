use std::vec;

use crate::trash_item::TrashItem;
use crate::{structure_manager, trash_item};
use rusqlite::{params, types::FromSql, Connection, Row};

pub fn setup_data_base(is_test: bool) -> Connection {
    let connection = structure_manager::create_data_base_file(is_test);
    let table_name = structure_manager::get_data_base_table_name(is_test);

    connection
        .execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             hash NOT NULL UNIQUE,
             path TEXT NOT NULL,
             date TEXT NOT NULL,
             real_size INTEGER NOT NULL,
             compression_size INTEGER
         )",
                table_name
            ),
            [],
        )
        .expect(&format!(
            "Unable to execute creation of {} table",
            table_name
        ));

    connection
}

fn get<T: FromSql>(row: &Row, index: usize) -> T {
    let element: T = row.get(index).expect(&format!("Get a {} not valid", index));
    element
}

pub fn find_all_trash_items(connection: &Connection, is_test: bool) -> Vec<TrashItem> {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    let mut stmt = connection
        .prepare(&format!("SELECT * FROM {}", table_name))
        .expect("Cannot select every element in database");

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
        })
    });

    for item in items.unwrap() {
        trash_items.push(item.unwrap());
    }

    trash_items
}

pub fn find_trash_item_by_id(connection: &Connection, is_test: bool, id: i8) -> TrashItem {
    find_all_trash_items(connection, is_test)
        .into_iter()
        .find(|trash_item| trash_item.id == id)
        .unwrap()
}


pub fn delete_trash_item_by_id(connection: &Connection, is_test: bool, id: i8) {
    let table_name = structure_manager::get_data_base_table_name(is_test);
    connection.execute(&format!("DELETE FROM {} WHERE id = {}", table_name, id), params![]).unwrap();
}

pub fn draw_data_base(connection: &Connection, is_test: bool) {
    let trash_items = find_all_trash_items(connection, is_test);
    for (i, item) in trash_items.iter().enumerate() {
        println!("{} -> {:?}", i, item);
    }
}

pub fn insert_trash_item(connection: &Connection, trash_item: &TrashItem, is_test: bool) {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    connection
        .execute(
            &format!("INSERT INTO {} (name, hash, path, date, real_size, compression_size) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", table_name),
            params![
                trash_item.name,
                trash_item.hash,
                trash_item.path,
                trash_item.date,
                trash_item.real_size,
                trash_item.compression_size,
            ],
        )
        .expect(&format!(
            "Unable to do insert element request with this values : {:?}",
            trash_item
        ));
}

pub fn delete_trash_item(connection: &Connection, trash_item_id: i8, is_test: bool) {
    let table_name = structure_manager::get_data_base_table_name(is_test);

    connection
        .execute(
            &format!("DELETE FROM {} WHERE id = (?1)", table_name),
            params![trash_item_id],
        )
        .expect(&format!(
            "Unable to do delete element request with this id : {}",
            trash_item_id
        ));
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
        );
        insert_trash_item(&connection, &trash_item, is_test);

        let trash_items = find_all_trash_items(&connection, is_test);

        structure_manager::clear_structure(is_test);
        assert_eq!(trash_items.len(), 1);
        trash_item.id = trash_items[0].id;
        assert_eq!(trash_items[0], trash_item);
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
        );
        insert_trash_item(&connection, &trash_item, is_test);

        let trash_items = find_all_trash_items(&connection, is_test);

        structure_manager::clear_structure(is_test);
        assert_eq!(trash_items.len(), 1);
        trash_item.id = trash_items[0].id;
        assert_eq!(trash_items[0], trash_item);
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
        );

        let mut trash_item2 = TrashItem::new(
            "Amine".to_string(),
            "Unique2".to_string(),
            "home/user".to_string(),
            "00::00::01".to_string(),
            10,
            Some(4),
        );

        insert_trash_item(&connection, &trash_item1, is_test);
        insert_trash_item(&connection, &trash_item2, is_test);

        let trash_items = find_all_trash_items(&connection, is_test);

        structure_manager::clear_structure(is_test);
        assert_eq!(trash_items.len(), 2);

        trash_item1.id = trash_items[0].id;
        trash_item2.id = trash_items[1].id;

        assert!(trash_items.contains(&trash_item1));
        assert!(trash_items.contains(&trash_item2));
    }

    #[test]
    fn test_delete_trash_item_() {
        let is_test = true;
        let (_, connection) = structure_manager::setup_structure(is_test);

        let trash_item = TrashItem::new(
            "Amine".to_string(),
            "Unique1".to_string(),
            "home/user".to_string(),
            "00::00::01".to_string(),
            10,
            None,
        );

        insert_trash_item(&connection, &trash_item, is_test);
        let mut trash_items = find_all_trash_items(&connection, is_test);
        assert_eq!(trash_items.len(), 1);

        delete_trash_item(&connection, trash_items[0].id, is_test);
        trash_items = find_all_trash_items(&connection, is_test);
        assert_eq!(trash_items.len(), 0);

        structure_manager::clear_structure(is_test);
    }
}
