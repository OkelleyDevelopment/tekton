use crate::{
    snippets::{
        build_friendly_string, errors::SnippetError, friendly::FriendlySnippet,
        gen_friendly_snippets, gen_snippet,
    },
    utils::read_lines,
};

pub fn compose_snippets(fname: &String) -> Result<String, SnippetError> {
    let lines: Vec<String> = read_lines(fname);

    // TODO: This is where we would determine the kind of snippet
    let friendlies: Vec<FriendlySnippet> = gen_friendly_snippets(gen_snippet(lines));
    let result: Result<String, SnippetError> = build_friendly_string(friendlies);
    match result {
        Ok(result) => {
            Ok(result)
        }
        Err(e) => {
            eprintln!("Closing Program: {:?}", e);
            Err(e)
        }
    }
}
