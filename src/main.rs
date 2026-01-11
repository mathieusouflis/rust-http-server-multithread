mod request;
mod types;
mod utils;
mod response;

use std::net::{TcpListener, TcpStream};

use crate::request::main::Request;
use crate::response::Response;

fn main() {
    let host = "127.0.0.1:8080";
    let listener = TcpListener::bind(host).expect("failed to bind address");
    println!("Listening http://{}", host);

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("THREAD");
    let _request = Request::new(&mut stream);
    let response = Response::new(&stream);

    if let Err(e) = response.send(Some("Hello world!")) {
        eprintln!("Failed to send response: {}", e);
    }

    if let Err(e) = response.end() {
        eprintln!("Failed to shutdown connection: {}", e);
    }
}
