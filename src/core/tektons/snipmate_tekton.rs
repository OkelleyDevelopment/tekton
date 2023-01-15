use crate::{
    errors::TektonError,
    models::{
        friendly::{FriendlySnippets, Table},
        snipmate::Snipmate,
    },
};
use regex::{bytes::RegexSetBuilder, Regex};

/// A function to convert JSON snippets to Snipmate snippets
///
/// Arguments:
/// - `friendlies`: the structure holding the table of snippets.
///
/// Returns:
/// - Result of the composed snipmate string representation or an error
pub fn compose_snipmate_snippets(friendlies: FriendlySnippets) -> Result<String, TektonError> {
    let snippets = create_snipmate_structs_from_json(friendlies)?;
    let snipmate_string = build_snipmate_string(snippets)?;
    Ok(snipmate_string)
}

/// A function to create a string representation of a Vec of Snipmate Snippets
///
/// Arguments:
/// - `snippets`: a vector of Snipmate snippets to convert to a string
///
/// Returns:
/// - The built string or an error if there are zero (0) snippets
pub fn build_snipmate_string(snippets: Vec<Snipmate>) -> Result<String, TektonError> {
    match snippets.len() {
        0 => Err(TektonError::Reason("No snippets to convert".to_string())),
        _ => {
            let mut snipmate_string = String::from("");
            for snip in snippets {
                snipmate_string = snipmate_string + &snip.display();
            }
            Ok(snipmate_string)
        }
    }
}

/// Function to generate a Vec of Snippet structs from parsed JSON, will return TektonError if Vec is empty
///
/// This function does not ensure any sorting of snippets from the parsed HashMap.
///
/// Arguments:
/// - `friendlies`: the structure that holds the table of snippets
///
/// Returns:
/// - A resulting vector of Snipmate snippets or an error
pub fn create_snipmate_structs_from_json(
    friendlies: FriendlySnippets,
) -> Result<Vec<Snipmate>, TektonError> {
    let table: Table = friendlies.snippets;
    if table.is_empty() {
        return Err(TektonError::Reason("No snippets to convert".to_string()));
    }

    let target = table.len();
    let mut count: usize = 0;
    let mut snipmate_snippets: Vec<Snipmate> = Vec::new();
    for (_name, v) in table {
        if let Some(prefix) = v.prefix {
            let snip: Snipmate = Snipmate {
                prefix,
                body: v.body,
                description: v.description,
            };
            count += 1;
            snipmate_snippets.push(snip)
        }
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
///
/// Arguments:
/// - `lines`: a vector with the snipmate source file read in as a vec of strings
///
/// Returns:
/// - A vec of snipmate snippets (length can be 0), expects caller to check this condition
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
            let description: Option<String> = match desc.len() {
                0 => None,
                _ => Some(desc),
            };
            // Building the snippet and adding to the array
            snippets.push(Snipmate::new(name, Vec::new(), description));
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

#[test]
fn test_building_snipmate_on_empty_string() {
    let input: Vec<String> = vec![];

    let snippets = build_snippets_from_file(input);

    assert_eq!(snippets.len(), 0);
}

#[test]
fn test_building_snippets_from_file() {
    let input: Vec<String> = vec![
        "snippet test".to_string(),
        "   test snippet".to_string(),
        "snippet test2 an epic description".to_string(),
        "    a second snippet".to_string(),
        "    with several".to_string(),
        "    lines.".to_string(),
    ];

    let snippets = build_snippets_from_file(input);

    assert_eq!(snippets.len(), 2);
    let snip = snippets.get(0).unwrap();
    let snip2 = snippets.get(1).unwrap();
    let expected = vec![
        Snipmate::new(
            "test".to_string(),
            vec!["   test snippet".to_string()],
            None,
        ),
        Snipmate::new(
            "test2".to_string(),
            vec![
                "    a second snippet".to_string(),
                "    with several".to_string(),
                "    lines.".to_string(),
            ],
            Some("an epic description".to_string()),
        ),
    ];
    assert_eq!(snip.prefix, expected.get(0).unwrap().prefix);
    assert_eq!(snip.body, expected.get(0).unwrap().body);
    assert_eq!(snip.description, expected.get(0).unwrap().description);

    assert_eq!(snip2.prefix, expected.get(1).unwrap().prefix);
    assert_eq!(snip2.body, expected.get(1).unwrap().body);
    assert_eq!(snip2.description, expected.get(1).unwrap().description);
}

#[test]
fn test_output_string() {
    let input: Vec<String> = vec!["snippet test".to_string(), "   test snippet".to_string()];

    let snippets = build_snippets_from_file(input);

    if let Ok(res) = build_snipmate_string(snippets) {
        let spaces = "   "; // Note four spaces
        let expected: &str = &("snippet test\n\t".to_string() + spaces + "test snippet\n");
        assert_eq!(res, expected);
    } else {
        assert!(false);
    }
}
