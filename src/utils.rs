use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

pub fn get_input() -> String {
    let mut input = String::new();
    while input == String::new() {
        io::stdin()
            .read_line(&mut input)
            .expect("Error with reading input");
    }
    return input.trim().to_string();
}

pub fn read_lines(fname: &String) -> Vec<String> {
    let file = File::open(fname.to_owned()).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}

pub fn write_to_file(name: String, finished: String) {
    println!("Writing Snippets to file: {}", name);
    let mut outfile = File::create(Path::new("./").join(name))
        .unwrap_or_else(|err| panic!("Could not create the file {}", err));

    outfile
        .write_all(finished.as_bytes())
        .unwrap_or_else(|err| panic!("Could not write the snippets {}", err));
}
