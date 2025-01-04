use std::process::Command;
use serde_json;
use crate::web_server::utils::communications::{IPCStatus, Status};

const RUNNER_PATH: &str = "./test_code/generator_python/runner.py";

pub fn test_python3(question_id: String, filepath: String) -> IPCStatus {
    let question_details = format!(r#"{{"filepath": "{}", "question_id": "{}"}}"#, filepath, question_id);
    println!("Input JSON: {}", question_details);
    // {"filepath": ./live-code/pyenv-JnLy/main.py, "question_id": 00b6c7df-f861-4378-b9d9-112cfe1a935e}

    let command = Command::new("python3")
        .arg(RUNNER_PATH)
        .arg("--question_details")
        .arg(question_details)
        .output()
        .unwrap();

    let output = String::from_utf8(command.stdout).unwrap();
    if command.status.success() {
        // parse the output
        let parsed_status = serde_json::from_str::<IPCStatus>(&output);
        if let Ok(status) = parsed_status {
            return status;
        } else {
            eprintln!("ERROR: UNABLE TO PARSE: \n{output}, len: {}, {:?}", output.len(), parsed_status.err());
            return IPCStatus{status: Status::Cooked};
        }
    } else {
        let output_err = String::from_utf8(command.stderr).unwrap();
        eprintln!("ERROR: {output_err}");
        return IPCStatus{status: Status::Cooked};
    }
}

