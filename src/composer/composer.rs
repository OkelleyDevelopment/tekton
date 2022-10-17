//! This is the driving exo-skeleton of the Tekton snippet tool
//!
//! It filters the snippet file type pairs and then calls the appropriate composition
//! function. These functions are split into their own files to keep this file
//! simple.
//!

use core::panic;
use std::fs;

use crate::{errors::TektonError, utils::read_lines};

use super::friendly_tekton::sort_friendly_snippets;
use super::{friendly_tekton::compose_friendly_snippets, snipmate_tekton::compose_snipmate_snippets};

/// The main snippet composition function
pub fn composer(fname: &String, types: (&str, &str)) -> Result<String, TektonError> {
    match types {
        ("snippet", "json") => {
            let lines: Result<Vec<String>, std::io::Error> = read_lines(fname);
            compose_friendly_snippets(lines.unwrap())
        }
        ("json", "snippet") => {
            let file = fs::read_to_string(fname).expect("Unable to read file");
            compose_snipmate_snippets(file)
        }
        ("json", "tekton-sort") => {
            let file = fs::read_to_string(fname).expect("Unable to read file");
            sort_friendly_snippets(file)
        }
        _ => {
            panic!("No supported mapping!");
        }
    }
}
