use std::{
    io::{self, Write},
    net::{Shutdown, TcpStream},
};

#[derive(Debug, Clone)]
pub enum ResponseMessageType {
    Text(String),
    Json(serde_json::Value),
}

#[derive(Debug)]
pub struct Response {
    base_stream: TcpStream,
    content_type: String,
    status: u16,
    message: ResponseMessageType,
}

impl Response {
    pub fn new(base_stream: &TcpStream) -> Self {
        Response {
            base_stream: base_stream.try_clone().expect("failed to clone TcpStream"),
            status: 200,
            content_type: "text/plain".to_string(),
            message: ResponseMessageType::Text(String::new()),
        }
    }

    pub fn send<T>(&self, message: Option<T>) -> io::Result<()>
    where
        T: Into<ResponseMessageType>,
    {
        let message_to_send: ResponseMessageType = match message {
            Some(m) => m.into(),
            None => self.message.clone(),
        };

        let (body, content_type) = match &message_to_send {
            ResponseMessageType::Text(s) => (s.clone(), self.content_type.clone()),
            ResponseMessageType::Json(v) => (v.to_string(), "application/json".to_string()),
        };

        let reason = reason_phrase(self.status);

        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            reason,
            content_type,
            body.as_bytes().len(),
            body
        );

        let mut stream = self.base_stream.try_clone()?;
        stream.write_all(response.as_bytes())?;
        Ok(())
    }

    pub fn end(&self) -> io::Result<()> {
        self.base_stream.shutdown(Shutdown::Both)?;
        Ok(())
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn set_content_type<S: Into<String>>(&mut self, ct: S) {
        self.content_type = ct.into();
    }

    pub fn set_message(&mut self, msg: ResponseMessageType) {
        self.message = msg;
    }
}

impl From<&str> for ResponseMessageType {
    fn from(value: &str) -> Self {
        ResponseMessageType::Text(value.to_string())
    }
}

impl From<String> for ResponseMessageType {
    fn from(value: String) -> Self {
        ResponseMessageType::Text(value)
    }
}

impl From<serde_json::Value> for ResponseMessageType {
    fn from(value: serde_json::Value) -> Self {
        ResponseMessageType::Json(value)
    }
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        100 => "Continue",
        101 => "Switching Protocols",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        503 => "Service Unavailable",
        _ => "",
    }
}
