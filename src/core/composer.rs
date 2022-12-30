//! This is the central driving function of tekton
//!
//! It filters the snippet file type pairs and then calls the appropriate composition
//! function. These functions are split into their own files to keep this file
//! simple.
//!

use super::friendly_tekton::{read_in_json_snippets, sort_friendly_snippets};
use super::multiprefix_tekton::{dynamic_prefix_combinator, order_friendly_snippets};
use super::{
    friendly_tekton::compose_friendly_snippets, snipmate_tekton::compose_snipmate_snippets,
};
use crate::errors::TektonError;
use crate::utils::read_lines;
use std::fs;

/// The main snippet composition function
///
/// Arguments:
/// - `fname` is the filename of the snippets to read from
/// - `types` is the tuple which signifies what mapping to use in the match statement
///
/// Returns:
/// - Result of String (to write to file) or a TektonError with the reason for the error
pub fn composer(
    fname: &String,
    types: (&str, &str),
    interactive: bool,
) -> Result<String, TektonError> {
    match types {
        ("snippet", "json") => match read_lines(fname) {
            Ok(lines) => compose_friendly_snippets(lines),
            Err(e) => Err(TektonError::Reason(e.to_string())),
        },
        ("json", "snippet") => match fs::read_to_string(fname) {
            Ok(lines) => compose_snipmate_snippets(read_in_json_snippets(&lines, interactive)?),
            Err(e) => Err(TektonError::Reason(e.to_string())),
        },
        ("json", "tekton-sort") => {
            sort_friendly_snippets(read_in_json_snippets(fname, interactive)?)
        }
        _ => Err(TektonError::Reason(
            "Unsupported mapping attempted in the composer function".to_string(),
        )),
    }
}

/// The 'fall-back' mode for the SORT portion of the program.
///
/// An 'all-else-fails try this' method to build the snippets from the file provided. This is done by
/// manually searching for snippet components and building them out via a helper method. The final return
/// value is the same criteria as the `composer()`.
///
/// Arguments:
/// - `fname` is the filename of the snippets to read from
///
/// Returns:
/// - Result of String (to write to file) or a TektonError with the reason for the error
pub fn multiprefix_composer(fname: &str) -> Result<String, TektonError> {
    match fs::read_to_string(&fname) {
        Ok(file_content) => {
            let snippets = dynamic_prefix_combinator(&file_content)?;
            order_friendly_snippets(snippets)
        }
        Err(e) => Err(TektonError::Reason(e.to_string())),
    }
}
