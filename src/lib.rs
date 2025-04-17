// use sha2::{Digest, Sha256};
use std::{fs, path::Path, process};

// fn hash_file(path: &Path) -> Result<Vec<u8>, std::io::Error> {
//     let mut file = fs::File::open(path)?;
//     let mut hasher = Sha256::new();
//     let mut buffer = [0; 4096];

//     loop {
//         let bytes_read = file.read(&mut buffer)?;
//         if bytes_read == 0 {
//             break;
//         }
//         hasher.update(&buffer[..bytes_read]);
//     }

//     Ok(hasher.finalize().to_vec())
// }

fn check_file_exists(file_path: &str) -> bool {
    Path::new(file_path).metadata().is_ok()
}

// fn compare_hashes(file1: &Path, file2: &Path) -> Result<bool, std::io::Error> {
//     let hash1 = hash_file(file1)?;
//     let hash2 = hash_file(file2)?;
//     Ok(hash1 == hash2)
// }

/// The function which backups the file
/// It erases any existing backup with the same name
pub fn backup_file(file_name: &str) {
    if check_file_exists(file_name) {
        let bak_name = format!("{}.bak", file_name);
        fs::copy(file_name, &bak_name).expect("Failed to create backup file");
    } else {
        eprintln!("File {} not found", file_name);
        process::exit(1);
    }
}

/// A function to delete the backup file
/// Outputs a warning if files changed
pub fn delete_backup_file(file_name: &str) -> Result<(), std::io::Error> {
    let bak_name = format!("{}.bak", file_name);
    if Path::new(&bak_name).try_exists()? {
        fs::remove_file(&bak_name).expect("Failed to delete backup file");
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Backup file {} not found", bak_name),
        ));
    }
    Ok(())
}
