//! Structures to model the JSON snippet format

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendlySnippet {
    pub name: String,
    pub snip_body: FriendlySnippetBody,
}

impl FriendlySnippet {
    pub fn new(name: String, snip_body: FriendlySnippetBody) -> FriendlySnippet {
        FriendlySnippet { name, snip_body }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
fn test_json_snippet_creation() {
    let body = FriendlySnippetBody::new("snip".to_string(), Vec::new(), "Description".to_string());
    let snippet = FriendlySnippet::new("test".to_string(), body);
    assert_eq!(snippet.name, "test".to_string());
    assert_eq!(snippet.snip_body.prefix, "snip".to_string());
    assert_eq!(snippet.snip_body.body.len(), 0);
    assert_eq!(snippet.snip_body.description, "Description".to_string());
}
