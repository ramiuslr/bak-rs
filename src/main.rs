use clap::Parser;
use std::path::{Path, PathBuf};
use std::{fs, process};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    /// Add this flag to delete the backup file
    #[arg(short, long)]
    delete: bool,

    /// Specify the filename to backup
    filename: PathBuf,
}

fn check_file_exists(file_path: &str) -> bool {
    match Path::new(file_path).metadata() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn backup_file(file_name: &str) {
    if check_file_exists(file_name) {
        let bak_name = format!("{}.bak", file_name);
        fs::copy(file_name, &bak_name).expect("Failed to create backup file");
    } else {
        eprintln!("File {} not found", file_name);
        process::exit(1);
    }
}

fn delete_backup_file(file_name: &str) {
    let bak_name = format!("{}.bak", file_name);
    if check_file_exists(&bak_name) {
        fs::remove_file(&bak_name).expect("Failed to delete backup file");
    } else {
        eprintln!("File {} not found", bak_name);
        process::exit(1);
    }
}

fn main() {
    let opts = Opts::parse();

    if opts.delete {
        delete_backup_file(opts.filename.to_str().unwrap());
    } else {
        backup_file(opts.filename.to_str().unwrap());
    }
}
