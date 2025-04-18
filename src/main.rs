use bak::{backup_file, delete_backup_file};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    /// Add this flag to delete the backup file
    #[arg(short, long)]
    delete: bool,

    /// Specify the filename to backup
    file: PathBuf,
}

fn error_handling(e: std::io::Error) {
    match e.kind() {
        std::io::ErrorKind::NotFound => eprintln!("File not found"),
        std::io::ErrorKind::PermissionDenied => eprintln!("Cannot open file, check permissions"),
        _ => eprintln!("Critical error: {}", e),
    }
    std::process::exit(1);
}

fn main() -> Result<(), std::io::Error> {
    let opts = Opts::parse();

    if opts.delete {
        if let Err(e) = delete_backup_file(&opts.file) {
            error_handling(e);
        }
    } else {
        if let Err(e) = backup_file(&opts.file) {
            error_handling(e);
        }
    }
    Ok(())
}
