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

#[derive(Serialize, Deserialize)]
struct StdinRequest{
    stdin: String,
}


fn send_message(stream: &mut UnixStream, buffer: &mut Vec<u8>, message: &str) {
    let message_bytes = message.as_bytes();

    if message_bytes.len() > buffer.len() {
        panic!("Message too big");
    }

    buffer.fill(0);
    buffer[..message_bytes.len()].copy_from_slice(message_bytes);

    stream.write_all(buffer).unwrap();
}

fn send_request(mut stream: UnixStream) {
    let mut request = vec![0u8; 1024];
    let mut response = [0; 1024];
    let json_request = OutputRequest {
        code: "name = input('Enter your name');print('hello ', name);initial = input('Enter initial');print('full name:', name, initial);".to_string(),
    };
    let stdin_request1 = StdinRequest {
        stdin: "Aadish".to_string(),
    };
    let stdin_request2 = StdinRequest {
        stdin: "M".to_string(),
    };
    let json_msg = serde_json::to_string(&json_request).unwrap();
    let stdin_msg1 = serde_json::to_string(&stdin_request1).unwrap();
    let stdin_msg2 = serde_json::to_string(&stdin_request2).unwrap();

    send_message(&mut stream, &mut request, &json_msg);
    send_message(&mut stream, &mut request, &stdin_msg1);
    send_message(&mut stream, &mut request, &stdin_msg2);

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
