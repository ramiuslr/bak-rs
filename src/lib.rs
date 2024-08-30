use std::path::Path;
use std::{fs, process};

fn check_file_exists(file_path: &str) -> bool {
    Path::new(file_path).metadata().is_ok()
}

pub fn backup_file(file_name: &str) {
    if check_file_exists(file_name) {
        let bak_name = format!("{}.bak", file_name);
        fs::copy(file_name, &bak_name).expect("Failed to create backup file");
    } else {
        eprintln!("File {} not found", file_name);
        process::exit(1);
    }
}

pub fn delete_backup_file(file_name: &str) {
    let bak_name = format!("{}.bak", file_name);
    if check_file_exists(&bak_name) {
        fs::remove_file(&bak_name).expect("Failed to delete backup file");
    } else {
        eprintln!("File {} not found", bak_name);
        process::exit(1);
    }
}
