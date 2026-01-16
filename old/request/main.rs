use std::net::TcpStream;
use std::io::Read;

use crate::types::body::Body;
use crate::types::header::{HeaderType, Headers};
use crate::utils::parse_string_to_vec::parse_string_to_vec;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
}



#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    pub body: serde_json::Value
}

impl Request {

    fn get_method(method: String) -> Method {
      match method.to_lowercase().as_str() {
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "delete" => Method::DELETE,
        "options" => Method::OPTIONS,
        _ => Method::GET,
      }
    }

    fn parse_stream(stream: &mut TcpStream) -> Vec<String> {
        let mut buffer = [0; 2048];
        if let Ok(bytes_read) = stream.read(&mut buffer) {
          let request_str = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
          let filtered_lines: Vec<String> = parse_string_to_vec(request_str, "\r\n");

          filtered_lines
        } else {
            eprintln!("Failed to read from the stream");
            Vec::new()
        }
    }

    pub fn new(stream: &mut TcpStream) -> Request {
        let parsed_data = Self::parse_stream(stream);
        let method = Self::get_method(parsed_data[0].split_whitespace().next().unwrap().to_string());
        let path = parsed_data[0].split_whitespace().nth(1).unwrap().to_string();
        let headers = Headers::new(&HeaderType::Vector(&parsed_data));
        let body = Body::new(&parsed_data.last().unwrap_or(&String::new()));
        Request {
            method,
            path,
            headers,
            body,
        }
    }
}
