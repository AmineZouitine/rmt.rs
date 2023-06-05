use crate::arguments_manager::ArgumentsManager;
use crate::display_manager;
use crate::structure_manager::{self, get_element_path, get_home_directory_path};
use crate::{
    config::Config, data_manager, structure_manager::get_trash_directory_path,
    trash_item::TrashItem,
};

use chacha20poly1305::{aead::stream, KeyInit, XChaCha20Poly1305};
use chrono;
use colored::Colorize;
use fs_extra::dir::{self, get_size};
use rusqlite::Connection;
use sha256;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

use rand::{rngs::OsRng, RngCore};
use std::fs::{self, File};
use std::io::{copy, stdout, Read, Write};
use std::path::{Path, MAIN_SEPARATOR};

pub fn add_element_to_trash(
    connection: &Connection,
    config: &Config,
    element_path: &str,
    arguments_manager: &ArgumentsManager,
) {
    let element_size = get_size(&element_path).expect("Unable to get element size");

    let hash = sha256::digest(format!(
        "{}{}{}",
        &element_path,
        element_size,
        chrono::offset::Local::now().timestamp_nanos()
    ));

    let date = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");

    let mut compression_size: Option<u64> = None;
    let mut is_compressed = false;
    let mut is_encrypted = false;

    let element_is_directory = Path::new(&element_path).is_dir();

    if config.compression {
        let compressed_path = format!("{}.zip", element_path);
        compress_element(&element_path, &compressed_path).expect("Failed to compress");
        compression_size =
            Some(get_size(&compressed_path).expect("Unable to get compressed element size"));
        is_compressed = true;

        if config.encryption {
            encrypt_element(
                &compressed_path,
                &format!(
                    "{}{}{}",
                    get_trash_directory_path(arguments_manager.is_test),
                    MAIN_SEPARATOR,
                    hash
                ),
            )
            .expect("Failed to encrypt");
            fs::remove_file(&compressed_path).unwrap();
            is_encrypted = true;
        } else {
            let new_name = format!(
                "{}{}{}",
                get_element_path(element_path),
                MAIN_SEPARATOR,
                hash
            );
            fs::rename(&compressed_path, &new_name).unwrap();
            fs_extra::move_items(
                &[&new_name],
                &get_trash_directory_path(arguments_manager.is_test),
                &dir::CopyOptions::new(),
            )
            .unwrap();
        }
        if !element_is_directory {
            fs::remove_file(element_path).unwrap();
        } else {
            fs::remove_dir_all(element_path).unwrap();
        }
    } else if config.encryption && !element_is_directory {
        encrypt_element(
            element_path,
            &format!(
                "{}{}{}",
                get_trash_directory_path(arguments_manager.is_test),
                MAIN_SEPARATOR,
                hash
            ),
        )
        .expect("Failed to encrypt");
        is_encrypted = true;
        fs::remove_file(element_path).unwrap();
    } else {
        let new_name = format!(
            "{}{}{}",
            get_element_path(element_path),
            MAIN_SEPARATOR,
            hash
        );
        fs::rename(&element_path, &new_name).unwrap();
        fs_extra::move_items(
            &[&new_name],
            &get_trash_directory_path(arguments_manager.is_test),
            &dir::CopyOptions::new(),
        )
        .unwrap();
    };

    let trash_item = TrashItem::new(
        structure_manager::get_element_name(element_path),
        hash,
        get_element_path(element_path),
        date.to_string(),
        element_size,
        compression_size,
        element_is_directory,
        is_encrypted,
        is_compressed,
    );
    if !arguments_manager.is_destroy {
        data_manager::insert_trash_item(connection, &trash_item, arguments_manager.is_test);
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

fn compress_element(source_path: &str, dist_path: &str) -> Result<(), std::io::Error> {
    let mut zip_wtr = ZipWriter::new(File::create(dist_path)?);
    let zip_opts = FileOptions::default();

    let mut buffer = Vec::new();
    let base_path = Path::new(source_path);
    if base_path.is_file() {
        zip_wtr.start_file(base_path.file_name().unwrap().to_string_lossy(), zip_opts)?;
        let mut f = File::open(source_path)?;
        f.read_to_end(&mut buffer)?;
        zip_wtr.write_all(&buffer)?;
        zip_wtr.flush()?;
        zip_wtr.finish()?;
        Ok(())
    } else {
        zip_wtr.add_directory(base_path.file_name().unwrap().to_string_lossy(), zip_opts)?;
        for entry in WalkDir::new(source_path).into_iter().filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            let entry_name = entry_path.strip_prefix(base_path).unwrap();

            if entry_path.is_file() {
                zip_wtr.start_file(entry_name.to_string_lossy(), zip_opts)?;
                let mut f = File::open(entry_path)?;
                f.read_to_end(&mut buffer)?;
                zip_wtr.write_all(&buffer)?;
                buffer.clear();
            } else if !entry_name.as_os_str().is_empty() {
                zip_wtr.add_directory(entry_name.to_string_lossy(), zip_opts)?;
            }
        }
        zip_wtr.flush()?;
        zip_wtr.finish()?;
        Ok(())
    }
}

fn decompress_element(compressed_path: &str, dist_path: &str) -> Result<(), std::io::Error> {
    let mut zip_arc = ZipArchive::new(File::open(compressed_path)?)?;

    let mut base_dir = dist_path.to_string();
    for i in 0..zip_arc.len() {
        let mut f = zip_arc.by_index(i)?;
        let entry_name = f
            .enclosed_name()
            .ok_or(zip::result::ZipError::InvalidArchive("Invalid file path"))
            .expect("Failed to get extract path");

        let output_path = Path::new(&base_dir).join(entry_name);
        if i == 0 {
            base_dir = format!(
                "{}{}{}",
                dist_path.clone(),
                MAIN_SEPARATOR,
                entry_name.file_name().unwrap().to_string_lossy(),
            )
        };

        if f.name().ends_with("/") {
            fs::create_dir_all(&output_path)?;
        } else {
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut output_f = File::create(output_path)?;
            copy(&mut f, &mut output_f)?;
        };

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = f.unix_mode() {
                fs::set_permissions(&output_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    Ok(())
}

fn encrypt_element(source_path: &str, dist_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encryption_key = dialoguer::Password::new()
        .with_prompt("Encryption key")
        .with_confirmation("Confirm encryption key", "Inputs do not match")
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
    let encryption_key = dialoguer::Password::new()
        .with_prompt("Encryption key")
        .with_confirmation("Confirm encryption key", "Inputs do not match")
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
            add_element_to_trash(connection, config, path, arguments_manager)
        }
    }
}

pub fn remove_all_elements_selected(
    connection: &Connection,
    is_test: bool,
    trash_items_ids: &[i32],
) {
    trash_items_ids.iter().for_each(|trash_item_id| {
        let trash_item = data_manager::find_trash_item_by_id(connection, is_test, *trash_item_id)
            .expect(&format!("Failed to get item with id {}", &trash_item_id));
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
    let element_path = format!(
        "{}{}{}",
        get_trash_directory_path(is_test),
        MAIN_SEPARATOR,
        trash_item.hash
    );
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
        let trash_item = data_manager::find_trash_item_by_id(connection, is_test, *trash_item_id)
            .expect(&format!("Failed to get item with id {}", &trash_item_id));
        restore_element(&trash_item, is_test);
        data_manager::delete_trash_item_by_id(connection, is_test, *trash_item_id);
    });
}

fn restore_element(trash_item: &TrashItem, is_test: bool) {
    let path_in_trash = format!(
        "{}{}{}",
        get_trash_directory_path(is_test),
        MAIN_SEPARATOR,
        trash_item.hash
    );

    let initial_path = format!("{}{}{}", &trash_item.path, MAIN_SEPARATOR, &trash_item.name);

    if Path::new(&trash_item.path).is_dir() && !Path::new(&initial_path).exists() {
        if trash_item.is_encrypted && trash_item.is_compressed {
            let decrypted_path_in_trash = format!(
                "{}{}decrypted.zip",
                get_trash_directory_path(is_test),
                MAIN_SEPARATOR
            );
            decrypt_element(&path_in_trash, &decrypted_path_in_trash).expect("Failed to decrypt");
            decompress_element(&decrypted_path_in_trash, &trash_item.path)
                .expect("Failed to decompress");
            fs::remove_file(decrypted_path_in_trash).unwrap();
        } else if trash_item.is_encrypted {
            decrypt_element(
                &path_in_trash,
                &format!("{}{}{}", &trash_item.path, MAIN_SEPARATOR, trash_item.name),
            )
            .expect("Failed to decrypt");
            fs::remove_file(&path_in_trash).unwrap();
        } else if trash_item.is_compressed {
            decompress_element(&path_in_trash, &trash_item.path).expect("Failed to decompress");
            fs::remove_file(&path_in_trash).unwrap();
        } else {
            let renamed_path_in_trash = format!(
                "{}{}{}",
                get_trash_directory_path(is_test),
                MAIN_SEPARATOR,
                trash_item.name
            );
            fs::rename(&path_in_trash, &renamed_path_in_trash).unwrap();
            fs_extra::move_items(
                &[&renamed_path_in_trash],
                &trash_item.path,
                &dir::CopyOptions::new(),
            )
            .unwrap();
        };
        println!(
            "{} has been restored ! :D\r",
            trash_item.name.green().bold()
        );
        println!(
            "You can find it at this path: {}\r",
            initial_path.green().bold()
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
        || Path::new(&format!(
            "{}{}{}",
            &new_path, MAIN_SEPARATOR, &trash_item.name
        ))
        .exists()
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

    if trash_item.is_encrypted {
        let dist_path = format!("{}{}{}", &new_path, MAIN_SEPARATOR, trash_item.name);
        decrypt_element(&path_in_trash, &dist_path).expect("Error decrypting file");
        fs::remove_file(&path_in_trash).unwrap();
    } else {
        let new_name = format!(
            "{}{}{}",
            get_trash_directory_path(is_test),
            MAIN_SEPARATOR,
            trash_item.name
        );
        fs::rename(&path_in_trash, &new_name).unwrap();
        fs_extra::move_items(&[new_name], &new_path, &dir::CopyOptions::new()).unwrap();
    }
    println!(
        "{} has been restored ! :D\r",
        trash_item.name.green().bold()
    );
    if !new_path.is_empty() && new_path.as_bytes()[new_path.len() - 1] as char == MAIN_SEPARATOR {
        new_path.pop();
    }
    println!(
        "You can find it at this path: {}\r",
        format!("{}{}{}", &new_path, MAIN_SEPARATOR, &trash_item.name)
            .green()
            .bold()
    );
}

pub fn display_trash_information(connection: &Connection, is_test: bool) {
    let trash_items = data_manager::find_all_trash_items(connection, is_test);
    let total_size = if let Some(size) = trash_items
        .iter()
        .map(|trash_item| {
            if trash_item.is_compressed {
                trash_item.compression_size.unwrap_or(trash_item.real_size)
            } else {
                trash_item.real_size
            }
        })
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
