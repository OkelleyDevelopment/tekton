use regex::Regex;
use serde::de::IntoDeserializer;

use crate::{
    errors::TektonError,
    snippets::{
        friendly::{FriendlySnippet, FriendlySnippetBody},
        snippet::Snippet,
    },
    utils::get_input,
};

use super::snipmate_tekton::gen_snippet;

/// A helper function to handle Snipmate to JSON
pub fn compose_friendly_snippets(lines: Vec<String>) -> Result<String, TektonError> {
    let snips = gen_snippet(lines);
    let friendlies: Vec<FriendlySnippet> = gen_friendly_snippets(snips);
    // Perhaps this can be removed and return a Vec<FriendlySnippet> instead
    let result: Result<String, TektonError> = build_friendly_string(friendlies);
    match result {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Closing Program: {:?}", e);
            Err(e)
        }
    }
}

pub fn gen_friendly_snippets(snips: Vec<Snippet>) -> Vec<FriendlySnippet> {
    let mut friendly_handle: Vec<FriendlySnippet> = Vec::new();
    let re = Regex::new(r##"\\""##).unwrap();

    for snippet in snips {
        let prefix: String = snippet.prefix;
        //println!("The prefix is: {}", prefix);
        let mut body: Vec<String> = snippet.body;
        // --------------------------------------------------------------
        // NOTE: Remove the whitespace for the very first line of the snippet
        body.reverse();
        let first_line: String = body.pop().unwrap().to_string();
        body.reverse();
        body.insert(0, first_line.trim_start().to_string());
        // --------------------------------------------------------------
        let description: String = re.replace_all(&snippet.description, "").to_string();

        let name: String = "snippet ".to_owned() + &prefix;
        //println!("Name is now ---> {:?}", prefix);
        let friendly_body = FriendlySnippetBody::new(prefix, body, description);
        friendly_handle.push(FriendlySnippet {
            name,
            snip_body: friendly_body,
        });
    }
    friendly_handle
}

pub fn build_friendly_string(friendlies: Vec<FriendlySnippet>) -> Result<String, TektonError> {
    if friendlies.is_empty() {
        return Err(TektonError::Reason("No snippets provided".to_string()));
    }

    let mut finished: String = String::from("{\n");
    let mut count: usize = 0;
    let mut length: usize = friendlies.len();
    let target = friendlies.len();

    for obj in friendlies {
        count += 1;
        print!("\x1B[2J\x1B[1;1H"); // Clear terminal
        let snip = &serde_json::to_string_pretty(&obj.snip_body).unwrap();
        println!(
            "Snippet {} of {}:\n{}\n\nEnter name below:",
            count, target, snip
        );
        finished = finished + "\"" + &get_input() + "\": " + snip;
        if length > 1 {
            finished += ",\n"
        }
        length -= 1;
    }

    print!("\x1B[2J\x1B[1;1H");
    finished += "\n}";
    Ok(finished)
}

pub fn sort_friendly_snippets(json_snippets: String) -> Result<String, TektonError> {
    let json: serde_json::Value = serde_json::from_str(&json_snippets).unwrap();
    let mut snippets: Vec<FriendlySnippet> = Vec::new();
    for s in json.into_deserializer().as_object() {
        for (name, v) in s {
            let mut body: Vec<String> = Vec::new();

            for line in v["body"].as_array().unwrap().iter() {
                body.push(line.to_string());
            }

            let snippet_body = FriendlySnippetBody::new(
                v["prefix"].to_string(),
                body,
                v["description"].to_string(),
            );

            snippets.push(FriendlySnippet {
                name: name.to_string(),
                snip_body: snippet_body,
            });
        }
    }

    // Inplace sort of the snippets by their name
    snippets.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    let writable = serde_json::to_string(&snippets);

    match writable {
        Ok(s) => Ok(s),
        Err(e) => Err(TektonError::Reason(e.to_string())),
    }
}

#[test]
fn test_snippet_alphabatizing() {
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
