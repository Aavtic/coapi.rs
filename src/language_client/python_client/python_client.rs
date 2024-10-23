use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

const IPC_SOCKET: &str = "./src/language_servers/python_ls/ipc.sock";
// const BUFFER_MUL: u32 = 1024;

#[derive(Serialize, Deserialize)]
struct OutputRequest {
    code: String,
}


fn send_request(mut stream: UnixStream) {
    let mut request = Vec::with_capacity(1024);
    let mut response = [0; 1024];
    let json_request = OutputRequest {
        code: "print('Hello World');\nprint(\"HELLO\")".to_string(),
    };
    let json_msg = serde_json::to_string(&json_request).unwrap();

    request.write_all(json_msg.as_bytes()).unwrap();
    stream.write_all(&request).unwrap();
    stream.read(&mut response).unwrap();
    println!("{}", String::from_utf8(response.to_vec()).unwrap());
}

fn get_connection() -> UnixStream {
    return UnixStream::connect(IPC_SOCKET).unwrap();
}

pub fn get_output() {
    let stream = get_connection();
    send_request(stream);
}
