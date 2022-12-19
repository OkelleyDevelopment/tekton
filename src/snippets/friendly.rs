//! Structures to model the JSON snippet format

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Struct representing the JSON file of snippets
#[derive(Debug, Serialize, Deserialize)]
pub struct FriendlySnippets {
    #[serde(flatten)]
    pub snippets: HashMap<String, FriendlySnippetBody>,
}

/// A struct representing the body of a snippet from the `FriendlySnippets` hashmap
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FriendlySnippetBody {
    pub prefix: String,
    pub body: Vec<String>,
    pub description: Option<String>,
}

impl FriendlySnippetBody {
    /// A constructor function for the Snippet body
    pub fn new(prefix: String, body: Vec<String>, description: String) -> FriendlySnippetBody {
        FriendlySnippetBody {
            prefix,
            body,
            description: Some(description),
        }
    }
}

#[test]
fn test_snippet_body_creation() {
    let body = FriendlySnippetBody::new("snip".to_string(), Vec::new(), "Description".to_string());
    assert_eq!(body.prefix, "snip".to_string());
    assert_eq!(body.body.len(), 0);
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
