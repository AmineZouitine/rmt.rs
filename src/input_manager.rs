use rusqlite::Connection;
use std::io::stdout;
use std::process::exit;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::Stylize;
use crossterm::terminal::{enable_raw_mode, Clear, ClearType, disable_raw_mode};
use crossterm::{cursor, execute};

use crate::{
    data_manager,
    display_manager::{self, DisplayInfos},
    trash_manager,
};

pub fn start_display(connection: &Connection, is_test: bool) {
    enable_raw_mode().unwrap();
    let mut stdout = stdout();
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Hide
    )
    .unwrap();

    let mut display_informations = display_manager::DisplayInfos::new(
        data_manager::get_element_count(connection, is_test)
            .expect("Failed to count all the trash"),
    );

    let mut current_selected_item =
        display_manager::display_trash(connection, is_test, &mut display_informations);

    let mut current_page: usize = display_informations.current_page;
    loop {
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        if display_informations.filter.is_filter {
            match read() {
                Ok(Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind: KeyEventKind::Press,
                    state: _,
                })) => {
                    if code == KeyCode::Char('d') && modifiers == KeyModifiers::CONTROL {
                        display_informations.filter.content.clear();
                    } else if code == KeyCode::Backspace {
                        display_informations.filter.content.pop();
                    } else if code == KeyCode::Esc {
                        display_informations.filter.is_filter = false;
                    } else if let KeyCode::Char(c) = code {
                        if (c as u8) >= 33 && (c as u8) <= 126 {
                            display_informations.filter.content.push(c);
                        }
                    }
                    execute!(stdout, Clear(ClearType::All)).unwrap();
                }
                Err(e) => {
                    execute!(stdout, Clear(ClearType::All)).unwrap();
                    println!("{}: {}", "Error".to_string().red().bold(), e);
                    exit(1)
                }
                _ => (),
            }
        } else {
            match read() {
                Ok(Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind: KeyEventKind::Press,
                    state: _,
                })) => {
                    if code == KeyCode::Char('q')
                        || (code == KeyCode::Char('c') && modifiers == KeyModifiers::CONTROL)
                        || (code == KeyCode::Char('z') && modifiers == KeyModifiers::CONTROL)
                    {
                        execute!(stdout, Clear(ClearType::All)).unwrap();
                        break;
                    }
                    if code == KeyCode::Up || code == KeyCode::Char('k') {
                        set_cursor(&mut display_informations, true);
                    }
                    if code == KeyCode::Down || code == KeyCode::Char('j') {
                        set_cursor(&mut display_informations, false);
                    }
                    if code == KeyCode::Left || code == KeyCode::Char('h') {
                        set_page(&mut display_informations, false);
                    }
                    if code == KeyCode::Right || code == KeyCode::Char('l') {
                        set_page(&mut display_informations, true);
                    }
                    if code == KeyCode::Esc {
                        display_informations.filter.is_filter = true;
                        display_informations.current_cursor_index = 0;
                        display_informations.current_page = 1;
                        execute!(stdout, Clear(ClearType::All)).unwrap();
                    }
                    if code == KeyCode::Char(' ') {
                        toggle_item(
                            current_selected_item,
                            &mut display_informations.selected_trash_items.restore,
                            &mut display_informations.selected_trash_items.delete,
                        );
                    }
                    if code == KeyCode::Delete {
                        toggle_item(
                            current_selected_item,
                            &mut display_informations.selected_trash_items.delete,
                            &mut display_informations.selected_trash_items.restore,
                        );
                    }
                    if code == KeyCode::Enter {
                        execute!(stdout, cursor::Show, Clear(ClearType::All)).unwrap();

                        trash_manager::remove_all_elements_selected(
                            connection,
                            is_test,
                            &display_informations.selected_trash_items.delete,
                        );
                        trash_manager::restore_all_elements_selected(
                            connection,
                            is_test,
                            &display_informations.selected_trash_items.restore,
                        );
                        break;
                    }
                }
                Err(e) => {
                    execute!(stdout, Clear(ClearType::All)).unwrap();
                    println!("{}: {}", "Error".to_string().red().bold(), e);
                    exit(1)
                }
                _ => (),
            }
        }
        if current_page != display_informations.current_page {
            execute!(stdout, Clear(ClearType::All)).unwrap();
            current_page = display_informations.current_page;
        }
        current_selected_item =
            display_manager::display_trash(connection, is_test, &mut display_informations);
    }
    execute!(stdout, cursor::Show).unwrap();
    disable_raw_mode().expect("Unable to disable raw mode");
}

fn set_cursor(display_infos: &mut DisplayInfos, top: bool) {
    if !top
        && display_infos.current_cursor_index + 1
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

fn toggle_item(selected_elements: i32, storage: &mut Vec<i32>, linked_storage: &mut Vec<i32>) {
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

