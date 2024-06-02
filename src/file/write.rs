use std::fs;
use std::io;
use std::io::Write;

pub fn get_version() -> io::Result<String> {
    let file_path = "VERSION";
    let version = "0.1.0";
    match fs::metadata(file_path) {
        Ok(_) => fs::read_to_string(file_path),
        Err(_) => {
            println!("no VERSION file found, creating one with version {version}");
            let mut file = fs::File::create("VERSION").unwrap();
            file.write_all(version.as_bytes()).unwrap();
            Ok(version.to_string())
        }
    }
}

pub fn write_version(version: &str) -> io::Result<()> {
    let file_path = "VERSION";
    let mut file = fs::File::create(file_path)?;
    file.write_all(version.as_bytes())?;
    println!("File: {file_path} updated to version: {version}\n");
    Ok(())
}
