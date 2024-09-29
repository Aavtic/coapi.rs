use axum::{Router, routing::{post,options}, extract::Json, response::Response, http::StatusCode, body::Body};
use serde::{Serialize, Deserialize};
//use serde_json::to_string_pretty;
use crate::runner::console_run::python_console_run;


#[derive(Serialize, Deserialize)]
struct CodeRequest {
    code: String,
    language: String,
}


async fn get_code_output(Json(json_request): Json<CodeRequest>) -> Response {
    let code = json_request.code;
    let filename = "./code/python-code/code.py";
    std::fs::create_dir_all("./code/python-code").unwrap();
    std::fs::write(filename, code.as_bytes()).expect("ERROR WRITING TO FILE.");
    let code_output = python_console_run::run_python(filename);
    // docker_build_python::docker_build();
    // let code_output = docker_python_execution::run_python_code();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin","*")
        .header("Access-Control-Allow-Methods", "POST")
        .header("Access-Control-Allow-Headers","Content-Type")
        .body(Body::from(code_output))
        .unwrap();

    return response;
} 


async fn preflight_response() -> Response {
    let response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(Body::default())
        .unwrap();
    return response;
}

#[tokio::main]
pub async fn code_output_api(addr: &str, path: &str) { 
    let app = Router::new()
        .route(path, post(get_code_output))
        .route(path, options(preflight_response));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
