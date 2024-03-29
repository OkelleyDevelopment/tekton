use crate::{
    errors::TektonError,
    models::multiprefix_friendly::{MultiBody, MultiPrefixTable},
};
use std::collections::HashMap;

use super::friendly_tekton::retrieve_body;

/// Essential the samething for the default
pub fn dynamic_prefix_combinator(file_content: &str) -> Result<MultiPrefixTable, TektonError> {
    let mut snippets: HashMap<String, MultiBody> = HashMap::new();
    let json: serde_json::Value = serde_json::from_str(file_content).unwrap();

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
                        "Failed on snippet ---> ".to_string() + &name + " | " + &e.to_string();
                    return Err(TektonError::Reason(message));
                }
            }
        }
    }

    Ok(MultiPrefixTable { snippets })
}

// Private function to handle parsing out the prefix.
fn retrieve_prefix(val: &serde_json::Value) -> Result<Vec<String>, TektonError> {
    if let Some(array) = val.as_array() {
        Ok(array
            .iter()
            .filter_map(|e| e.as_str().map(|val| val.to_string()))
            .collect())
    } else if let Some(prefix) = val.as_str() {
        Ok(vec![prefix.to_string()])
    } else {
        Err(TektonError::Reason(
            "Check source file, not possible to construct snippet.".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
