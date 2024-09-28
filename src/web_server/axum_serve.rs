use axum::{Router, routing::post, extract::Json};
use serde::{Serialize, Deserialize};
//use serde_json::to_string_pretty;
use crate::runner::{
    docker_run::docker_python_execution,
    docker_build::docker_build_python
};


#[derive(Serialize, Deserialize)]
struct CodeRequest {
    code: String,
    language: String,
}


async fn get_code_output(Json(json_request): Json<CodeRequest>) -> String {
    let code = json_request.code;
    std::fs::write("./code/python-code/code.py", code.as_bytes()).expect("ERROR WRITING TO FILE.");
    docker_build_python::docker_build();
    let code_output = docker_python_execution::run_python_code();

    return code_output;
} 


#[tokio::main]
pub async fn code_output_api(addr: &str, path: &str) { 
    let app = Router::new().route(path, post(get_code_output));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
