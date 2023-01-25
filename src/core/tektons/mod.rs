pub mod friendly_tekton;
pub mod multiprefix_tekton;
pub mod snipmate_tekton;

#[cfg(test)]
mod tests {

    use crate::models::friendly::FriendlySnippets;

    use super::snipmate_tekton::create_snipmate_structs_from_json;
    #[test]
    fn snippet_bodies_match() {
        let input = r#"{
            "alpha": {
                "prefix": "print",
                "body": ["print!(", "\"${1}\");)"],
                "description": "print!(…);"
          }
        }"#;

        let friendlies: FriendlySnippets = serde_json::from_str(input).unwrap();
        let snippet = friendlies.snippets.get("alpha").unwrap().clone();

        let snipmate = create_snipmate_structs_from_json(friendlies);

        match snipmate {
            Ok(other) => {
                assert_eq!(other.len(), 1);
                let s = &other[0];
                assert_eq!(s.description, snippet.description);
            }
            Err(e) => panic!("{}", e),
        }
    }
}
