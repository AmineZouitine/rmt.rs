extern crate termion;

use rusqlite::Connection;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    data_manager,
    display_manager::{self, DisplayInfos},
};

pub fn handle_input(connection: &Connection, is_test: bool) {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();

    let mut display_informations = display_manager::DisplayInfos::new(
        data_manager::find_all_trash_items(connection, is_test).len(),
    );
    println!("Which elements do you want to restore ?\n\r");
    display_manager::display_trash(connection, is_test, &display_informations);
    stdout.flush().unwrap();
    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('z') => break,
            Key::Char('k') | Key::Up => set_cursor(&mut display_informations, true),
            Key::Char('j') | Key::Down => set_cursor(&mut display_informations, false),
            Key::Char('h') | Key::Left => set_page(
                &mut display_informations,
                false,
            ),
            Key::Char('l') | Key::Right => set_page(
                &mut display_informations,
                true,
            ),
            Key::Esc => println!("Toggle_filter"),
            Key::Ctrl('d') => println!("Clear filter"),
            Key::Char(' ') => println!("Toggle restore"),
            Key::Char('\n') => println!("Validation"),
            Key::Delete => println!("Toggle flush"),
            e => println!("Salut {:?}", e),
        }

        println!("Which elements do you want to restore ?\n\r");
        display_manager::display_trash(connection, is_test, &display_informations);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn set_cursor(display_infos: &mut DisplayInfos, top: bool) {
    if !top
        && display_infos.current_cursor_index + 1 * display_infos.current_page
            < display_infos.max_element_per_page * display_infos.current_page
        && display_infos.current_cursor_index < display_infos.total_elements - 1
    {
        display_infos.current_cursor_index += 1;
    } else if top
        && display_infos.current_cursor_index
            > display_infos.max_element_per_page * (display_infos.current_page - 1)
    {
        display_infos.current_cursor_index -= 1;
    }
}

fn set_page(display_infos: &mut DisplayInfos, next: bool) {
    if next && (display_infos.current_page as f64) < display_infos.total_page {
        display_infos.current_page += 1;
        display_infos.current_cursor_index =
            (display_infos.current_page - 1) * display_infos.max_element_per_page;
    } else if !next && display_infos.current_page > 1 {
        display_infos.current_page -= 1;
        display_infos.current_cursor_index =
            (display_infos.current_page - 1) * display_infos.max_element_per_page;
    }
}
// fn set_current_page()
