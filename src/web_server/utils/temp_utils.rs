use std::process::Command;
use std::fs;


pub fn create_temp_dir(fmt_path: &str) -> std::io::Result<String> {
    let command = Command::new("mktemp")
        .args(["-d", fmt_path])
        .output();
    let output = String::from_utf8(command?.stdout).unwrap();
    Ok(output)
}

pub fn create_temp_file(filepath: &str) {
    println!("creating file {}", filepath);
    fs::File::create(filepath).unwrap();
}
