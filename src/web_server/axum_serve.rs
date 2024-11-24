use axum::{Router, routing::{get, post,options},
    extract::{Json, Path},
    response::{Response, Html},
    http::StatusCode, body::Body};
use serde::{Serialize, Deserialize};
use tower_http::services::ServeDir;
use tower_http::cors::{CorsLayer, Any};
use http::Method;
// use mongodb::bson::{doc, Document};
// use mongodb::Client;
// use serde_json::to_string_pretty;

use crate::runner::console_run::python_console_run;
use crate::web_server::database::mongo_funcs;
use crate::web_server::ssrenderer::ssrenderer;
use crate::web_server::utils::temp_utils;



static DATABASE_NAME: &str = "coapidb";
static QUESTIONS_COLLECTION_NAME: &str = "questions";


#[derive(Serialize, Deserialize)]
struct CodeRequest {
    code: String,
    language: String,
}

#[derive(Serialize)]
struct CodeResponse {
    output: String,
    status: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpectedInputOutput {
    input: String,
    output: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddQuestion {
    pub title: String,
    pub description: String,
    pub data: Vec<ExpectedInputOutput>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetQuestion {
    question_id: String,
}

async fn get_live_code_output(Json(json_request): Json<CodeRequest>) -> Response {
    println!("received request: {:?}", serde_json::to_string_pretty(&json_request).unwrap());
    let code = json_request.code;
    let temp_folder_format = "./live-code/pyenv-XXXX";
    let temp_folder = temp_utils::create_temp_dir(temp_folder_format).unwrap().trim().to_string();
    let file_path = &(temp_folder.clone() + "/main.py");
    temp_utils::create_temp_file(&(temp_folder + "/main.py"));
    std::fs::write(file_path, code.as_bytes()).expect("ERROR WRITING TO FILE.");
    // let proc_output = python_console_run::run_python(filename);
    // let code_output = CodeResponse{output: proc_output.output, status: proc_output.status};
    // docker_build_python::docker_build();
    // let code_output = docker_python_execution::run_python_code();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin","*")
        .header("Access-Control-Allow-Methods", "POST")
        .header("Access-Control-Allow-Headers","Content-Type")
        .body(Body::from(serde_json::to_string("AOK").unwrap()))
        .unwrap();
    println!("RESPONSE SENT!");

    return response;
} 

async fn serve_question(Path(question_id): Path<String>) -> Html <String> {
    let client = mongo_funcs::connect("mongodb://localhost:27017").await;
    if let Some(question) = mongo_funcs::get_question(&client, DATABASE_NAME, QUESTIONS_COLLECTION_NAME, question_id).await {
        let ren_html = ssrenderer::generate_question_html(question);
        return ren_html;
    } else {
        return ssrenderer::error_page();
    }
}

async fn serve_questions() -> Html<String> {
    let client = mongo_funcs::connect("mongodb://localhost:27017").await;
    let questions = mongo_funcs::get_all_questions(&client, DATABASE_NAME, QUESTIONS_COLLECTION_NAME).await;
    let ren_html = ssrenderer::generate_questions_html(questions);
    return ren_html;
}


async fn insert_question(Json(question_request): Json<AddQuestion>) -> Response {
    println!("got question request: {:?}", serde_json::to_string_pretty(&question_request));
    let title =  &question_request.title;
    let description = &question_request.description;
    let data = &question_request.data;

    let client = mongo_funcs::connect("mongodb://localhost:27017").await;

    mongo_funcs::insert_document(&client, DATABASE_NAME, QUESTIONS_COLLECTION_NAME, &question_request).await;

    println!("{}\n{}\n{:?}", title, description, data);
    println!("Database updated!");

    return Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Database Updated"))
        .unwrap();
}


async fn get_code_output(Json(json_request): Json<CodeRequest>) -> Response {
    let code = json_request.code;
    let filename = "./code/python-code/code.py";
    std::fs::create_dir_all("./code/python-code").unwrap();
    std::fs::write(filename, code.as_bytes()).expect("ERROR WRITING TO FILE.");
    let proc_output = python_console_run::run_python(filename);
    let code_output = CodeResponse{output: proc_output.output, status: proc_output.status};
    // docker_build_python::docker_build();
    // let code_output = docker_python_execution::run_python_code();

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin","*")
        .header("Access-Control-Allow-Methods", "POST")
        .header("Access-Control-Allow-Headers","Content-Type")
        .body(Body::from(serde_json::to_string(&code_output).unwrap()))
        .unwrap();
    println!("RESPONSE SENT!");

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
    println!("PREFLIGHT SENT!");
    return response;
}

#[tokio::main]
pub async fn code_output_api(addr: &str) { 
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow requests from any origin, for development purposes
        .allow_methods([Method::POST, Method::OPTIONS]);

    let api_routes = Router::new()
        .route("/v1", post(get_code_output))
        .route("/v1", options(preflight_response))
        .route("/v1/create_question", post(insert_question))
        .route("/v1/get_questions", get(serve_questions))
        .route("/v1/get_live_output", post(get_live_code_output));

    let page_routes = Router::new()
        .route("/question/:id", get(serve_question));


    let app = Router::new()
        .nest_service("/", ServeDir::new("coapi-frontend"))
        .nest("/api", api_routes)
        .nest("/pages", page_routes)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
