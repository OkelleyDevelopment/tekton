use crate::{errors::TektonError, snippets::snipmate::Snipmate};
use regex::{bytes::RegexSetBuilder, Regex};
use serde::de::IntoDeserializer;

/// A function to convert JSON snippets to Snipmate snippets
pub fn compose_snipmate_snippets(json_snippets: String) -> Result<String, TektonError> {
    let json: serde_json::Value = match serde_json::from_str(&json_snippets) {
        Ok(v) => v,
        Err(_) => {
            return Err(TektonError::Reason(
                "Error: No snippets were found".to_string(),
            ));
        }
    };
    let snippets = create_snipmate_structs_from_json(json)?;
    let snipmate_string = build_snipmate_string(snippets)?;
    Ok(snipmate_string)
}

/// A function to create a string representation of a Vec of Snipmate Snippets
pub fn build_snipmate_string(snippets: Vec<Snipmate>) -> Result<String, TektonError> {
    let mut snipmate_string = String::from("");
    for snip in snippets {
        snipmate_string = snipmate_string + &snip.display();
    }
    Ok(snipmate_string)
}

/// Function to generate a Vec of Snippet structs from parsed JSON, will return TektonError if Vec is empty
pub fn create_snipmate_structs_from_json(
    json: serde_json::Value,
) -> Result<Vec<Snipmate>, TektonError> {
    let re2 = Regex::new(r##"\\""##).unwrap();
    let mut snippets: Vec<Snipmate> = Vec::new();

    if let Some(obj) = json.into_deserializer().as_object() {
        // Note: This tuple is (name, value) but we omit the name for Snipmate snippets
        for (_, v) in obj {
            let snip: Snipmate = Snipmate::new(
                v["prefix"].to_string(),
                v["body"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|line| line.to_string())
                    .collect(),
                re2.replace_all(&v["description"].to_string(), '"'.to_string())
                    .to_string(),
            );

            snippets.push(snip);
        }
    }
    if snippets.is_empty() {
        Err(TektonError::Reason(
            "No JSON snippets were parsed".to_string(),
        ))
    } else {
        Ok(snippets)
    }
}

/// Function to construct the Snipmate structs from a Vec<String> representing the snippet file that was read in.
pub fn build_snippets_from_file(lines: Vec<String>) -> Vec<Snipmate> {
    let mut snippets: Vec<Snipmate> = Vec::new();
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
            let mut s = line.split_whitespace();
            s.next();
            let name = s.next().unwrap_or("").to_string();

            let mut desc = s.collect::<Vec<&str>>().join(" ");
            desc = re.replace_all(&desc, "").to_string();
            desc = desc.replace("\"", "");
            // Building the snippet and adding to the array
            snippets.push(Snipmate::new(name, Vec::new(), desc));
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
