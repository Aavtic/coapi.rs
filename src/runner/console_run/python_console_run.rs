use std::process::Command;

pub struct CodeOutput {
    pub output: String,
    pub status: i32,
}


pub fn run_python(filename: &str) -> CodeOutput {
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

