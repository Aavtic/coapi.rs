use crate::axum_serve::ExpectedInputOutput;
use std::process::Command;
use std::io::ErrorKind;
use serde::{Serialize, Deserialize};
use serde_json;

//{
//         "title": "Factorial",
//         "description": "Factorial is ..",
//         "question_id": id,
//         "function_name": "factorial",
//         "argument_name": "n",
//         "input_output": {
//             "5": 120,
//             "3": 6,
//         },
//         "input_type": "int",
//         "output_type": "int",
//}


#[derive(Serialize, Deserialize, Debug)]
pub struct GenInput {
    pub title: String,
    pub description: String,
    pub question_id: String,
    pub function_name: String,
    pub argument_name: String,
    pub input_type: String,
    pub output_type: String,
    pub input_output: Vec<ExpectedInputOutput>
}

pub fn bind_gen_python(gen_input: GenInput) -> Result<String, std::io::ErrorKind> {
    let input_json = serde_json::to_string(&gen_input).unwrap();
    println!("{}", input_json);
    let command = Command::new("python3")
        .arg("./test_code/generator_python/generate.py")
        .arg("--details")
        .arg(input_json + "\n")
        .output()
        .unwrap();
    let output = String::from_utf8(command.stdout).unwrap();
    let output_err = String::from_utf8(command.stderr).unwrap();
    println!("Output: {}, {}", output, output_err);
    if command.status.success() {
        return Ok(output);
    } else {
        return Err(ErrorKind::NotFound);
    }
}
