//! Structures to model the Snipmate snippet format

use regex::Regex;
use serde::Serialize;

/// A structure representing the vim-snippet/ Snipmate format
#[derive(Debug, Serialize)]
pub struct Snipmate {
    pub prefix: String,
    pub body: Vec<String>,
    pub description: String,
}

impl Snipmate {
    /// A function to create new Snipmate snippet structs
    pub fn new(prefix: String, body: Vec<String>, description: String) -> Snipmate {
        Snipmate {
            prefix,
            body,
            description,
        }
    }

    /// Converts the snippet to a string
    pub fn display(self) -> String {
        let re = Regex::new(r##"^"|"$"##).unwrap();
        let re2 = Regex::new(r##"\\""##).unwrap();
        let quote = String::from("\"+");
        let tab = String::from("\\t");
        let re3 = Regex::new(&quote).unwrap();
        let tab_regex = Regex::new(&tab).unwrap();

        // This creates the first line of the snippet,
        // taking the form: `snippet <prefix> <Optional: description in quotes>`
        let mut s = "snippet ".to_string() + &self.prefix;
        // Note: this is done in an attempt to remove the extra quotes needed in JSON
        s = str::replace(&s, "\"", "");

        s = s + " " + &self.description + "\n";

        s = re3.replace_all(&s, '"'.to_string()).to_string();

        for item in self.body {
            let mut edited_item = re.replace_all(&item, "").to_string();
            edited_item = re2.replace_all(&edited_item, '"'.to_string()).to_string();
            edited_item = tab_regex.replace_all(&edited_item, " ").to_string();
            let line = "\t".to_string() + &edited_item + "\n";
            s += &line;
        }
        s
    }
}

#[test]
fn test_vim_snippet_creation() {
    let snip = Snipmate::new(
        String::from("test"),
        Vec::new(),
        String::from("An epic test description"),
    );
    assert_eq!(snip.prefix, "test".to_string());
    assert_eq!(snip.body.len(), 0);
    assert_eq!(snip.description, "An epic test description".to_string());
}

#[test]
fn test_vim_snippet_display() {
    let mut snip = Snipmate::new(
        String::from("test"),
        Vec::new(),
        String::from("An epic test description"),
    );
    snip.body.push(String::from("A line of snippet"));
    let string_rep = snip.display();

    assert_eq!(
        string_rep,
        String::from("snippet test An epic test description\n\tA line of snippet\n")
    );
}
