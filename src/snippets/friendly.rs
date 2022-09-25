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
