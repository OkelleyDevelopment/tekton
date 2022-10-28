use crate::{
    errors::TektonError,
    snippets::{friendly::FriendlySnippets, snipmate::Snipmate},
};
use regex::{bytes::RegexSetBuilder, Regex};

/// A function to convert JSON snippets to Snipmate snippets
pub fn compose_snipmate_snippets(friendlies: FriendlySnippets) -> Result<String, TektonError> {
    let snippets = create_snipmate_structs_from_json(friendlies)?;
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
///
/// This function does not ensure any sorting of snippets from the parsed HashMap.
pub fn create_snipmate_structs_from_json(
    friendlies: FriendlySnippets,
) -> Result<Vec<Snipmate>, TektonError> {
    let table = friendlies.snippets;
    if table.is_empty() {
        return Err(TektonError::Reason("No snippets to convert".to_string()));
    }

    let target = table.len();
    let mut count: usize = 0;
    let mut snipmate_snippets: Vec<Snipmate> = Vec::new();
    for (_k, v) in table {
        let snip: Snipmate = Snipmate {
            prefix: v.prefix,
            body: v.body,
            description: v.description,
        };
        count += 1;
        snipmate_snippets.push(snip)
    }

    println!("Count: {} || Target: {}", count, target);
    match count == target {
        true => Ok(snipmate_snippets),
        false => Err(TektonError::Reason(
            "Missing snippets in count, conversion aborted.".to_string(),
        )),
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
            desc = desc.replace('\"', "");
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
