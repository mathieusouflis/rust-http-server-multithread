#[derive(Debug)]
pub struct Headers {
    pub cookies: serde_json::Value,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub accept_language: String,
    pub accept_encoding: String,
    pub connection: String,
    pub content_type: Option<String>,
    pub content_length: Option<usize>,
}

#[derive(Debug)]
pub enum HeaderType {
    Vector(Vec<String>),
    Object(serde_json::Value),
}

impl Headers {
    fn parse_header_line(header: &str) -> Option<(&str, &str)> {
        let parts: Vec<&str> = header.splitn(2, ": ").collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    }

    fn process_header_field(
        key: &str,
        value: &str,
        cookies: &mut String,
        host: &mut String,
        user_agent: &mut String,
        accept: &mut String,
        accept_language: &mut String,
        accept_encoding: &mut String,
        connection: &mut String,
        content_type: &mut Option<String>,
        content_length: &mut Option<usize>,
    ) {
        match key {
            "Cookie" => *cookies = value.to_string(),
            "Host" => *host = value.to_string(),
            "User-Agent" => *user_agent = value.to_string(),
            "Accept" => *accept = value.to_string(),
            "Accept-Language" => *accept_language = value.to_string(),
            "Accept-Encoding" => *accept_encoding = value.to_string(),
            "Connection" => *connection = value.to_string(),
            "Content-Type" => *content_type = Some(value.to_string()),
            "Content-Length" => *content_length = value.parse().ok(),
            _ => {}
        }
    }

    fn from_vector(headers: Vec<String>) -> Self {
        let mut cookies = String::new();
        let mut host = String::new();
        let mut user_agent = String::new();
        let mut accept = String::new();
        let mut accept_language = String::new();
        let mut accept_encoding = String::new();
        let mut connection = String::new();
        let mut content_type = None;
        let mut content_length = None;

        for header in headers {
            if let Some((key, value)) = Self::parse_header_line(&header) {
                Self::process_header_field(
                    key,
                    value,
                    &mut cookies,
                    &mut host,
                    &mut user_agent,
                    &mut accept,
                    &mut accept_language,
                    &mut accept_encoding,
                    &mut connection,
                    &mut content_type,
                    &mut content_length,
                );
            }
        }

        Headers {
            cookies: Self::parse_cookies(cookies),
            host,
            user_agent,
            accept,
            accept_language,
            accept_encoding,
            connection,
            content_type,
            content_length,
        }
    }

    fn parse_cookies(cookies: String) -> serde_json::Value {
        let mut cookie_map = serde_json::Map::new();
        for cookie in cookies.split("; ") {
            let parts: Vec<&str> = cookie.split("=").collect();
            if parts.len() == 2 {
                cookie_map.insert(parts[0].to_string(), serde_json::Value::String(parts[1].to_string()));
            }
        }
        serde_json::Value::Object(cookie_map)
    }

    pub fn new(headers: HeaderType) -> Self {
        match headers {
            HeaderType::Vector(values) => Self::from_vector(values),
            HeaderType::Object(value) => {
                println!("Head is a JsonValue with value: {:?}", value);
                let cookie = value.get("cookie").cloned().expect("Cookie is missing in header...");
                Headers {
                  cookies: cookie,
                    host: String::new(),
                    user_agent: String::new(),
                    accept: String::new(),
                    accept_language: String::new(),
                    accept_encoding: String::new(),
                    connection: String::new(),
                    content_type: None,
                    content_length: None,
                }
            }
        }
    }
}
