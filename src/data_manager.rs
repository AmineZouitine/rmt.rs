use crate::trash_item::{Compression, TrashItem};
use rusqlite::{types::FromSql, Connection, Row};

const DATA_BASE_NAME: &str = "trash_table.db";

pub fn get_connection() -> Connection {
    let connection = Connection::open(DATA_BASE_NAME).expect("Unable to create trash table");

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

pub fn find_all_trash_items(connection: &Connection) -> Vec<TrashItem> {
    let mut stmt = connection
        .prepare(&format!("SELECT * FROM {}", DATA_BASE_NAME))
        .expect("Cannot select every element in database");

    let mut trash_items = Vec::<TrashItem>::new();

    let items = stmt.query_map((), |row| {
        let compression_method: Option<String> = get(row, 6);
        let compression_size: Option<u8> = get(row, 7);

        let compression = if compression_method.is_none() || compression_size.is_none() {
            None
        } else {
            Some(Compression::new(
                compression_method.unwrap(),
                compression_size.unwrap(),
            ))
        };

        Ok(TrashItem {
            id: get(row, 0),
            name: get(row, 1),
            hash: get(row, 2),
            path: get(row, 3),
            date: get(row, 4),
            real_size: get(row, 5),
            compression,
        })
    });

    for item in items.unwrap() {
        trash_items.push(item.unwrap());
    }

    trash_items
}

pub fn draw_data_base(connection: &Connection) {
    let trash_items = get_all_trash_items(connection);
    for (i, item) in trash_items.iter().enumerate() {
        println!("{} -> {:?}", i, item);
    }
}
