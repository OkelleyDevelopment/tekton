use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Snippets {
    pub prefix: String,
    pub body: Vec<String>,
}

impl Snippets {
    pub fn new(prefix: String, body: Vec<String>) -> Snippets {
        Snippets { prefix, body }
    }
}
