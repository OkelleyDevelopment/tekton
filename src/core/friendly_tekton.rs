use super::snipmate_tekton::build_snippets_from_file;
use crate::{
    errors::TektonError,
    snippets::{
        friendly::{FriendlySnippetBody, FriendlySnippets},
        snipmate::Snipmate,
    },
    utils::get_input,
};
use regex::Regex;
use std::collections::HashMap;

/// A helper function to handle Snipmate to JSON
pub fn compose_friendly_snippets(lines: Vec<String>) -> Result<String, TektonError> {
    let snips = build_snippets_from_file(lines);
    let friendlies = convert_snipmate_to_friendlysnippets(snips);
    let result = build_friendly_string(friendlies)?;
    Ok(result)
}

/// A function that takes a Vec of FriendlySnippet structs and returns the string representation or a TektonError
pub fn build_friendly_string(friendlies: FriendlySnippets) -> Result<String, TektonError> {
    if let Some(v) = get_friendly_snippet_keys(&friendlies.snippets) {
        let string_rep = build_string(v, &friendlies.snippets)?;
        Ok(string_rep)
    } else {
        Err(TektonError::Reason("No snippets provided".to_string()))
    }
}

/// A function to convert an array of Snipmate structs to an array of FriendlySnippet structs
pub fn convert_snipmate_to_friendlysnippets(snips: Vec<Snipmate>) -> FriendlySnippets {
    let mut friendly_handle: FriendlySnippets = FriendlySnippets {
        snippets: HashMap::new(),
    };

    let re = Regex::new(r##"\\""##).unwrap();

    let mut count: usize = 0;
    let target = snips.len();
    for snippet in snips {
        let prefix: String = snippet.prefix;
        let mut body: Vec<String> = snippet.body;
        // --------------------------------------------------------------
        // NOTE: Remove the whitespace for the very first line of the snippet
        body.reverse();
        let first_line: String = body.pop().unwrap().to_string();
        body.reverse();
        body.insert(0, first_line.trim_start().to_string());
        // --------------------------------------------------------------
        let description: String = re.replace_all(&snippet.description, "").to_string();
        let friendly_body = FriendlySnippetBody::new(prefix, body, description);

        match serde_json::to_string_pretty(&friendly_body) {
            Ok(snip) => {
                count += 1;
                print!("\x1B[2J\x1B[1;1H"); // Clear terminal
                println!(
                    "Snippet {} of {}:\n{}\n\nEnter name below:",
                    count, target, snip
                );
                let key = get_input();
                friendly_handle.snippets.insert(key, friendly_body);
            }
            Err(e) => {
                // Err(TektonError::Reason(e.to_string())
                eprintln!("Match had an error in conversion ----> {}", e);
            }
        }
    }
    friendly_handle
}

/// Helper function to read the JSON as a `FriendlySnippets` struct
pub fn read_in_json_snippets(file: String) -> Result<FriendlySnippets, TektonError> {
    let snippets: Result<FriendlySnippets, serde_json::Error> = serde_json::from_str(&file);

    match snippets {
        Ok(s) => Ok(s),
        Err(e) => Err(TektonError::Reason(e.to_string())),
    }
}

/// Helper function to retrive and sort the names of the snippets (the keys of the hashmap)
fn get_friendly_snippet_keys(
    table: &std::collections::HashMap<String, FriendlySnippetBody>,
) -> Option<Vec<String>> {
    let keys = table.keys();
    let mut names: Vec<String> = Vec::new();
    for k in keys {
        names.push(k.clone());
    }
    names.sort_by_key(|a| a.to_lowercase()); // this might cause problems (leaving note for easy search)
    Some(names)
}

/// Function that builds a string representing the snippets in sorted order
pub fn sort_friendly_snippets(snippets: FriendlySnippets) -> Result<String, TektonError> {
    let table = snippets.snippets;

    match get_friendly_snippet_keys(&table) {
        Some(list) => {
            let string_rep = build_string(list, &table)?;
            Ok(string_rep)
        }
        None => Err(TektonError::Reason(
            "Cannot sort nor write `None`.".to_string(),
        )),
    }
}

/// Helper function to construct the JSON string representation of the `FriendlySnippets` struct
fn build_string(
    names: Vec<String>,
    table: &std::collections::HashMap<String, FriendlySnippetBody>,
) -> Result<String, TektonError> {
    match names.len() {
        0 => Err(TektonError::Reason(
            "Refusing to build string for 0 snippets".to_string(),
        )),
        _ => {
            // Manually implement so that we can ensure sort
            let mut count: usize = 0;
            let target: usize = names.len();
            let mut snippet_string: String = String::from("{\n");
            for key in names {
                if let Some(v) = table.get(&key) {
                    count += 1;
                    let mut snippet: String = "\"".to_string()
                        + &key
                        + "\": "
                        + &serde_json::to_string_pretty(&v).unwrap();

                    if count < target {
                        snippet += ",";
                    }
                    snippet_string += &snippet;
                }
            }
            snippet_string += "\n}";
            Ok(snippet_string)
        }
    }
}
