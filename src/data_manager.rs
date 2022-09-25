use crate::trash_item::TrashItem;
use rusqlite::{params, types::FromSql, Connection, Row};

const DATA_BASE_NAME: &str = "trash_table.db";
const TEST_DATA_BASE_NAME: &str = "test_trash_table.db";
const ABSOLUTE_PATH_DATABASE: &str = "/home/amine/perso/rmt.rs";

pub fn get_connection(is_test: bool) -> Connection {
    let data_base_name = if is_test {
        TEST_DATA_BASE_NAME
    } else {
        DATA_BASE_NAME
    };
    let connection = Connection::open(data_base_name).expect("Unable to create trash table");

    connection
        .execute(
            "create table if not exists trash (
             id integer primary key,
             name text not null,
             hash not null unique,
             path text not null,
             date text not null,
             real_size integer not null,
             compression_size integer,
             compression_method text,

         )",
            (),
        )
        .expect("Unable to execute creation trash table query");

    connection
}

fn get<T: FromSql>(row: &Row, index: usize) -> T {
    let element: T = row.get(index).expect(&format!("Get a {} not valid", index));
    element
}

pub fn find_all_trash_items(connection: &Connection, is_test: bool) -> Vec<TrashItem> {
    let data_base_name = if is_test {
        TEST_DATA_BASE_NAME
    } else {
        DATA_BASE_NAME
    };

    let mut stmt = connection
        .prepare(&format!("SELECT * FROM {}", data_base_name))
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
            compression_method: get(row, 6),
            compression_size: get(row, 7),
        })
    });

    for item in items.unwrap() {
        trash_items.push(item.unwrap());
    }

    trash_items
}

pub fn draw_data_base(connection: &Connection, is_test: bool) {
    let trash_items = find_all_trash_items(connection, is_test);
    for (i, item) in trash_items.iter().enumerate() {
        println!("{} -> {:?}", i, item);
    }
}

pub fn insert_element(connection: &Connection, trash_item: &TrashItem, is_test: bool) {
    let data_base_name = if is_test {
        TEST_DATA_BASE_NAME
    } else {
        DATA_BASE_NAME
    };

    connection
        .execute(
            "INSERT INTO (?1) VALUES (?2, ?3, ?4, ?5, 6?, 7?, 8?)",
            params![
                data_base_name,
                trash_item.name,
                trash_item.hash,
                trash_item.path,
                trash_item.date,
                trash_item.real_size,
                trash_item.compression_method,
                trash_item.compression_size,
            ],
        )
        .expect(&format!(
            "Unable to do insert element request with this values : {:?}",
            trash_item
        ));
}

#[cfg(test)]
mod tests {

    use std::{fs, path::Path};

    use super::*;

    fn delete_test_data_base() {
        let test_data_base_path = format!("{}/{}", ABSOLUTE_PATH_DATABASE, TEST_DATA_BASE_NAME);
        if Path::new(&test_data_base_path).is_file() {
            fs::remove_file(&test_data_base_path).expect("Unable to delete test database");
        }
    }

    #[test]
    fn test_insert_without_compression() {
        // let connection = get_connection(true);

        // let trash_item = TrashItem::new(
        //     1,
        //     "Amine".to_string(),
        //     "test".to_string(),
        //     "home/user".to_string(),
        //     "00::00::01".to_string(),
        //     10,
        //     None,
        //     None,
        // );
        // insert_element(&connection, &trash_item, true);

        // let trash_items = find_all_trash_items(&connection, true);

        // assert_eq!(trash_items.len(), 1);
        // assert_eq!(trash_items[0], trash_item);
        delete_test_data_base();
    }
}
