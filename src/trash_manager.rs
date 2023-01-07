use crate::arguments_manager::ArgumentsManager;
use crate::display_manager;
use crate::structure_manager::{self, get_element_path, get_home_directory_path};
use crate::{
    config::Config, data_manager, structure_manager::get_trash_directory_path,
    trash_item::TrashItem,
};
use chrono;
use colored::Colorize;
use fs_extra::dir::{self, get_size};
use rusqlite::Connection;
use sha256;
use std::process::Command;

use chacha20poly1305::{aead::stream, KeyInit, XChaCha20Poly1305};
use rand::{rngs::OsRng, RngCore};
use std::fs::{self, File};
use std::io::{stdout, Read, Write};
use std::path::Path;
use dialoguer::console::Term;

pub fn add_element_to_trash(
    connection: &Connection,
    config: &Config,
    element_path: &str,
    is_test: bool,
    arguments_manager: &ArgumentsManager,
) {
    let element_size = get_size(&element_path).expect("Unable to get element size");

    let mut hash = sha256::digest(format!(
        "{}{}{}",
        &element_path,
        element_size,
        chrono::offset::Local::now().timestamp_nanos()
    ));

    let date = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");

    let mut compression_size: Option<u64> = None;
    let mut is_encrypted = false;
    let element_is_directory = Path::new(&element_path).is_dir();

    if config.compression && !element_is_directory {
        let mut option = String::from("-qrjm");
        if config.encryption {
            option.push('e');
        }
        hash.push_str(".zip");
        let dist_path = format!("{}/{}", get_trash_directory_path(is_test), hash);
        let output = Command::new("zip")
            .args([&option, &dist_path, element_path])
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("Error compressing file");
        if !output.status.success() {
            panic!("Error compressing file");
        }
        if config.encryption {
            // remove lines that ask for password
            // same behaviour as `encrypt_element`
            Term::clear_last_lines(&Term::stdout(), 2).unwrap();
        }
        // zip command automatically appends .zip extension
        compression_size = Some(fs::metadata(&dist_path).unwrap().len());
    } else if config.encryption && !element_is_directory {
        let dist_path = format!("{}/{}", get_trash_directory_path(is_test), hash);
        encrypt_element(element_path, &dist_path).expect("Error encrypting file");
        fs::remove_file(element_path).unwrap();
        is_encrypted = true;
    } else {
        let new_name = format!("{}/{}", get_element_path(element_path), hash);
        fs::rename(&element_path, &new_name).unwrap();
        fs_extra::move_items(
            &[&new_name],
            &get_trash_directory_path(is_test),
            &dir::CopyOptions::new(),
        )
        .unwrap();
    }

    let trash_item = TrashItem::new(
        structure_manager::get_element_name(element_path),
        hash,
        get_element_path(element_path),
        date.to_string(),
        element_size,
        compression_size,
        element_is_directory,
        is_encrypted,
    );
    if !arguments_manager.is_destroy {
        data_manager::insert_trash_item(connection, &trash_item, is_test);
    }

    if arguments_manager.is_verbose {
        println!(
            "this {} {} has been added to the trash.",
            if element_is_directory {
                "directory".bold().white()
            } else {
                "file".bold().white()
            },
            element_path.green().bold()
        );
    }
}

fn encrypt_element(source_path: &str, dist_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encryption_key = dialoguer::Password::new()
        .report(false)
        .with_prompt("Enter password")
        .with_confirmation("Verify password", "Inputs do not match")
        .interact()?;
    let argon2_config = argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);
    // turn insecure passphrase into secure key
    let key = argon2::hash_raw(encryption_key.as_bytes(), &salt, &argon2_config)?;
    let aead = XChaCha20Poly1305::new(key[..].into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce[..].into());
    let mut source_file = File::open(source_path)?;
    let mut dist_file = File::create(dist_path)?;

    // store salt and nonce in the encrypted file to be used when decrypting
    dist_file.write_all(&salt)?;
    dist_file.write_all(&nonce)?;
    // encrypt data in small chunk of 500 bytes
    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    loop {
        let read_count = source_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor.encrypt_next(buffer.as_slice());
            match ciphertext {
                Ok(ciphertext) => dist_file.write(&ciphertext)?,
                Err(_) => {
                    let _ = fs::remove_file(dist_path);
                    panic!("Error encrypting file");
                }
            };
        } else {
            let ciphertext = stream_encryptor.encrypt_last(&buffer[..read_count]);
            match ciphertext {
                Ok(ciphertext) => dist_file.write(&ciphertext)?,
                Err(_) => {
                    let _ = fs::remove_file(dist_path);
                    panic!("Error encrypting file");
                }
            };
            break;
        }
    }
    Ok(())
}

fn decrypt_element(
    encrypted_path: &str,
    dist_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];

    let mut encrypted_file = File::open(encrypted_path)?;
    let mut dist_file = File::create(dist_path)?;

    let mut read_count = encrypted_file.read(&mut salt)?;
    if read_count != salt.len() {
        panic!("Error reading salt.");
    }
    read_count = encrypted_file.read(&mut nonce)?;
    if read_count != nonce.len() {
        panic!("Error reading nonce.");
    }

    let argon2_config = argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };
    let encryption_key: String = dialoguer::Password::new()
        .with_prompt("Enter password")
        .interact()?;
    let key = argon2::hash_raw(encryption_key.as_bytes(), &salt, &argon2_config).unwrap();

    let aead = XChaCha20Poly1305::new(key[..32].into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce[..].into());
    const BUFFER_LEN: usize = 500 + 16;
    let mut buffer = [0u8; BUFFER_LEN];

    loop {
        let read_count = encrypted_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let plaintext = stream_decryptor.decrypt_next(buffer.as_slice());
            match plaintext {
                Ok(plaintext) => dist_file.write(&plaintext)?,
                Err(_) => {
                    let _ = fs::remove_file(dist_path);
                    panic!("Error decrypting file");
                }
            };
        } else {
            let plaintext = stream_decryptor.decrypt_last(&buffer[..read_count]);
            match plaintext {
                Ok(plaintext) => dist_file.write(&plaintext)?,
                Err(_) => {
                    let _ = fs::remove_file(dist_path);
                    panic!("Error decrypting file");
                }
            };
            break;
        }
    }
    Ok(())
}

pub fn add_all_elements_to_trash(
    connection: &Connection,
    config: &Config,
    element_paths: &[String],
    is_test: bool,
    arguments_manager: &ArgumentsManager,
) {
    if arguments_manager.confirmation_once && element_paths.len() > 3 {
        let message = format!(
            "Sure you want to delete all {} files ?",
            element_paths.len().to_string().bold().green()
        );
        if !display_manager::get_user_validation(&message) {
            return;
        }
    }
    for path in element_paths {
        let message = format!("Are you sure to delete {} ?", path.bold().green());
        if !arguments_manager.confirmation_always || display_manager::get_user_validation(&message)
        {
            add_element_to_trash(connection, config, path, is_test, arguments_manager)
        }
    }
}

pub fn remove_all_elements_selected(
    connection: &Connection,
    is_test: bool,
    trash_items_ids: &[i32],
) {
    trash_items_ids.iter().for_each(|trash_item_id| {
        let trash_item = data_manager::find_trash_item_by_id(connection, is_test, *trash_item_id);
        remove_element(&trash_item, is_test);
        data_manager::delete_trash_item_by_id(connection, is_test, *trash_item_id);
    });
}

pub fn remove_all_elements(connection: &Connection, is_test: bool) {
    let trash_items = data_manager::find_all_trash_items(connection, is_test);
    trash_items.iter().for_each(|trash_item| {
        remove_element(trash_item, is_test);
    });
    data_manager::delete_all_trash_item(connection, is_test);
}

fn remove_element(trash_item: &TrashItem, is_test: bool) {
    let element_path = format!("{}/{}", get_trash_directory_path(is_test), trash_item.hash);
    if Path::new(&element_path).is_dir() {
        fs::remove_dir_all(&element_path).unwrap();
    } else {
        fs::remove_file(&element_path).unwrap();
    }

    println!(
        "{} {}\r",
        trash_item.name.red().bold(),
        "deleted !".red().bold()
    );
}

pub fn restore_all_elements_selected(
    connection: &Connection,
    is_test: bool,
    trash_items_ids: &[i32],
) {
    trash_items_ids.iter().for_each(|trash_item_id| {
        let trash_item = data_manager::find_trash_item_by_id(connection, is_test, *trash_item_id);
        restore_element(&trash_item, is_test);
        data_manager::delete_trash_item_by_id(connection, is_test, *trash_item_id);
    });
}

fn restore_element(trash_item: &TrashItem, is_test: bool) {
    let path_in_trash = format!("{}/{}", get_trash_directory_path(is_test), trash_item.hash);
    let element_path_name = format!("{}/{}", &trash_item.path, &trash_item.name);

    if Path::new(&trash_item.path).is_dir() && !Path::new(&element_path_name).exists() {
        if trash_item.compression_size.is_some() {
            let output = Command::new("unzip")
                .args([&path_in_trash, "-d", &trash_item.path])
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::inherit())
                .output()
                .expect("Error extracting file");
            if !output.status.success() {
                panic!("Error extracting file");
            }
            fs::remove_file(&path_in_trash).unwrap();
        } else if trash_item.is_encrypted {
            let dist_path = format!("{}/{}", &trash_item.path, trash_item.name);
            decrypt_element(&path_in_trash, &dist_path).expect("Error decrypting file");
            fs::remove_file(&path_in_trash).unwrap();
        } else {
            let element_path_renamed =
                format!("{}/{}", get_trash_directory_path(is_test), trash_item.name);
            fs::rename(&path_in_trash, &element_path_renamed).unwrap();
            fs_extra::move_items(
                &[&element_path_renamed],
                &trash_item.path,
                &dir::CopyOptions::new(),
            ).unwrap();
        }
        println!(
            "{} has been restored ! :D\r",
            trash_item.name.green().bold()
        );
        println!(
            "You can find it at this path: {}\r",
            element_path_name.green().bold()
        );
        return;
    }
    println!("Unfortunately Path {} doesn't exist anymore or there is a file with the same name inside, so we can't restore your element to the original path :c\r\n{}\r",
     &trash_item.path.green().bold(), "Please enter a new absolute path to restore your element".bold());

    let mut new_path = get_home_directory_path();
    print!("{} {}", ">>".green().bold(), new_path.bold());
    stdout().flush().unwrap();
    std::io::stdin().read_line(&mut new_path).unwrap();
    new_path.pop();
    while !Path::new(&new_path).is_dir()
        || Path::new(&format!("{}/{}", &new_path, &trash_item.name)).exists()
    {
        if !Path::new(&new_path).exists() {
            println!(
                "{} doesn't exist ! You have to give a valid {} path of a {}\r",
                new_path.green().bold(),
                "absolute path".green().bold(),
                "directory".green().bold()
            );
        } else if !Path::new(&new_path).is_dir() {
            println!(
                "{} exist but it's not a {} ! \r",
                new_path.green().bold(),
                "directory".green().bold()
            );
        } else {
            println!(
                "{} exist and it's a {}, but it's already contain a element with the same name {}!\r",
                new_path.green().bold(),
                "directory".green().bold(),
                trash_item.name.green().bold()
            );
        }
        new_path.clear();
        new_path = get_home_directory_path();
        print!("{} {}", ">>".green().bold(), new_path.bold());
        stdout().flush().unwrap();
        std::io::stdin().read_line(&mut new_path).unwrap();
        new_path.pop();
    }

    if trash_item.compression_size.is_some() {
        let output = Command::new("unzip")
            .args([&path_in_trash, "-d", &trash_item.path])
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::inherit())
            .output()
            .expect("Error extracting file");
        if !output.status.success() {
            panic!("Error extracting file");
        }
        fs::remove_file(&path_in_trash).unwrap();
    } else if trash_item.is_encrypted {
        let dist_path = format!("{}/{}", &new_path, trash_item.name);
        decrypt_element(&path_in_trash, &dist_path).expect("Error decrypting file");
        fs::remove_file(&path_in_trash).unwrap();
    } else {
        let new_name = format!("{}/{}", get_trash_directory_path(is_test), trash_item.name);
        fs::rename(&path_in_trash, &new_name).unwrap();
        fs_extra::move_items(&[new_name], &new_path, &dir::CopyOptions::new()).unwrap();
    }
    println!(
        "{} has been restored ! :D\r",
        trash_item.name.green().bold()
    );
    if !new_path.is_empty() && new_path.as_bytes()[new_path.len() - 1] as char == '/' {
        new_path.pop();
    }
    println!(
        "You can find it at this path: {}\r",
        format!("{}/{}", &new_path, &trash_item.name).green().bold()
    );
}

pub fn display_trash_information(connection: &Connection, is_test: bool) {
    let trash_items = data_manager::find_all_trash_items(connection, is_test);
    let total_size = if let Some(size) = trash_items
        .iter()
        .map(|trash_item| trash_item.real_size)
        .reduce(|a, b| a + b)
    {
        size
    } else {
        0
    };
    println!(
        "{} elements are stored in the trash.",
        trash_items.len().to_string().green().bold()
    );
    println!(
        "{} {} is the total size of the trash.",
        (total_size / 1000).to_string().green().bold(),
        "ko".bold().white()
    )
}

#[cfg(test)]
mod tests {}
