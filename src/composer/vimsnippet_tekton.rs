use serde::de::IntoDeserializer;

use crate::{errors::TektonError, snippets::snippet::Snippet};
use regex::{bytes::RegexSetBuilder, Regex};

/// A private helper function to strip JSON down to Snippet objects
pub fn compose_vim_snippets(json_snippets: String) -> Result<String, TektonError> {
    // Read the JSON
    let json: serde_json::Value = serde_json::from_str(&json_snippets).unwrap();
    let re2 = Regex::new(r##"\\""##).unwrap();
    // Declare a snippets vec
    let mut snippets: Vec<Snippet> = Vec::new();

    // TODO: cargo clippy -- -D warnings throws an error, says this should be `if let`.
    // Deserialize and form the object
    for obj in json.into_deserializer().as_object() {
        // For each object, get the Key (name of snippet), and value (snippet body)
        for (_, v) in obj {
            // Declare a Vec for the body of the snippet
            let mut body: Vec<String> = Vec::new();

            // Then for the body, iterate and push the lines to the new Vec
            for line in v["body"].as_array().unwrap().iter() {
                //body.push(line.to_string());
                body.push(line.to_string());
            }
            // Extract and deref the prefix
            let prefix = v["prefix"].to_string();
            let mut description = v["description"].to_string();
            //println!("{}", description);
            description = re2.replace_all(&description, '"'.to_string()).to_string();
            // Build out the snippet
            let s: Snippet = Snippet::new(prefix, body, description);
            // Push to the end
            snippets.push(s);
        }
    }

    if snippets.is_empty() {
        return Err(TektonError::Reason(
            "No JSON snippets were parsed".to_string(),
        ));
    }

    let mut finished: String = String::from("");
    for obj in snippets {
        let snip = &obj.display();
        //println!("\n ---> {}", snip);
        finished = finished + snip;
    }

    Ok(finished)
}

/// Function to construct each snippet object from the `example.snippet` format
pub fn gen_snippet(lines: Vec<String>) -> Vec<Snippet> {
    let mut snippets: Vec<Snippet> = Vec::new();
    let tab = String::from("\\t");
    let tab_regex = Regex::new(&tab).unwrap();

    let set = RegexSetBuilder::new(&[r#"snippet ([a-zA-Z0-9]*)"#])
        .case_insensitive(true)
        .build()
        .expect("failed");
    let re = Regex::new(r##"\\""##).unwrap();
    for line in lines.iter() {
        // Construct a new snippet
        if set.is_match(line.as_bytes()) {
            let mut s = line.split_whitespace().into_iter();
            s.next();
            let name = s.next().unwrap_or("").to_string();

            let mut desc = s.map(|s| &*s).collect::<Vec<&str>>().join(" ");
            desc = re.replace_all(&desc, "").to_string();
            // Building the snippet and adding to the array
            snippets.push(Snippet::new(name, Vec::new(), desc));
        }
        // Continue to add the body of the snippet to the most recently
        // added snippet struct.
        else {
            let index = snippets.len() - 1;
            let handle = snippets.get_mut(index).unwrap();
            handle
                .body
                .push(tab_regex.replace_all(&line.to_string(), "  ").to_string());
        }
    }
    snippets
}