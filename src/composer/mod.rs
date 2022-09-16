use core::panic;
use std::fs;

use serde::de::IntoDeserializer;

use crate::{
    snippets::{
        build_friendly_string, friendly::FriendlySnippet,
        gen_friendly_snippets, gen_snippet, snippet::Snippet,
    },
    utils::read_lines, errors::SnippetError,
};

/// The main snippet composition function 
pub fn compose_snippets(fname: &String, types: (&str, &str)) -> Result<String, SnippetError> {
    match types {
        ("snippet", "json") => {
            let lines: Result<Vec<String>, std::io::Error> = read_lines(fname);
            compose_friendly_snippets(lines.unwrap())
        },
        ("json", "snippet") => {
            let file = fs::read_to_string(fname).expect("Unable to read file");
            compose_vim_snippets(file)
        },
        _ => {
            panic!("Not a supported mapping!");
        }
    }
}

/// A private helper function to handle `.snippet` -> `.json`
fn compose_friendly_snippets(lines: Vec<String>) -> Result<String, SnippetError> {
    let snips = gen_snippet(lines);
    let friendlies: Vec<FriendlySnippet> = gen_friendly_snippets(snips);
    let result: Result<String, SnippetError> = build_friendly_string(friendlies);
    match result {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Closing Program: {:?}", e);
            Err(e)
        }
    }
}

/// A private helper function to strip JSON down to Snippet objects
fn compose_vim_snippets(json_snippets: String) -> Result<String, SnippetError> {
    
    // Read the JSON 
    let json: serde_json::Value = serde_json::from_str(&json_snippets).unwrap();
    

    // Declare a snippets vec
    let mut snippets: Vec<Snippet> = Vec::new();

    // Deserialize and form the object
    for obj in json.into_deserializer().as_object() {
        // For each object, get the Key (name of snippet), and value (snippet body)
        for (_,v) in obj {
            // Declare a Vec for the body of the snippet
            let mut body: Vec<String> = Vec::new();

            // Then for the body, iterate and push the lines to the new Vec
            for line in v["body"].as_array().unwrap().iter() {
                body.push(line.to_string());
            }
            // Extract and deref the prefix
            let prefix = v["prefix"].to_string();
            // Build out the snippet 
            let s: Snippet = Snippet::new(prefix, body);
            // Push to the end
            snippets.push(s);
        }
    }

    if snippets.is_empty() {
        return Err(SnippetError::Reason("No JSON snippets were parsed".to_string()));
    }

    let mut finished: String = String::from("");
    //let mut count: usize = 0;

    for obj in snippets {
        //count += 1;
        let snip = &obj.display();
        finished = finished + snip;
    }
    //println!("Total snippets converted: {}", count);
    Ok(finished)
}
