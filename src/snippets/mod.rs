pub mod errors;
pub mod friendly;
pub mod snippet;

use self::{friendly::FriendlySnippet, snippet::Snippets};
use crate::snippets::{errors::SnippetError, friendly::FriendlySnippetBody};
use crate::utils::get_input;
use regex::bytes::RegexSetBuilder;

pub fn standardize_string(line: String) -> String {
    let mut res = str::replace(&line, "\"", "\\\"");
    res = str::replace(&res, "\\t", "    ");
    return String::from(res);
}

pub fn gen_snippet(lines: Vec<String>) -> Vec<Snippets> {
    let mut snippets: Vec<Snippets> = Vec::new();

    let set = RegexSetBuilder::new(&[r#"snippet ([a-zA-Z0-9]*)"#])
        .case_insensitive(true)
        .build()
        .expect("failed");

    for line in lines.iter() {
        if set.is_match(line.as_bytes()) {
            let mut s = line.split(" ");
            s.next();
            let name = s.next().unwrap().to_string();
            let snip = Snippets::new(name, Vec::new());
            snippets.push(snip);
        } else {
            let index = snippets.len() - 1;
            let handle = snippets.get_mut(index).unwrap();
            handle.body.push(standardize_string(line.to_string()));
        }
    }
    return snippets;
}

pub fn gen_friendly_snippets(snips: Vec<Snippets>) -> Vec<FriendlySnippet> {
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
        let name: String = String::from("snippet ".to_owned() + &prefix);
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
    if friendlies.len() < 1 {
        return Err(SnippetError::Reason("No snippets provided".to_string()));
    }

    let mut finished: String = String::from("{\n");
    let mut count: usize = 0;
    let mut length: usize = friendlies.len();
    let target = length.clone();

    for obj in friendlies {
        count = count + 1;
        print!("\x1B[2J\x1B[1;1H"); // Clear terminal
        let snip = &serde_json::to_string_pretty(&obj.snip_body).unwrap();
        println!(
            "Snippet {} of {}:\n{}\n\nEnter name below:",
            count, target, snip
        );
        finished = finished + "\"" + &get_input() + "\": " + snip;
        if length > 1 {
            finished = finished + ",\n"
        }
        length = length - 1;
    }

    print!("\x1B[2J\x1B[1;1H");
    finished = finished + "\n}";
    Ok(finished)
}
