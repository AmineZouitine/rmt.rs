extern crate termion;

use rusqlite::Connection;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::display_manager;

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

    let mut cursor_index = 0;
    println!("Which elements do you want to restore ?\r");
    display_manager::display_trash(connection, is_test, cursor_index);
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
            Key::Char('k') | Key::Up => set_cursor(
                &mut cursor_index,
                display_manager::MAX_ELEMENT_PER_PAGE,
                true,
            ),
            Key::Char('j') | Key::Down => set_cursor(
                &mut cursor_index,
                display_manager::MAX_ELEMENT_PER_PAGE,
                false,
            ),
            Key::Esc => println!("Toggle_filter"),
            Key::Ctrl('d') => println!("Clear filter"),
            Key::Char(' ') => println!("Toggle restore"),
            Key::Char('\n') => println!("Validation"),
            Key::Delete => println!("Toggle flush"),
            e => println!("Salut {:?}", e),
        }

        println!("Which elements do you want to restore ?\n\r");
        display_manager::display_trash(connection, is_test, cursor_index);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
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
