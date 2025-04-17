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
    filename: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let opts = Opts::parse();

    if opts.delete {
        if let Err(e) = delete_backup_file(opts.filename.to_str().unwrap()) {
            match e.kind() {
                std::io::ErrorKind::NotFound => eprintln!("File not found"),
                _ => eprintln!("Critical error: {}", e),
            }
        }
    } else {
        backup_file(opts.filename.to_str().unwrap());
    }
    Ok(())
}
