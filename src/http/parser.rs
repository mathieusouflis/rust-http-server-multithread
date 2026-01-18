use crate::http::{header::{Header, Headers}, method::Method, request::Request};

#[derive(Debug)]
pub enum ParseError {
    MissingRequestLine,
    MissingMethod,
    MissingPath,
    MissingVersion,
    UnsupportedVersion,
    MissingHeaderName,
    MissingHeaderValue,
    InvalidMethod,
}

pub struct Parser;

impl Parser {
  pub fn parse_headers(lines: &[&str]) -> Result<Headers, ParseError> {
    let mut headers = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }

        let mut parts = line.splitn(2, ": ");
        let name = parts.next().ok_or(ParseError::MissingHeaderName)?;
        let value = parts.next().ok_or(ParseError::MissingHeaderValue)?;

        headers.push(Header::new(name.to_string(), value.to_string()));
    }
    Ok(headers)
  }

  fn parse_request_line(input: &str) -> Result<(Method, String, String), ParseError> {
      let mut parts = input.split_whitespace();
      let method = parts.next().ok_or(ParseError::MissingMethod)?;
      let path = parts.next().ok_or(ParseError::MissingPath)?;
      let version = parts.next().ok_or(ParseError::MissingVersion)?;

      if version != "HTTP/1.1" {
          return Err(ParseError::UnsupportedVersion);
      }

      Ok((
          Method::from_str(method).map_err(|_| ParseError::InvalidMethod)?,
          path.to_string(),
          version.to_string(),
      ))
  }

  fn parse_request(input: &str) -> Result<Request, ParseError> {
      let mut lines = input.lines();
      let request_line = lines.next().ok_or(ParseError::MissingRequestLine)?;

      let (method, path, version) = Self::parse_request_line(request_line)?;

      let headers = match Self::parse_headers(&lines.collect::<Vec<&str>>()) {
          Ok(h) => h,
          Err(e) => return Err(e),
      };

      Ok(Request::new(
          method,
          path.to_string(),
          headers,
          None
      ))
  }
}
