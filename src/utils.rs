//! Simple and easy to use Utilities
//!
//! Various Utilities that may be used throughout the CLI program
//!

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
