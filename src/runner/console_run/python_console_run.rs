use std::process::Command;
use std::fs;


const STDIN_FILE: &str = "./code/python-code/stdin";


pub struct CodeOutput {
    pub output: String,
    pub status: i32,
}

fn clean_up() -> Result<(), std::io::Error> {
    fs::File::create(STDIN_FILE).unwrap().set_len(0)
}

pub fn run_python(filename: &str) -> CodeOutput {
    let _ = clean_up();
    let command = Command::new("python3")
        .arg(filename)
        .output()
        .unwrap();

    if command.status.success() {
        let output = String::from_utf8(command.stdout).unwrap();
        let status = command.status;
        match status.code() {
            Some(status) => {
                return CodeOutput {
                    output,
                    status,
                }
            },
            None => {
                return CodeOutput {
                    output,
                    status: -1,
                }
            }
        }

    } else {
        let output = String::from_utf8(command.stderr).unwrap();
        return CodeOutput {
            output,
            status: -1,
        }
    }
}

