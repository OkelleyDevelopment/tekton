//! Structures to model the JSON snippet format

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Struct representing the JSON file of snippets
#[derive(Debug, Serialize, Deserialize)]
pub struct FriendlySnippets {
    /// The hashmap (table) that represents the source snippet file
    #[serde(flatten)]
    pub snippets: HashMap<String, FriendlySnippetBody>,
}

/// A struct representing the body of a snippet from the `FriendlySnippets` hashmap
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct FriendlySnippetBody {
    /// The trigger for the snippet
    pub prefix: String,
    /// The 'snippet' contents
    pub body: Vec<String>,
    /// An optional description explaining the snippet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FriendlySnippetBody {
    /// A constructor function for the Snippet body
    pub fn new(
        prefix: String,
        body: Vec<String>,
        description: Option<String>,
    ) -> FriendlySnippetBody {
        FriendlySnippetBody {
            prefix,
            body,
            description,
        }
    }
}

#[test]
fn test_snippet_body_creation() {
    let body = FriendlySnippetBody::new(
        "snip".to_string(),
        Vec::new(),
        Some("Description".to_string()),
    );
    assert_eq!(body.prefix, "snip".to_string());
    assert_eq!(body.body.len(), 0);
}

#[test]
fn test_friendly_snippets() {
    let mut hp: FriendlySnippets = FriendlySnippets {
        snippets: HashMap::new(),
    };
    let body = FriendlySnippetBody::new(
        "snip".to_string(),
        Vec::new(),
        Some("Description".to_string()),
    );
    let expected_body = FriendlySnippetBody::new(
        "snip".to_string(),
        Vec::new(),
        Some("Description".to_string()),
    );
    hp.snippets.insert("test".to_string(), body);
    assert_eq!(
        hp.snippets.get(&"test".to_string()).unwrap(),
        &expected_body
    );
}

#[test]
fn friendly_description_is_none() {
    let mut hp: FriendlySnippets = FriendlySnippets {
        snippets: HashMap::new(),
    };
    let body = FriendlySnippetBody::new("snip".to_string(), Vec::new(), None);
    let expected_body = FriendlySnippetBody::new("snip".to_string(), Vec::new(), None);
    hp.snippets.insert("test".to_string(), body);
    assert_eq!(
        hp.snippets.get(&"test".to_string()).unwrap(),
        &expected_body
    );
}
