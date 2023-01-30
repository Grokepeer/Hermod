// Library to read, interpretate and operate on JSON data in Rust
pub struct Json {
    json: String
}

impl Json {
    pub fn from(content: &str) -> Json {
        Json {
            json: content.to_string(),
        }
    }

    pub fn find(&self, query: &str) -> u8 {
        return 0;
    }
}

pub fn read(query: &str) -> u8 {
    return 0;
}