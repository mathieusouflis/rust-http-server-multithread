use crate::http::{header::Header, method::Method};

pub struct Request {
    method: Method,
    path: String,
    headers: Vec<Header>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(method: Method, path: String, headers: Vec<Header>, body: Option<Vec<u8>>) -> Self {
        Request {
            method,
            path,
            headers,
            body,
        }
    }
}
