use rusqlite::{Connection, Result};
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

pub fn draw_data_base(connection: &Connection) {}
