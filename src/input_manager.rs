extern crate termion;

use rusqlite::Connection;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::{
    data_manager,
    display_manager::{self, DisplayInfos},
    trash_manager,
};

pub fn handle_input(connection: &Connection, is_test: bool) {
    let stdin = stdin();
    let mut stdout_display = stdout().into_raw_mode().unwrap();

    write!(
        stdout_display,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();

    let mut display_informations = display_manager::DisplayInfos::new(
        data_manager::find_all_trash_items(connection, is_test).len(),
    );

    let mut current_selected_item =
        display_manager::display_trash(connection, is_test, &mut display_informations);

    stdout_display.flush().unwrap();
    for c in stdin.keys() {
        write!(
            stdout_display,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        if display_informations.filter.is_filter {
            match c.unwrap() {
                Key::Char(c) => {
                    display_informations.filter.content =
                        format!("{}{}", &display_informations.filter.content, c)
                }
                Key::Ctrl('d') => display_informations.filter.content.clear(),
                Key::Backspace => {
                    display_informations.filter.content.pop();
                }
                _ => display_informations.filter.is_filter = false,
            };
        } else {
            match c.unwrap() {
                Key::Char('q') | Key::Ctrl('c') | Key::Ctrl('z') => break,
                Key::Char('k') | Key::Up => set_cursor(&mut display_informations, true),
                Key::Char('j') | Key::Down => set_cursor(&mut display_informations, false),
                Key::Char('h') | Key::Left => set_page(&mut display_informations, false),
                Key::Char('l') | Key::Right => set_page(&mut display_informations, true),
                Key::Esc => {
                    display_informations.filter.is_filter = true;
                    display_informations.current_cursor_index = 0;
                    display_informations.current_page = 1;
                }
                Key::Ctrl('d') => display_informations.filter.content.clear(),
                Key::Char(' ') => toggle_item(
                    current_selected_item,
                    &mut display_informations.selected_trash_items.restore,
                    &mut display_informations.selected_trash_items.delete,
                ),
                Key::Delete => toggle_item(
                    current_selected_item,
                    &mut display_informations.selected_trash_items.delete,
                    &mut display_informations.selected_trash_items.restore,
                ),
                Key::Char('\n') => {
                    stdout_display.suspend_raw_mode().unwrap();
                    write!(stdout_display, "{}", termion::cursor::Show).unwrap();
                    write!(stdout_display, "{}", termion::clear::All).unwrap();

                    trash_manager::remove_all_elements(
                        connection,
                        is_test,
                        &display_informations.selected_trash_items.delete,
                    );
                    trash_manager::restore_all_elements(
                        connection,
                        is_test,
                        &display_informations.selected_trash_items.restore,
                    );
                    stdout_display.activate_raw_mode().unwrap();
                    break;
                }
                _ => (),
            }
        }
        current_selected_item =
            display_manager::display_trash(connection, is_test, &mut display_informations);

        stdout_display.flush().unwrap();
    }

    write!(stdout_display, "{}", termion::cursor::Show).unwrap();
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

fn toggle_item(selected_elements: i8, storage: &mut Vec<i8>, linked_storage: &mut Vec<i8>) {
    if storage.contains(&selected_elements) {
        let index = storage
            .iter()
            .position(|x| *x == selected_elements)
            .unwrap();
        storage.remove(index);
    } else {
        if linked_storage.contains(&selected_elements) {
            let index = linked_storage
                .iter()
                .position(|x| *x == selected_elements)
                .unwrap();
            linked_storage.remove(index);
        }
        storage.push(selected_elements);
    }
}

// fn set_current_page()
