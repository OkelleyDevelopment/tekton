//! This is the central driving function of tekton
//!
//! It filters the snippet file type pairs and then calls the appropriate composition
//! function. These functions are split into their own files to keep this file
//! simple.
//!

use super::friendly_tekton::{read_in_json_snippets, sort_friendly_snippets};
use super::{
    friendly_tekton::compose_friendly_snippets, snipmate_tekton::compose_snipmate_snippets,
};
use crate::{errors::TektonError, utils::read_lines};
use std::fs::{self};

/// The main snippet composition function
///
/// Arguments:
/// - `fname` is the filename of the snippets to read from
/// - `types` is the tuple which signifies what mapping to use in the match statement
///
/// Returns:
/// - Result of String (to write to file) or a TektonError with the reason for the error
pub fn composer(fname: &String, types: (&str, &str)) -> Result<String, TektonError> {
    match types {
        ("snippet", "json") => match read_lines(fname) {
            Ok(lines) => compose_friendly_snippets(lines),
            Err(e) => Err(TektonError::Reason(e.to_string())),
        },
        ("json", "snippet") => match fs::read_to_string(fname) {
            Ok(lines) => compose_snipmate_snippets(read_in_json_snippets(&lines)?),
            Err(e) => Err(TektonError::Reason(e.to_string())),
        },
        ("json", "tekton-sort") => {
            sort_friendly_snippets(read_in_json_snippets(fname)?)
        }
        _ => Err(TektonError::Reason(
            "Unsupported mapping attempted in the composer function".to_string(),
        )),
    }
}

#[test]
fn test_composer() {
    let name = "testfile.snippet";

    let res = composer(&name.to_string(), ("snippet", "json"));

    match res {
        Err(error) => {
            assert_eq!(
                error,
                TektonError::Reason("No such file or directory (os error 2)".to_string())
            );
        }
        _ => {} // This case isn't possible as the file doesn't exist for this test
    }
}
