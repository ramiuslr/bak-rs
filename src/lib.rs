// use sha2::{Digest, Sha256};
use std::path::Path;

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

// fn compare_hashes(file1: &Path, file2: &Path) -> Result<bool, std::io::Error> {
//     let hash1 = hash_file(file1)?;
//     let hash2 = hash_file(file2)?;
//     Ok(hash1 == hash2)
// }

/// The function which backups the file
/// It erases any existing backup with the same name
pub fn backup_file(file: &Path) -> Result<(), std::io::Error> {
    let mut bak_name = file.to_path_buf();
    bak_name.set_extension("bak");
    if let Err(e) = std::fs::copy(file, bak_name) {
        return Err(e);
    } else {
        return Ok(());
    }
}

/// A function to delete the backup file
pub fn delete_backup_file(file: &Path) -> Result<(), std::io::Error> {
    let mut bak_name = file.to_path_buf();
    bak_name.set_extension("bak");
    if let Err(e) = std::fs::remove_file(&bak_name) {
        Err(e)
    } else {
        Ok(())
    }
}
