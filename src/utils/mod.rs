//! Simple and easy to use utilities that may be used throughout the CLI program

use core::panic;
use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Error};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::errors::TektonError;

/// Function to retrive user input, looping until text is in the buffer
/// and is not an empty line
///
/// Returns:
/// - The user input as a String
pub fn get_input() -> String {
    let mut input = String::new();
    while input == String::new() {
        io::stdin()
            .read_line(&mut input)
            .expect("Error with reading input");
    }
    return input.trim().to_string();
}

/// A helper function to clear the screen and
/// provide better flexibility later on.
pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H"); // Clear terminal
}

/// Function to read the lines of a file and returns a Vec of Strings
/// Arguments:
/// - `fname` is the filename of the snippets to read from
///
/// Returns:
/// - Result of vector of String or Error
pub fn read_lines(fname: &String) -> Result<Vec<String>, Error> {
    let file = File::open(fname)?;
    let buf = BufReader::new(file);
    Ok(buf
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect())
}
/// Function to write to a newly created file.
///
/// Arguments:
/// - `output_name` : file name to write the snippets to
/// - `finished` : the final serialized string representation of the snippets
///
pub fn write_to_file(output_name: String, finished: String) {
    let mut outfile = File::create(Path::new("./").join(output_name))
        .unwrap_or_else(|err| panic!("Could not create the file {}", err));

    outfile
        .write_all(finished.as_bytes())
        .unwrap_or_else(|err| panic!("Could not write the snippets\n>>> Error >>>{}", err));
}

/// Function to create a vector of PathBuf's that will be consumed
/// by the program during runtime.
///
/// Arguments:
/// - `path`: the path to the directory or file to read
/// - `crawl`: an optional boolean to indicate if the `path` is a directory crawl through.
///
/// Returns:
/// - A list of Pathbuf's representing files the program may read
pub fn crawl_files(path: String, crawl: Option<String>) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    if crawl.is_some() {
        // This for loop is from the WalkDir documentation, I claim no credit
        // with respect to this.
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                files.push(file.path().to_path_buf());
            }
        }
    } else {
        files.push(PathBuf::from(path));
    }
    files
}

/// Helper function to get the file extension being passed in.
///
/// Arguments:
/// - `filename`: a string slice representing the name of the file
///
/// Returns:
/// - Optional string slice representing the file extension (e.g. `.json` or `.snippet`)
pub fn get_filetype(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

/// A generalized function to convert a JSON blob to a alphabetized JSON string.
///
/// Arguments:
/// - `table`: HashMap with the keys as Strings and the values (T) as a type that are De/Serialize-able
///
/// Returns:
/// - Result of either the finished JSON string or an Error
pub fn hash2ordered_string<T>(table: &HashMap<String, T>) -> Result<String, TektonError>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    match table.len() {
        0 => Err(TektonError::Reason(
            "Refusing to build string for 0 snippets".to_string(),
        )),
        _ => {
            // 1. Get the keys
            let mut keys: Vec<String> = table.iter().map(|(k, _)| k.to_string()).collect();

            // 2. Sort alphabetically (all lowercase)
            keys.sort_by_key(|a| a.to_lowercase());

            // 3. Iterate over the HashMap based on the ordered keys and collect into the BTreeMap
            let ordered: BTreeMap<String, _> = keys
                .iter()
                .map(|key| {
                    let snippet = table.get(key).unwrap();
                    (key.clone(), snippet)
                })
                .collect();

            // 4. Return the result as a JSON string
            match serde_json::to_string_pretty(&ordered) {
                Ok(string) => Ok(string),
                Err(e) => Err(TektonError::Reason(e.to_string())),
            }
        }
    }
}

#[test]
fn test_empty_string_on_extension() {
    let filename = String::from("");
    let result = get_filetype(&filename);
    assert_eq!(result, None);
}

#[test]
fn test_extension() {
    let filename = String::from("example.json");
    let result = get_filetype(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}

#[test]
fn test_long_filename_extension() {
    let filename = String::from("long_file_name_example.json");
    let result = get_filetype(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}

#[test]
fn test_extension_on_snippet() {
    let filename = String::from("example.snippet");
    let result = get_filetype(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "snippet");
}

#[test]
fn test_filename_with_parens_extension() {
    let filename = String::from("exam(file)ple.json");
    let result = get_filetype(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}

#[test]
fn test_filename_with_braces_extension() {
    let filename = String::from("exam{file}ple.json");
    let result = get_filetype(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}

#[test]
fn test_filename_with_brackets_extension() {
    let filename = String::from("exam[file]ple.json");
    let result = get_filetype(&filename);
    assert_ne!(result, None);
    assert_eq!(result.unwrap(), "json");
}
