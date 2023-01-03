//! Structures to model the Snipmate snippet format

use regex::Regex;

/// A structure representing the vim-snippet/ Snipmate format
#[derive(Debug)]
pub struct Snipmate {
    /// The trigger for the snippet
    pub prefix: String,
    /// The content of the snippet
    pub body: Vec<String>,
    /// A small summary of the snippet
    pub description: Option<String>,
}

impl Snipmate {
    /// A function to create new Snipmate snippet structs
    pub fn new(prefix: String, body: Vec<String>, description: Option<String>) -> Snipmate {
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
        let mut snippet_string = "snippet ".to_string() + &self.prefix;
        // Note: this is done in an attempt to remove the extra quotes needed in JSON
        snippet_string = str::replace(&snippet_string, "\"", "");

        // Check for a description, append if its there
        if let Some(description) = &self.description {
            if !description.is_empty() {
                snippet_string = snippet_string + " " + description;
            }
        }

        snippet_string += "\n"; // Catch the new line after either the prefix or the end of the description

        snippet_string = re3
            .replace_all(&snippet_string, '"'.to_string())
            .to_string();

        // Loops over the lines of the body to replace any extra quotations
        for line in self.body {
            let mut edited_line_item = re.replace_all(&line, "").to_string();
            edited_line_item = re2
                .replace_all(&edited_line_item, '"'.to_string())
                .to_string();
            edited_line_item = tab_regex.replace_all(&edited_line_item, " ").to_string();
            let line = "\t".to_string() + &edited_line_item + "\n";
            snippet_string += &line;
        }
        snippet_string
    }
}

#[test]
fn test_vim_snippet_creation_with_description() {
    let snip = Snipmate::new(
        String::from("test"),
        Vec::new(),
        Some(String::from("An epic test description")),
    );
    assert_eq!(snip.prefix, "test".to_string());
    assert_eq!(snip.body.len(), 0);
    assert_eq!(
        snip.description,
        Some("An epic test description".to_string())
    );
}

#[test]
fn test_vim_snippet_creation_without_description() {
    let snip = Snipmate::new(String::from("test"), Vec::new(), None);
    assert_eq!(snip.prefix, "test".to_string());
    assert_eq!(snip.body.len(), 0);
    assert_eq!(snip.description, None);
}

#[test]
fn test_vim_snippet_display() {
    let mut snip = Snipmate::new(
        String::from("test"),
        Vec::new(),
        Some(String::from("An epic test description")),
    );
    snip.body.push(String::from("A line of snippet"));
    let string_rep = snip.display();

    assert_eq!(
        string_rep,
        String::from("snippet test An epic test description\n\tA line of snippet\n")
    );
}

#[test]
fn test_vim_snippet_display_without_description() {
    let mut snip = Snipmate::new(String::from("test"), Vec::new(), None);
    snip.body.push(String::from("A line of snippet"));
    let string_rep = snip.display();

    assert_eq!(
        string_rep,
        String::from("snippet test\n\tA line of snippet\n")
    );
}
