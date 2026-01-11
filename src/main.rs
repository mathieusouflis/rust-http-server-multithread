mod request;
mod types;
mod utils;
use std::{net::{TcpListener, TcpStream}};

use crate::request::main::Request;

fn main() {
  let host = "127.0.0.1:8080";
  let listener = TcpListener::bind(host).unwrap();

  println!("Listening http://{}", host);

  for streams in listener.incoming(){
    let stream = streams.unwrap();
    handle_connection(stream);
  }
}

fn handle_connection(stream: TcpStream) {
  let request = Request::new(stream);
}
