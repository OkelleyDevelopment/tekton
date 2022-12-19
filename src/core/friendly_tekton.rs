//! Functions related to creating and manipulating FriendlySnippets (JSON)

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
use std::{collections::{HashMap, BTreeMap}, fs};

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
        let string_rep = build_sorted_string(v, &friendlies.snippets)?;
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
pub fn read_in_json_snippets(file_name: &str) -> Result<FriendlySnippets, TektonError> {
    if let Ok(file_contents) = fs::read_to_string(&file_name) {
        let snippets: Result<FriendlySnippets, serde_json::Error> =
            serde_json::from_str(&file_contents);
        if let Ok(snippets) = snippets {
            return Ok(snippets);
        } else if let Ok(snippets) = dynamically_read_json_snippets(file_contents) {
            return Ok(snippets);
        }
    }
    Err(TektonError::Reason(
        "Failed to read in the JSON as a string".to_string(),
    ))
}

/// Helper function to read in the JSON for the `FriendlySnippets`, given uncertain JSON formatting
///
/// The `read_in_json_snippets` function should be preferred, however the ordering of fields in JSON isn't promised
/// and thus, this function builds the HashMap (backing the `FriendlySnippets` structure) by dynamically searching the
/// the table for the necessary fields.
pub fn dynamically_read_json_snippets(file: String) -> Result<FriendlySnippets, TektonError> {
    let mut snippets: HashMap<String, FriendlySnippetBody> = HashMap::new();
    let json: serde_json::Value = serde_json::from_str(&file).unwrap();

    if let Some(obj) = json.as_object() {
        for (k, v) in obj {
            let name = k;
            let prefix = v["prefix"].to_string();
            let mut body: Vec<String> = Vec::new();
            for line in v["body"].as_array().unwrap().iter() {
                body.push(line.to_string());
            }

            let description = v["description"].to_string();
            let snip_body = FriendlySnippetBody::new(prefix, body, description);

            snippets.insert(name.to_string(), snip_body);
        }
    }

    Ok(FriendlySnippets { snippets })
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

    match get_friendly_snippet_keys(&snippets.snippets) {
        Some(list) => {
            let string_rep = build_sorted_string(list, &snippets.snippets)?;
            Ok(string_rep)
        }
        None => Err(TektonError::Reason(
            "Cannot sort nor write `None`.".to_string(),
        )),
    }
}

/// Helper function to construct the JSON string representation of the `FriendlySnippets` struct
///
/// It is done like this to ensure we sort correctly.
fn build_sorted_string(
    names: Vec<String>,
    table: &std::collections::HashMap<String, FriendlySnippetBody>,
) -> Result<String, TektonError> {
    // 1. Check that we have something to work with
    match names.len() {
        0 => Err(TektonError::Reason(
            "Refusing to build string for 0 snippets".to_string(),
        )),
        _ => {
            // 2. This provides an ordering
            let ordered: BTreeMap<_, _> = table.into_iter().collect();
            // Return the result
            match serde_json::to_string_pretty(&ordered) {
                Ok(snippets) => Ok(snippets),
                Err(e) => Err(TektonError::Reason(e.to_string()))
            }
        }
    }
}
