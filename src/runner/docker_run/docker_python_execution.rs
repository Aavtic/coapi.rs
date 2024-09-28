use std::process::Command;

pub fn run_python_code() -> String {
    let command = Command::new("sudo")
        .args(["docker", "run", "--rm", "secure-python"])
        .output();
    let output = match command {
        Ok(output) => {
            String::from_utf8(output.stdout).unwrap()
        },
        Err(e) => {
            format!("{:?}", e)
        }
    };

    return output;
}

