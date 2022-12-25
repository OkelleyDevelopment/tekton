//!
//! TODO: Ideally this file will be removed if we can make the hashmap
//! generic.
//!

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{core::friendly_tekton::retrieve_body, errors::TektonError};

/// The Struct representing the JSON file of snippets
#[derive(Debug, Serialize, Deserialize)]
pub struct FriendlySnippet {
    #[serde(flatten)]
    pub snippets: HashMap<String, MultiBody>,
}

/// A struct representing the body of a snippet from the `FriendlySnippets` hashmap
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MultiBody {
    pub prefix: Vec<String>,
    pub body: Vec<String>,
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

/// Essential the samething for the default
pub fn dynamic_prefix_combinator(file_content: &str) -> Result<FriendlySnippet, TektonError> {
    let mut snippets: HashMap<String, MultiBody> = HashMap::new();
    // println!("{}", file);
    let json: serde_json::Value = serde_json::from_str(&file_content).unwrap();

    if let Some(obj) = json.as_object() {
        for (k, v) in obj {
            let name = k.clone();
            let mut description: String = String::new();

            match retrieve_prefix(&v["prefix"]) {
                Ok(prefix) => {
                    let body = retrieve_body(&v["body"]);
                    if let Some(val) = v["description"].as_str() {
                        description.push_str(val);
                    }
                    let snip_body = MultiBody::new(prefix, body, description);

                    snippets.insert(name.to_string(), snip_body);
                }
                Err(e) => {
                    let message =
                        "Failed on snippet ---> ".to_owned() + &name + " | " + &e.to_string();
                    return Err(TektonError::Reason(message));
                }
            }
        }
    }

    Ok(FriendlySnippet { snippets })
}

fn retrieve_prefix(val: &serde_json::Value) -> Result<Vec<String>, TektonError> {
    if let Some(array) = val.as_array() {
        Ok(array
            .iter()
            .map(|e| e.as_str().unwrap().to_string())
            .collect())
    } else if let Some(prefix) = val.as_str() {
        Ok(vec![prefix.to_string()])
    } else {
        Err(TektonError::Reason(
            "Check source file, not possible to construct snippet.".to_string(),
        ))
    }
}

#[test]
fn test_multiple_prefix_entries() {
    let file = r#"{
        "Unreal GetLifeTimeReplicates": {
            "prefix": ["ugetlifetimereplicatedprops", "usetupreplicatedproperties"],
            "body": [
              "void ${1:ClassName}::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const",
              "{",
              "\tSuper::GetLifetimeReplicatedProps(OutLifetimeProps);",
              "\t//DON'T FORGET TO #include \"Net/UnrealNetwork.h\"",
              "\tDOREPLIFETIME(${1:ClassName}, ${2:ClassProperty});",
              "}"
            ],
            "description": "Creates the Function in which you setup replicated properties"
          }
    }"#
    .to_string();

    let res = dynamic_prefix_combinator(&file);

    match res {
        Ok(res) => {
            let expected_struct = MultiBody::new(
                vec!["ugetlifetimereplicatedprops".to_string(), "usetupreplicatedproperties".to_string()],
                vec![
                    "void ${1:ClassName}::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const",
                    "{",
                    "\tSuper::GetLifetimeReplicatedProps(OutLifetimeProps);",
                    "\t//DON'T FORGET TO #include \"Net/UnrealNetwork.h\"",
                    "\tDOREPLIFETIME(${1:ClassName}, ${2:ClassProperty});",
                    "}"
                  ].iter().map(|e| e.to_string()).collect(),
                  "Creates the Function in which you setup replicated properties".to_string(),
            );
            assert_eq!(res.snippets.len(), 1);
            let item = res.snippets.get("Unreal GetLifeTimeReplicates").unwrap();
            assert_eq!(item.prefix, expected_struct.prefix);
            assert_eq!(item, &expected_struct);
        }
        Err(e) => {
            println!("Error: {}", e.to_string());
            assert!(false);
        }
    }
}

#[test]
fn test_single_prefix_entries_in_array() {
    let file = r#"{
        "Unreal GetLifeTimeReplicates": {
            "prefix": "ugetlifetimereplicatedprops",
            "body": [
              "void ${1:ClassName}::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const",
              "{",
              "\tSuper::GetLifetimeReplicatedProps(OutLifetimeProps);",
              "\t//DON'T FORGET TO #include \"Net/UnrealNetwork.h\"",
              "\tDOREPLIFETIME(${1:ClassName}, ${2:ClassProperty});",
              "}"
            ],
            "description": "Creates the Function in which you setup replicated properties"
          }
    }"#
    .to_string();

    let res = dynamic_prefix_combinator(&file);

    match res {
        Ok(res) => {
            let expected_struct = MultiBody::new(
                vec!["ugetlifetimereplicatedprops".to_string()],
                vec![
                    "void ${1:ClassName}::GetLifetimeReplicatedProps(TArray<FLifetimeProperty>& OutLifetimeProps) const",
                    "{",
                    "\tSuper::GetLifetimeReplicatedProps(OutLifetimeProps);",
                    "\t//DON'T FORGET TO #include \"Net/UnrealNetwork.h\"",
                    "\tDOREPLIFETIME(${1:ClassName}, ${2:ClassProperty});",
                    "}"
                  ].iter().map(|e| e.to_string()).collect(),
                  "Creates the Function in which you setup replicated properties".to_string(),
            );
            assert_eq!(res.snippets.len(), 1);
            let item = res.snippets.get("Unreal GetLifeTimeReplicates").unwrap();
            assert_eq!(item.prefix, expected_struct.prefix);
            assert_eq!(item, &expected_struct);
        }
        Err(e) => {
            println!("Error: {}", e.to_string());
            assert!(false);
        }
    }
}
