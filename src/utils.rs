//! Simple and easy to use utilities that may be used throughout the CLI program

use core::panic;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error};
use std::path::Path;

/// Function to retrive user input, looping until text is in the buffer
/// and is not an empty line
pub fn get_input() -> String {
    let mut input = String::new();
    while input == String::new() {
        io::stdin()
            .read_line(&mut input)
            .expect("Error with reading input");
    }
    return input.trim().to_string();
}

/// Function to read the lines of a file and returns a Vec of Strings
pub fn read_lines(fname: &String) -> Result<Vec<String>, Error> {
    let file = File::open(fname)?;
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect())
}
/// Function to write to a newly created file.
pub fn write_to_file(name: String, finished: String) {
    let mut outfile = File::create(Path::new("./").join(name))
        .unwrap_or_else(|err| panic!("Could not create the file {}", err));

    outfile
        .write_all(finished.as_bytes())
        .unwrap_or_else(|err| panic!("Could not write the snippets {}", err));
}

/// Helper function to get the file extension being passed in.
pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

#[test]
fn test_extension() {
    let filename = String::from("example.json");
    let result = get_extension_from_filename(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}

#[test]
fn test_empty_string_on_extension() {
    let filename = String::from("");
    let result = get_extension_from_filename(&filename);
    assert_eq!(result, None);
}

#[test]
fn test_long_filename_extension() {
    let filename = String::from("long_file_name_example.json");
    let result = get_extension_from_filename(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}


#[test]
fn test_extension_on_snippet() {
    let filename = String::from("example.snippet");
    let result = get_extension_from_filename(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "snippet");
}

#[test]
fn test_filename_with_parens_extension() {
    let filename = String::from("exam(file)ple.json");
    let result = get_extension_from_filename(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}


#[test]
fn test_filename_with_braces_extension() {
    let filename = String::from("exam{file}ple.json");
    let result = get_extension_from_filename(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}


#[test]
fn test_filename_with_brackets_extension() {
    let filename = String::from("exam[file]ple.json");
    let result = get_extension_from_filename(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}