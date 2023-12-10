use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::{env, fs, path::PathBuf};

pub fn read_binary_file(file_path: &Path, data: &mut Vec<u8>) -> io::Result<()> {
    let mut file: File = File::open(file_path)?;
    file.read_to_end(data)?;
    Ok(())
}

pub fn write_binary_file(file_path: &Path, data: &[u8; 16]) -> io::Result<()> {
    let mut file: File = File::create(file_path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn generate_binary_file() -> io::Result<()> {
    let current_dir: PathBuf = env::current_dir()?;
    let relative_path: PathBuf = PathBuf::from("data/binary/example.bin");
    let file_path: PathBuf = current_dir.join(relative_path);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let binary_data: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    write_binary_file(&file_path, &binary_data)?;

    println!(
        "Binary file '{}' generated successfully.",
        file_path.display()
    );

    Ok(())
}
