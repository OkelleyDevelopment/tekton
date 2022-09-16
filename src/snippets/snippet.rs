use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Snippet {
    pub prefix: String,
    pub body: Vec<String>,
}

impl Snippet {
    pub fn new(prefix: String, body: Vec<String>) -> Snippet {
        Snippet { prefix, body }
    }

    pub fn display(self) -> String {
        let mut s = String::from("snippet ".to_string() + &self.prefix + &"\n".to_string());

        s = str::replace(&s, "\"", "");
        for item in self.body {
            
            let edited_item = str::replace(&item, "\"", "" );
            let line = "\t".to_string() + &edited_item+ "\n";
            s += &line;
        }
        s
    }
}