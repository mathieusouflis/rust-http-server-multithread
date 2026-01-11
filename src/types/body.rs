type BodyType = serde_json::Value;

pub trait Body {
    fn new(body: &str) -> Self;
}

impl Body for BodyType {
    fn new(body: &str) -> Self {
        serde_json::from_str(body).unwrap_or(serde_json::Value::Null)
    }
}
