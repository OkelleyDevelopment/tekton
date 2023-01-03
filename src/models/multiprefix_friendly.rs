//!
//! The core representation for the snippets with multiple prefixes.
//!

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Struct representing the JSON file of snippets
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiPrefixTable {
    #[serde(flatten)]
    pub snippets: HashMap<String, MultiBody>,
}

/// A struct representing the body of a snippet from the `MultiPrefixTable` hashmap
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MultiBody {
    /// The list of triggers for the snippet
    pub prefix: Vec<String>,
    /// The 'snippet' contents
    pub body: Vec<String>,
    /// An optional description explaining the snippet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl MultiBody {
    /// A constructor function for the Snippet body
    pub fn new(prefix: Vec<String>, body: Vec<String>, description: String) -> Self {
        Self {
            prefix,
            body,
            description: Some(description),
        }
    }
}
