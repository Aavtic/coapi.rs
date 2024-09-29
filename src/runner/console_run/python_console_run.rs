use std::process::Command;


pub fn run_python(filename: &str) -> String {
    let command = Command::new("python3")
        .arg(filename)
        .output()
        .unwrap();

    return String::from_utf8(command.stdout).unwrap();
}


