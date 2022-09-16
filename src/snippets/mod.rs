pub mod friendly;
pub mod snippet;

use self::{friendly::FriendlySnippet, snippet::Snippet};
use crate::snippets::friendly::FriendlySnippetBody;
use crate::utils::get_input;
use crate::errors::SnippetError;
use regex::bytes::RegexSetBuilder;

/// Escapes backslashes for the snippets to preserve strings
pub fn standardize_string(line: String) -> String {
    let mut res = str::replace(&line, "\"", "\\\"");
    res = str::replace(&res, "\\t", "    ");
    res
}

pub fn gen_snippet(lines: Vec<String>) -> Vec<Snippet> {
    let mut snippets: Vec<Snippet> = Vec::new();

    let set = RegexSetBuilder::new(&[r#"snippet ([a-zA-Z0-9]*)"#])
        .case_insensitive(true)
        .build()
        .expect("failed");

    for line in lines.iter() {
        if set.is_match(line.as_bytes()) {
            let mut s = line.split(' ');
            s.next();
            let name = s.next().unwrap().to_string();
            let desc = s.next().unwrap_or("").to_string();
            let snip = Snippet::new(name, Vec::new());
            snippets.push(snip);
        } else {
            let index = snippets.len() - 1;
            let handle = snippets.get_mut(index).unwrap();
            handle.body.push(standardize_string(line.to_string()));
        }
    }
    snippets
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

pub fn build_friendly_string(friendlies: Vec<FriendlySnippet>) -> Result<String, SnippetError> {
    if friendlies.is_empty() {
        return Err(SnippetError::Reason("No snippets provided".to_string()));
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
