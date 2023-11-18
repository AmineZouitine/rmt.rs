use std::fs::{self, File};
use std::io::{self, Write};
use rand::{Rng, thread_rng};
use rayon::prelude::*;

pub fn delete_file(file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let metadata = fs::metadata(file_path)?;
    let file_size = metadata.len() as usize;
    let mut rng = thread_rng();
    let random_bytes: Vec<u8> = (0..file_size).map(|_| rng.gen()).collect();
    file.write_all(&random_bytes)?;
    file.sync_all()?;
    fs::remove_file(file_path)?;
    Ok(())
}

pub fn delete_folder(folder_path: &str) -> io::Result<()> {
    let result: io::Result<()> = fs::read_dir(folder_path)?
    .par_bridge()
    .into_par_iter()
    .try_for_each(|entry| {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            delete_file(path.to_str().unwrap())?;
        } else {
            delete_file(path.to_str().unwrap())?;
        }
        Ok(())
     }
    );
    result?;
    fs::remove_dir(folder_path)?;
    Ok(())
}