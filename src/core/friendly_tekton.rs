use super::snipmate_tekton::build_snippets_from_file;
use crate::{
    errors::TektonError,
    snippets::{
        friendly::{FriendlySnippet, FriendlySnippetBody},
        snipmate::Snipmate,
    },
    utils::get_input,
};
use regex::Regex;
use serde::de::IntoDeserializer;

// Constant for the suffix to the JSON snippet string
const NEWLINE: &str = ",\n  ";
pub const IS_INTERACTIVE: bool = true;
pub const IS_NOT_INTERACTIVE: bool = false;

/// A helper function to handle Snipmate to JSON
pub fn compose_friendly_snippets(lines: Vec<String>) -> Result<String, TektonError> {
    let snips = build_snippets_from_file(lines);
    let friendlies = create_friendly_snippet_structs(snips);
    let result = build_friendly_string(friendlies, IS_INTERACTIVE)?;
    Ok(result)
}

/// A function that takes a Vec of FriendlySnippet structs and returns the string representation or a TektonError
pub fn build_friendly_string(
    friendlies: Vec<FriendlySnippet>,
    interactive: bool,
) -> Result<String, TektonError> {
    if friendlies.is_empty() {
        return Err(TektonError::Reason("No snippets provided".to_string()));
    }

    let mut json_string: String = String::from("{\n  ");
    let mut count: usize = 1;
    let target = friendlies.len();

    for mut obj in friendlies {
        // Remove extra quotes
        obj.snip_body.description = obj.snip_body.description.replace('\"', "");
        // build our snippet String
        let body: String = serde_json::to_string_pretty(&obj.snip_body).unwrap();

        // Meaning the program clears the terminal
        // and will prompt for a name
        if interactive {
            print!("\x1B[2J\x1B[1;1H"); // Clear terminal
            println!(
                "Snippet {} of {}:\n{}\n\nEnter name below:",
                count, target, body
            );
            json_string = json_string + "\"" + &get_input() + "\": " + &body;
        } else {
            // Otherwise,
            // we already have the name so just append
            json_string = json_string + "\"" + &obj.name + "\": " + &body;
        }

        // More than one snippet and we append a newline
        if count < target {
            json_string += NEWLINE
        }
        // Increment the count of snippets
        count += 1;
    }

    json_string += "\n}";
    Ok(json_string)
}

/// A function to convert an array of Snipmate structs to an array of FriendlySnippet structs
pub fn create_friendly_snippet_structs(snips: Vec<Snipmate>) -> Vec<FriendlySnippet> {
    let mut friendly_handle: Vec<FriendlySnippet> = Vec::new();
    let re = Regex::new(r##"\\""##).unwrap();

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
        let name: String = "snippet ".to_owned() + &prefix;
        let description: String = re.replace_all(&snippet.description, "").to_string();
        let friendly_body = FriendlySnippetBody::new(prefix, body, description);
        friendly_handle.push(FriendlySnippet {
            name,
            snip_body: friendly_body,
        });
    }
    friendly_handle
}

/// Helper function to create the JSON structs
pub fn create_json_snippet_structs(file: String) -> Result<Vec<FriendlySnippet>, TektonError> {
    let json: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&file);
    match json {
        Ok(json) => {
            let mut snippets: Vec<FriendlySnippet> = Vec::new();

            if let Some(obj) = json.into_deserializer().as_object() {
                for (name, v) in obj {
                    let mut body: Vec<String> = Vec::new();
                    for line in v["body"].as_array().unwrap().iter() {
                        let edited_line = line.to_string().replace("\\t", "    ");
                        body.push(edited_line);
                    }

                    let snippet_body = FriendlySnippetBody::new(
                        v["prefix"].to_string().replace('\"', ""),
                        body,
                        v["description"].to_string().replace('\"', ""),
                    );

                    snippets.push(FriendlySnippet {
                        name: name.to_string(),
                        snip_body: snippet_body,
                    });
                }
            }

            if snippets.is_empty() {
                Err(TektonError::Reason("No snippets created".to_string()))
            } else {
                Ok(snippets)
            }
        }
        Err(e) => Err(TektonError::Reason(e.to_string())),
    }
}

/// Function to build JSON snippets, sort, then return the sorted Vec<FriendlySnippet>
pub fn sort_friendly_snippets(json_snippets: String) -> Result<String, TektonError> {
    let snippets = create_json_snippet_structs(json_snippets);
    match snippets {
        Ok(mut s) => {
            // Sort of the snippets by their name
            s.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            let mut sorted: String = String::from("{\n  ");

            let mut count = 0;
            let target = s.len();
            for snip in s {
                count += 1;
                let name = snip.name;
                let snippet_body_string = serde_json::to_string_pretty(&snip.snip_body).unwrap();
            
                sorted += &('\"'.to_string() + &name +  &"\":".to_string());
                sorted += &snippet_body_string;
                if count <  target {
                    sorted += ", \n"
                }
            }
            sorted += "\n}";
            Ok(sorted)
        }
        Err(e) => Err(TektonError::Reason(e.to_string())),
    }
}

#[test]
fn test_json_sort() {
    // This shows the testing of the sort directly
    let mut snippets: Vec<FriendlySnippet> = Vec::new();
    snippets.push(FriendlySnippet {
        name: String::from("zebra"),
        snip_body: FriendlySnippetBody {
            prefix: "wumbo".to_string(),
            body: Vec::new(),
            description: String::new(),
        },
    });
    snippets.push(FriendlySnippet {
        name: String::from("Plant"),
        snip_body: FriendlySnippetBody {
            prefix: "wumbo".to_string(),
            body: Vec::new(),
            description: String::new(),
        },
    });
    snippets.push(FriendlySnippet {
        name: String::from("pLants"),
        snip_body: FriendlySnippetBody {
            prefix: "wumbo".to_string(),
            body: Vec::new(),
            description: String::new(),
        },
    });
    snippets.push(FriendlySnippet {
        name: String::from("apples"),
        snip_body: FriendlySnippetBody {
            prefix: "wumbo".to_string(),
            body: Vec::new(),
            description: String::new(),
        },
    });
    snippets.push(FriendlySnippet {
        name: String::from("Apples"),
        snip_body: FriendlySnippetBody {
            prefix: "wumbo".to_string(),
            body: Vec::new(),
            description: String::new(),
        },
    });

    snippets.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    assert_eq!(snippets.get(0).unwrap().name, "apples".to_string());
    assert_eq!(snippets.get(1).unwrap().name, "Apples".to_string());
    assert_eq!(snippets.get(2).unwrap().name, "Plant".to_string());
    assert_eq!(snippets.get(3).unwrap().name, "pLants".to_string());
    assert_eq!(snippets.get(4).unwrap().name, "zebra".to_string());
}
