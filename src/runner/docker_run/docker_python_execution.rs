use std::process::Command;

pub fn run_python_code() -> String {
    let command = Command::new("docker")
        .args(["run", "--rm", "secure-python"])
        .output()
        .expect("ERROR RUNNING COMMAND!");
    return String::from_utf8(command.stdout).unwrap();
}

