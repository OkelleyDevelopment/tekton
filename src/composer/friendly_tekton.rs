use crate::{
    errors::TektonError,
    snippets::{
        friendly::{FriendlySnippet, FriendlySnippetBody},
        snippet::Snippet,
    },
    utils::get_input,
};

use super::vimsnippet_tekton::gen_snippet;

/// A private helper function to handle `.snippet` -> `.json`
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
        let description: String = String::new();
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
