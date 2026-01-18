pub struct Header {
  name: String,
  value: String
}

pub type Headers = Vec<Header>;

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Header {
            name,
            value,
        }
    }
}
