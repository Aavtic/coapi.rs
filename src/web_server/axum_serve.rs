use axum::{Router, routing::{get, post,options},
    extract::{Json, Path},
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    response::{Response, Html},
    http::StatusCode, body::Body};
use serde::{Serialize, Deserialize};
use tower_http::services::ServeDir;
use tower_http::cors::{CorsLayer, Any};
use http::Method;
use std::sync::mpsc::{channel, Sender};
use std::io::Write;
use tokio::{sync::mpsc, spawn};
use futures::{SinkExt, StreamExt, TryFutureExt};

// use mongodb::bson::{doc, Document};
// use std::sync::mpsc::channel;
// use mongodb::Client;
// use serde_json::to_string_pretty;

use crate::runner::console_run::python_console_run;
use crate::web_server::database::mongo_funcs;
use crate::web_server::ssrenderer::ssrenderer;
use crate::web_server::utils::temp_utils;

use crate::poller::poller;
use poller::StdinData;
use poller::StdinDataStatus;


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

fn poll_live_output(filename: String, sender: Sender<StdinData>) {
    let (tx, rx) = channel();

    let t1 = std::thread::spawn(move || {
        let _ = poller::live_read_stdin("python3".to_string(), vec![filename], tx);
    });
    // std::io::stdout().lock();
 
    while let Ok(stdin_data) = rx.recv() {
        match stdin_data {
            StdinData::Available(char) => {
                print!("{}", String::from_utf8(vec![char]).unwrap());
                sender.send(StdinData::Available(char)).unwrap();
                std::io::stdout().flush().unwrap();
            },
            StdinData::StdinSender(poll_sender) => {
                let sender_clone = sender.clone();
                std::thread::spawn(move || {
                    let (transmitter, receiver) = channel();
                    sender_clone.send(StdinData::StdinSender(transmitter.clone())).unwrap();
                    
                    while let Ok(data) = receiver.recv() {
                        println!("received data");
                        if poll_sender.send(data).is_err() {
                            let _ = sender_clone.send(StdinData::Over);
                            return;
                        }
                    }
                });
            },
            StdinData::Over => {
                println!("Execution Complete");
                let _ = sender.send(StdinData::Over);
                return;
            },
         }
     }
     t1.join().unwrap();
}

async fn live_code_ws_handler(socket: WebSocketUpgrade) -> Response {
    socket.on_upgrade(live_code_ws)
}

async fn live_code_ws(socket: WebSocket) {
    let (tx, rx) = channel();
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (socket_sender, mut socket_receiver) = mpsc::channel::<StdinDataStatus>(1);
    let poll_output = if let Some(Ok(Message::Text(msg))) = ws_receiver.next().await {
        println!("got request: {}", msg.clone());
        let code_req =  serde_json::from_str::<CodeRequest>(&msg);

        if let Ok(code_req) = code_req {
            println!("code: {}\nlanguage: {}", code_req.code, code_req.language);
            
            let code = code_req.code;
            let temp_folder_format = "./live-code/pyenv-XXXX";
            let temp_folder = temp_utils::create_temp_dir(temp_folder_format).unwrap().trim().to_string();
            let file_path = temp_folder.clone() + "/main.py";
            temp_utils::create_temp_file(&(temp_folder + "/main.py"));
            std::fs::write(file_path.clone(), code.as_bytes()).expect("ERROR WRITING TO FILE.");

            std::thread::spawn(move || poll_live_output(file_path, tx.clone()));
            rx.recv()
        } else {
            println!("{} is not deserializable!", msg);
            return;
        }
    } else {
        return;
    };
    if let Ok(StdinData::StdinSender(stdin_sender)) = poll_output {

        let output_task = tokio::spawn(async move {
            while let Ok(data) = rx.recv(){
                match data {
                    StdinData::Available(char) => {
                        let char = String::from_utf8(vec![char]).unwrap();
                        socket_sender.send(StdinDataStatus::Data(char)).await.unwrap();
                    },
                    StdinData::StdinSender(_) => {}, 
                    StdinData::Over => {
                        println!("RECEVED OVER");
                        socket_sender.send(StdinDataStatus::Over).await.unwrap();

                        return;
                    }
                }
            }
        });

        let send_task = spawn(async move {
            while let Some(msg) = socket_receiver.recv().await {
                match msg {
                    StdinDataStatus::Data(msg) => {
                        if ws_sender.send(Message::Text(msg)).await.is_err() {
                            println!("Failed to send message. Socket closed.");
                            return;
                        }
                    },
                    StdinDataStatus::Over => {
                        println!("Closing the connection...");
                        if let Err(e) = ws_sender.send(Message::Close(None)).await {
                            println!("Failed to send close frame: {:?}", e);
                        }

                        drop(ws_sender);
                        return;
                    }
                }
            }
        });

        let recv_task = spawn(async move {
            while let Some(msg) = ws_receiver.next().await {
                if msg.is_ok() {
                    let msg = msg.unwrap().to_text().unwrap().to_string() + "\n";
                    println!("got msg {}", msg.clone());
                    if stdin_sender.send(StdinDataStatus::Data(msg.clone())).is_err() {
                        println!("Can't send");
                        return
                    }
                } else {
                    let _ = stdin_sender.send(StdinDataStatus::Over);
                    return;
                }
            }
        });
        tokio::select! {
            _ = recv_task => println!("recv  task over"),
            _ = send_task => println!("send task over"),
            _ = output_task => println!("output task over"),
        }
    }
}

fn _get_live_code_output(json_request: CodeRequest) -> Response {
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

// TODO: Create a structure of functions with single db connection.

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
        .route("/v1/get_questions", get(serve_questions));

    let websocket_routes = Router::new()
        .route("/get_live_output", get(live_code_ws_handler));

    let page_routes = Router::new()
        .route("/question/:id", get(serve_question));

    let app = Router::new()
        .nest_service("/", ServeDir::new("coapi-frontend"))
        .nest("/api", api_routes)
        .nest("/pages", page_routes)
        .nest("/ws", websocket_routes)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}
