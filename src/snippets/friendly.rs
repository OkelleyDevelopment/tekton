//! Structures to model the JSON snippet format

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendlySnippets {
    /// Hashmap with key: Snippet name, value: FriendlySnippetBody
    #[serde(flatten)]
    pub snippets: HashMap<String, FriendlySnippetBody>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FriendlySnippetBody {
    pub prefix: String,
    pub body: Vec<String>,
    pub description: String,
}

impl FriendlySnippetBody {
    pub fn new(prefix: String, body: Vec<String>, description: String) -> FriendlySnippetBody {
        FriendlySnippetBody {
            prefix,
            body,
            description,
        }
    }
}

#[test]
fn test_snippet_body_creation() {
    let body = FriendlySnippetBody::new("snip".to_string(), Vec::new(), "Description".to_string());
    assert_eq!(body.prefix, "snip".to_string());
    assert_eq!(body.body.len(), 0);
    assert_eq!(body.description, "Description".to_string());
}

#[test]
fn test_friendly_snippets() {
    let mut hp: FriendlySnippets = FriendlySnippets {
        snippets: HashMap::new(),
    };
    let body = FriendlySnippetBody::new("snip".to_string(), Vec::new(), "Description".to_string());
    let expected_body =
        FriendlySnippetBody::new("snip".to_string(), Vec::new(), "Description".to_string());
    hp.snippets.insert("test".to_string(), body);
    assert_eq!(
        hp.snippets.get(&"test".to_string()).unwrap(),
        &expected_body
    );
}
