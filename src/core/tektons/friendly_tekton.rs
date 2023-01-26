//! Functions related to creating and manipulating FriendlySnippets (JSON)

use super::snipmate_tekton::build_snippets_from_file;
use crate::{
    errors::TektonError,
    models::{
        friendly::{FriendlySnippetBody, FriendlySnippets, Table},
        snipmate::Snipmate,
    },
    utils::{clear_terminal, get_input, hash2ordered_string},
};
use regex::Regex;
use std::{collections::HashMap, fs};

const MISSING_PREFIX: &str = "File contains snippets with missing prefix field(s). Aborting.";

/// Function to handle Snipmate to JSON
///
/// Arguments:
/// - `lines`: the lines of Snipmate snippets to be converted
///
/// Returns:
/// - The converted JSON string representation or an error
pub fn compose_friendly_snippets(lines: Vec<String>) -> Result<String, TektonError> {
    let snips = build_snippets_from_file(lines);
    let friendlies = convert_snipmate_to_friendlysnippets(snips);
    let result = build_friendly_string(friendlies)?;
    Ok(result)
}

/// A function that takes the FriendlySnippet table and returns an ordered string representation or a TektonError
///
/// Arguments:
/// - `friendlies`: the snippets that will be converted to a string
///
/// Returns:
/// - The string with 1 to many snippets or an error.
pub fn build_friendly_string(friendlies: FriendlySnippets) -> Result<String, TektonError> {
    hash2ordered_string(&friendlies.snippets)
}

/// A function to convert an array of Snipmate structs to an array of FriendlySnippet structs
///
/// Arguments:
/// - `snips`: the vector of Snipmate snippets to be converted
///
/// Returns:
/// - The table of FriendlySnippets with 0 to many snippets (more or less).
pub fn convert_snipmate_to_friendlysnippets(snips: Vec<Snipmate>) -> FriendlySnippets {
    let mut count: usize = 0;
    let target = snips.len();
    let re = Regex::new(r##"\\""##).unwrap();
    let mut friendly_handle: FriendlySnippets = FriendlySnippets::new();

    for snippet in snips {
        let prefix: String = snippet.prefix;
        let mut body: Vec<String> = snippet.body;
        let mut description: String = String::new();

        // NOTE: Remove the whitespace for the very first line of the snippet
        if let Some(first_line) = body.get(0) {
            body.insert(0, first_line.trim_start().to_string());
        }

        if let Some(descrip) = &snippet.description {
            description = re.replace_all(descrip, "").to_string();
        }

        let friendly_body = FriendlySnippetBody::new(Some(prefix), body, Some(description));

        match serde_json::to_string_pretty(&friendly_body) {
            Ok(snip) => {
                count += 1;
                clear_terminal();
                println!(
                    "Snippet {} of {}:\n{}\n\nEnter name below:",
                    count, target, snip
                );
                let key = get_input();
                friendly_handle.snippets.insert(key, friendly_body);
            }
            Err(e) => {
                eprintln!("Match had an error in conversion ----> {}", e);
            }
        }
    }
    friendly_handle
}

/// Helper function to read the JSON as a `FriendlySnippets` struct
///
/// Arguments:
/// - `file_name`: a string of the input file
/// - `interactive`: a boolean indicating if the user will be invovled or not
pub fn read_in_json_snippets(
    file_name: &str,
    interactive: bool,
) -> Result<FriendlySnippets, TektonError> {
    let file_contents = fs::read_to_string(file_name)?;
    let snippets: Result<FriendlySnippets, serde_json::Error> =
        serde_json::from_str(&file_contents);
    match snippets {
        Ok(snippets) => Ok(snippets),
        Err(_) => match dynamically_read_json_snippets(file_contents, interactive) {
            Ok(snippets) => Ok(snippets),
            Err(e) => Err(e),
        },
    }
}

/// Helper function to read in the JSON for the `FriendlySnippets`, given uncertain JSON formatting
///
/// The [read_in_json_snippets] function should be preferred, however the ordering of fields in JSON isn't promised
/// and thus, this function builds the HashMap (backing the `FriendlySnippets` structure) by dynamically searching the
/// the table for the necessary fields and handling the missing ones more appropriately.
///
/// Arguments:
/// - `file`: a string of the input file
/// - `interactive`: a boolean indicating if the user will be invovled or not
///
/// Returns:
/// - Result of the snippets read in or an error
pub fn dynamically_read_json_snippets(
    file: String,
    interactive: bool,
) -> Result<FriendlySnippets, TektonError> {
    // The snippet table (what is being created/ read in)
    let mut snippets: Table = HashMap::new();
    // The blob of JSON from serde_json
    let json: serde_json::Value = serde_json::from_str(&file).unwrap();
    // The 'need to fix this' pile
    let mut snippets_to_fix: Vec<(String, FriendlySnippetBody)> = Vec::new();

    if let Some(obj) = json.as_object() {
        for (name, v) in obj {
            // Collect the lines of the snippet body (outsourced to a helper function)
            let body = retrieve_body(&v["body"]);

            // Create snippet body assuming no description
            let mut snip_body = FriendlySnippetBody::new(None, body, None);

            // If we find one, then update the structure
            if let Some(description) = v["description"].as_str() {
                if !description.is_empty() {
                    snip_body.description = Some(description.to_string());
                }
            }

            // Find the prefix or add to the 'fix later' vec
            if let Some(pref_candidate) = retrieve_prefix(&v["prefix"]) {
                snip_body.prefix = Some(pref_candidate);
            } else if interactive {
                // skip inserting a malformed snippet into the table, will fix later
                snippets_to_fix.push((name.to_string(), snip_body));
                continue;
            } else {
                // Return an error, this is useful for automation since errors can be collected
                // and addressed after the batch is finished.
                return Err(TektonError::Reason(MISSING_PREFIX.into()));
            }

            // Insert the finished snippet into the table
            snippets.insert(name.to_string(), snip_body);
        }
        // Congrats, it is later, now to fix the snippets
        correct_missing_prefix_snippets(&mut snippets_to_fix, &mut snippets);
    }

    Ok(FriendlySnippets { snippets })
}

/// A function to handle the correction of snippets that are missing their prefix
///
/// Arguments:
/// - `snippets_to_fix`: A mutable reference to a vector with the name and partial snippet body
/// - `snippets`: A mutable reference to the table that the corrected snippet will be inserted into
///
pub fn correct_missing_prefix_snippets(
    snippets_to_fix: &mut Vec<(String, FriendlySnippetBody)>,
    snippets: &mut Table,
) {
    if !snippets_to_fix.is_empty() {
        let mut count = 0;
        let total = snippets_to_fix.len();
        loop {
            if count == total {
                break;
            }

            if let Some((name, snip_body)) = snippets_to_fix.pop() {
                count += 1;
                println!("Fixing snippet {} of {}", count, total);
                let snip_body = handle_prompt_for_prefix(&name, snip_body);
                snippets.insert(name.to_string(), snip_body);
            }
        }
    }
}

/// A function that gets the users new prefix and updates the snippet, returning the properly formed body.
///
/// Arguments:
/// - `name`: a string slice representing the snippet name
/// - `snip_body`: the snippets partially formed body
///
/// Returns:
/// - `FriendlySnippetBody`: the updated snippet body
fn handle_prompt_for_prefix(name: &str, mut snip_body: FriendlySnippetBody) -> FriendlySnippetBody {
    println!(
        "---- Snippet: {} ---\n{}\n--------",
        name,
        serde_json::to_string_pretty(&snip_body).ok().unwrap() // This unwrap will probably steal our lunch money later on.
    );
    println!("Enter a prefix:");
    loop {
        let prefix_candidate = get_input();

        println!("Proceed? (y/n):");
        let resp = get_input().to_lowercase();

        if resp == "y" {
            snip_body.prefix = Some(prefix_candidate);
            break;
        }

        // The user wants to correct the input, so we re-prompt
        println!("Enter a new prefix: ");
    }
    clear_terminal();
    snip_body
}

/// Function to handle the parsing of the prefix for a JSON snippet
///
/// Arguments:
/// - `val`: a reference to a serde_json::Value that represents the snippet prefix
///
/// Returns:
/// - Optional string if the provided Value can be modeled as a string
fn retrieve_prefix(val: &serde_json::Value) -> Option<String> {
    val.as_str().map(|prefix| prefix.to_string())
}

/// Function to handle processing the body of a JSON snippet
///
/// Arguments:
/// - `val`:  a reference to a serde_json::Value
///
/// Returns:
/// - `Vec<String>` representing the 'content' of the snippet
pub fn retrieve_body(val: &serde_json::Value) -> Vec<String> {
    let mut body: Vec<String> = Vec::new();
    if let Some(lines) = val.as_array() {
        for line in lines.iter() {
            body.push(line.as_str().unwrap_or("").to_string());
        }
    } else {
        body.push(val.as_str().unwrap_or("").to_string());
    }

    body
}

/// Function that builds a string representing the snippets in sorted order
///
/// Arguments:
/// - `snippets`: The snippets to be sorted
///
/// Returns:
/// - A result with the sorted string or an error if there were zero (0) snippets
pub fn sort_friendly_snippets(snippets: FriendlySnippets) -> Result<String, TektonError> {
    let table = &snippets.snippets;
    match table.len() {
        0 => Err(TektonError::Reason(
            "Refusing to build string for 0 snippets".to_string(),
        )),
        _ => hash2ordered_string(table),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const INTERACTIVE: bool = false;

    #[test]
    fn standard_json_reading() {
        let file = r#"{
            "beta": {
              "prefix": "println",
              "body": ["println!(\"${1}\");"],
              "description": "println!(…);"
            },
            "alpha": {
              "prefix": "print",
              "body": ["print!(\"${1}\");"],
              "description": "print!(…);"
            }
          }"#
        .to_string();

        let snippets: Result<FriendlySnippets, serde_json::Error> = serde_json::from_str(&file);

        match snippets {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    Some("println".to_string()),
                    vec!["println!(\"${1}\");".to_string()],
                    Some("println!(…);".to_string()),
                );
                assert_eq!(res.snippets.len(), 2);
                let item = res.snippets.get("beta").unwrap();
                assert_eq!(item.prefix, expected_struct.prefix);
                assert_eq!(item, &expected_struct);
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                assert!(false);
            }
        }
    }

    #[test]
    fn dyn_json_reading() {
        let file = r#"{
          "beta": {
            "prefix": "println",
            "body": ["println!(\"${1}\");"],
            "description": "println!(…);"
          },
          "alpha": {
            "prefix": "print",
            "body": ["print!(\"${1}\");"],
            "description": "print!(…);"
          }
        }"#
        .to_string();

        let res = dynamically_read_json_snippets(file, INTERACTIVE);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    Some("println".to_string()),
                    vec!["println!(\"${1}\");".to_string()],
                    Some("println!(…);".to_string()),
                );
                assert_eq!(res.snippets.len(), 2);
                let item = res.snippets.get("beta").unwrap();
                assert_eq!(item.prefix, expected_struct.prefix);
                assert_eq!(item, &expected_struct);
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                assert!(false);
            }
        }
    }

    #[test]
    fn jekyll() {
        let file = r#"{
        "Filter downcase": {
          "prefix": "downcase",
          "description": "String filter: downcase",
          "body": "| downcase }}"
        }}"#
        .to_string();

        let res = dynamically_read_json_snippets(file, INTERACTIVE);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    Some("downcase".to_string()),
                    vec!["| downcase }}".to_string()],
                    Some("String filter: downcase".to_string()),
                );
                assert_eq!(res.snippets.len(), 1);
                let item = res.snippets.get("Filter downcase").unwrap();
                assert_eq!(item.prefix, expected_struct.prefix);
                assert_eq!(item, &expected_struct);
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                assert!(false);
            }
        }
    }

    #[test]
    fn serialization() {
        let file = r#"{
    "Filter downcase": {
      "prefix": "downcase",
      "description": "String filter: downcase",
      "body": "| downcase }}"
    }}"#
        .to_string();

        let res = dynamically_read_json_snippets(file, INTERACTIVE);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    Some("downcase".to_string()),
                    vec!["| downcase }}".to_string()],
                    Some("String filter: downcase".to_string()),
                );
                assert_eq!(res.snippets.len(), 1);
                let item = res.snippets.get("Filter downcase").unwrap();
                assert_eq!(item.prefix, expected_struct.prefix);
                assert_eq!(item, &expected_struct);

                if let Ok(s) = serde_json::to_string(&res) {
                    const EXPECTED: &str = "{\"Filter downcase\":{\"prefix\":\"downcase\",\"body\":[\"| downcase }}\"],\"description\":\"String filter: downcase\"}}";
                    assert_eq!(s, EXPECTED);
                } else {
                    assert!(false);
                }
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                assert!(false);
            }
        }
    }

    #[test]
    fn serialization_with_empty_description() {
        let file = r#"{
    "Filter downcase": {
      "prefix": "downcase",
      "body": "| downcase }}"
    }}"#
        .to_string();

        let res = dynamically_read_json_snippets(file, INTERACTIVE);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    Some("downcase".to_string()),
                    vec!["| downcase }}".to_string()],
                    None,
                );
                assert_eq!(res.snippets.len(), 1);
                let item = res.snippets.get("Filter downcase").unwrap();
                assert_eq!(item.prefix, expected_struct.prefix);
                assert_eq!(item, &expected_struct);

                if let Ok(s) = serde_json::to_string(&res) {
                    const EXPECTED: &str = "{\"Filter downcase\":{\"prefix\":\"downcase\",\"body\":[\"| downcase }}\"]}}";
                    assert_eq!(s, EXPECTED);
                } else {
                    assert!(false);
                }
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                assert!(false);
            }
        }
    }
    #[test]
    fn serialization_missing_prefix() {
        let file = r#"{
    "Filter downcase": {
      "body": "| downcase }}"
    }}"#
        .to_string();

        let res = dynamically_read_json_snippets(file, INTERACTIVE);

        match res {
            Ok(_) => {
                assert!(false);
            }
            Err(e) => {
                assert_eq!(e, TektonError::Reason(MISSING_PREFIX.into()));
            }
        }
    }

    #[test]
    fn serialization_missing_prefix_throw_error() {
        let file = r#"{
        "Filter downcase": {
          "body": "| downcase }}"
        }
    }"#
        .to_string();

        let res = dynamically_read_json_snippets(file, INTERACTIVE);

        match res {
            Ok(_) => {
                assert!(false, "Failed to throw the error");
            }
            Err(e) => {
                assert_eq!(e, TektonError::Reason(MISSING_PREFIX.into()));
            }
        }
    }
}
