//! This is the central driving function of tekton
//!
//! It filters the snippet file type pairs and then calls the appropriate composition
//! function. These functions are split into their own files to keep this file
//! simple.
//!

use core::panic;
use std::fs;

use crate::{errors::TektonError, utils::read_lines};

use super::friendly_tekton::{build_friendly_string, sort_friendly_snippets};
use super::{
    friendly_tekton::compose_friendly_snippets, snipmate_tekton::compose_snipmate_snippets,
};

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
            let snippets = sort_friendly_snippets(file);
            match snippets {
                Ok(s) => build_friendly_string(s),
                Err(e) => Err(e),
            }
        }
        _ => {
            panic!("No supported mapping!");
        }
    }
}
