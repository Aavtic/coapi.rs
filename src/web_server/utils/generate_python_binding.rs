use crate::axum_serve::ExpectedInputOutput;
use std::process::Command;
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
    title: String,
    description: String,
    question_id: String,
    function_name: String,
    argument_name: String,
    input_output: Vec<ExpectedInputOutput>
}

pub fn bind_gen_python(gen_input: GenInput) {
    let input_json = serde_json::to_string(&gen_input).unwrap();
    let command = Command::new("python3")
        .arg("/test_code/generator_python/generate.py")
        .arg("--details")
        .arg(input_json)
        .output()
        .unwrap();
}
