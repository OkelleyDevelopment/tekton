//! Functions related to creating and manipulating FriendlySnippets (JSON)

use super::snipmate_tekton::build_snippets_from_file;
use crate::{
    errors::TektonError,
    models::{
        friendly::{FriendlySnippetBody, FriendlySnippets},
        multiprefix_friendly::MultiPrefixTable,
        snipmate::Snipmate,
    },
    utils::{clear_terminal, get_input},
};
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

/// A helper function to handle Snipmate to JSON
pub fn compose_friendly_snippets(lines: Vec<String>) -> Result<String, TektonError> {
    let snips = build_snippets_from_file(lines);
    let friendlies = convert_snipmate_to_friendlysnippets(snips);
    let result = build_friendly_string(friendlies)?;
    Ok(result)
}

/// A function that takes the FriendlySnippet table and returns an ordered string representation or a TektonError
pub fn build_friendly_string(friendlies: FriendlySnippets) -> Result<String, TektonError> {
    build_sorted_string(&friendlies.snippets)
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

        let mut description: String = String::new();
        if let Some(descrip) = &snippet.description {
            description = re.replace_all(descrip, "").to_string();
        }

        let friendly_body = FriendlySnippetBody::new(prefix, body, Some(description));

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
pub fn read_in_json_snippets(file_name: &str) -> Result<FriendlySnippets, TektonError> {
    let file_contents = fs::read_to_string(&file_name);

    match file_contents {
        Ok(file_contents) => {
            let snippets: Result<FriendlySnippets, serde_json::Error> =
                serde_json::from_str(&file_contents);

            match snippets {
                Ok(snippets) => Ok(snippets),
                Err(_) => match dynamically_read_json_snippets(file_contents) {
                    Ok(snippets) => Ok(snippets),
                    Err(e) => Err(e),
                },
            }
        }
        Err(e) => Err(TektonError::Reason(e.to_string())),
    }
}

/// Helper function to read in the JSON for the `FriendlySnippets`, given uncertain JSON formatting
///
/// The [read_in_json_snippets] function should be preferred, however the ordering of fields in JSON isn't promised
/// and thus, this function builds the HashMap (backing the `FriendlySnippets` structure) by dynamically searching the
/// the table for the necessary fields and handling the missing ones more appropriately.
pub fn dynamically_read_json_snippets(file: String) -> Result<FriendlySnippets, TektonError> {
    // The snippet table (what is being created/ read in)
    let mut snippets: HashMap<String, FriendlySnippetBody> = HashMap::new();
    // The blob of JSON from serde_json
    let json: serde_json::Value = serde_json::from_str(&file).unwrap();
    // The 'need to fix this' pile
    let mut snippets_to_fix: Vec<(String, FriendlySnippetBody)> = Vec::new();

    if let Some(obj) = json.as_object() {
        for (k, v) in obj {
            // Track the name for the issue of revision (helps provide context)
            let name = k.clone();

            // This will be set later in the function
            let prefix: String = String::new();

            // Collect the lines of the snippet body (outsourced to a helper function)
            let body = retrieve_body(&v["body"]);

            // Create snippet body assuming no description
            let mut snip_body = FriendlySnippetBody::new(prefix, body, None);

            // If we find one, then update the structure
            if let Some(description) = v["description"].as_str() {
                if !description.is_empty() {
                    snip_body.description = Some(description.to_string());
                }
            }

            // Attempt to fetch the prefix
            let pref_candidate = retrieve_prefix(&v["prefix"]);

            // If it's found, great!
            // Otherwise, we send it to the 'need to fix this' pile.
            match pref_candidate {
                Some(pref) => {
                    snip_body.prefix = pref;
                }
                None => {
                    snippets_to_fix.push((name, snip_body));
                    continue; // skip inserting a malformed snippet into the table
                }
            }
            snippets.insert(name.to_string(), snip_body);
        }

        // Note for future revision:
        // We could return a tuple of (snippets, snippets_to_fix)
        // therefore making the next chunk of code it's own function
        // and avoiding user I/O inside the "read in function"
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
                    let snip_body = handle_missing_prefix(&name, snip_body);
                    snippets.insert(name.to_string(), snip_body);
                }
            }
        }
    }

    Ok(FriendlySnippets { snippets })
}

fn handle_missing_prefix(name: &str, mut snip_body: FriendlySnippetBody) -> FriendlySnippetBody {
    println!(
        "---- Snippet: {} ---\n{:#?}\n--------",
        name,
        serde_json::to_string_pretty(&snip_body)
    );
    println!("Enter a prefix:");
    loop {
        let input = get_input();
        println!("Proceed? (y/n):");
        let resp = get_input().to_lowercase();
        if resp == "y" {
            snip_body.prefix = input;
            break;
        }

        println!("Enter a new prefix: ")
    }
    print!("\x1B[2J\x1B[1;1H"); // Clear terminal
    snip_body
}

/// Function to handle the parsing of the prefix for a JSON snippet
fn retrieve_prefix(val: &serde_json::Value) -> Option<String> {
    val.as_str().map(|prefix| prefix.to_string())
}

/// Function to handle processing the body of a JSON snippet
///
/// This function
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
pub fn sort_friendly_snippets(snippets: FriendlySnippets) -> Result<String, TektonError> {
    let table = &snippets.snippets;
    match table.len() {
        0 => Err(TektonError::Reason(
            "Refusing to build string for 0 snippets".to_string(),
        )),
        _ => build_sorted_string(table),
    }
}

/// Function that builds a string representing the snippets in sorted order
pub fn order_friendly_snippets(snippets: MultiPrefixTable) -> Result<String, TektonError> {
    //println!("{:?}", &snippets.snippets);
    let table = &snippets.snippets;
    match table.len() {
        0 => Err(TektonError::Reason(
            "Refusing to build string for 0 snippets".to_string(),
        )),
        _ => {
            match table.len() {
                0 => Err(TektonError::Reason(
                    "Refusing to build string for 0 snippets".to_string(),
                )),
                _ => {
                    // 2. This provides an ordering
                    let ordered: BTreeMap<_, _> = table.iter().collect();
                    // 3. Return the result
                    match serde_json::to_string_pretty(&ordered) {
                        Ok(snippets) => Ok(snippets),
                        Err(e) => Err(TektonError::Reason(e.to_string())),
                    }
                }
            }
        }
    }
}

/// Helper function that consumes a FriendlySnippets struct and returns the
/// ordered JSON string.
///
fn build_sorted_string(
    table: &std::collections::HashMap<String, FriendlySnippetBody>,
) -> Result<String, TektonError> {
    match table.len() {
        0 => Err(TektonError::Reason(
            "Refusing to build string for 0 snippets".to_string(),
        )),
        _ => {
            // 2. This provides an ordering
            let ordered: BTreeMap<_, _> = table.iter().collect();
            // 3. Return the result
            match serde_json::to_string_pretty(&ordered) {
                Ok(snippets) => Ok(snippets),
                Err(e) => Err(TektonError::Reason(e.to_string())),
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

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
                    "println".to_string(),
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

        let res = dynamically_read_json_snippets(file);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    "println".to_string(),
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

        let res = dynamically_read_json_snippets(file);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    "downcase".to_string(),
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

        let res = dynamically_read_json_snippets(file);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    "downcase".to_string(),
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

        let res = dynamically_read_json_snippets(file);

        match res {
            Ok(res) => {
                let expected_struct = FriendlySnippetBody::new(
                    "downcase".to_string(),
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
}
