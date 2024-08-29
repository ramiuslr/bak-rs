use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    /// Add this flag to delete the backup file
    #[arg(short, long)]
    delete: bool,

    /// Specify the filename to backup
    filename: PathBuf,
}

fn backup_file(file_name: &str) {
    let bak_name = format!("{}.bak", file_name);
    fs::copy(file_name, &bak_name).expect("Failed to create backup file");
}

fn delete_backup_file(file_name: &str) {
    let bak_name = format!("{}.bak", file_name);
    fs::remove_file(&bak_name).expect("Failed to delete backup file");
}

fn main() {
    let opts = Opts::parse();

    if opts.delete {
        delete_backup_file(opts.filename.to_str().unwrap());
    } else {
        backup_file(opts.filename.to_str().unwrap());
    }
}
