//! This is the central driving function of tekton
//!
//! It filters the snippet file type pairs and then calls the appropriate composition
//! function. These functions are split into their own files to keep this file
//! simple.
//!

use super::friendly_tekton::{build_friendly_string, sort_friendly_snippets};
use super::{
    friendly_tekton::compose_friendly_snippets, snipmate_tekton::compose_snipmate_snippets,
};
use crate::{errors::TektonError, utils::read_lines};
use core::panic;
use std::fs;

/// The main snippet composition function
pub fn composer(fname: &String, types: (&str, &str)) -> Result<String, TektonError> {
    match types {
        ("snippet", "json") => match read_lines(fname) {
            Ok(lines) => compose_friendly_snippets(lines),
            Err(e) => Err(TektonError::Reason(e.to_string())),
        },
        ("json", "snippet") => match fs::read_to_string(fname) {
            Ok(lines) => compose_snipmate_snippets(lines),
            Err(e) => Err(TektonError::Reason(e.to_string())),
        },
        ("json", "tekton-sort") => {
            let snippets = match fs::read_to_string(fname) {
                Ok(file) => sort_friendly_snippets(file),
                Err(e) => Err(TektonError::Reason(e.to_string())),
            };
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

#[test]
fn test_composer() {
    let name = "testfile.snippet";

    let res = composer(&name.to_string(), ("snippet", "json"));

    match res {
        Err(e) => {
            assert_eq!(
                e,
                TektonError::Reason("No such file or directory (os error 2)".to_string())
            );
        }
        _ => {}
    }
}